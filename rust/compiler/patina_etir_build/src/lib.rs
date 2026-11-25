use std::fmt::Debug;

use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::erl::ErlReprKind;
use rustc_middle::erl::etir::{self, ExprId, JoinId, LoopId, PatId, PatternInput, VarId};
use rustc_middle::middle::region;
use rustc_middle::thir::{LocalVarId, Thir};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_middle::util::Providers;
use rustc_middle::{bug, span_bug};
use rustc_span::{Ident, Span, Symbol};

macro_rules! with_expr_block {
    ($cx:expr, |$block:ident| $body:block) => {{
        let mut $block = $crate::ExprBlock::default();
        let result = (|| $body)();
        $block.emit(&mut $cx.body, result)
    }};
}

mod assigned_outer_vars;
mod expr;
mod graft;
mod inline;
mod pat;
mod witness;

/// Some expressions that we translate result in a whole block being emitted.
/// This helper structure collects additional exprs that should be emitted before
/// the final results.
#[derive(Default)]
struct ExprBlock(Vec<ExprId>);

impl ExprBlock {
    /// Wraps the final expression `value` in a block with all the expressions prepared
    /// through other functions of this struct.
    fn emit<'tcx>(mut self, body: &mut etir::Body<'tcx>, value: ExprId) -> ExprId {
        if self.0.is_empty() {
            value
        } else {
            let etir::Expr { ty, span, kind: _ } = body[value];
            self.0.push(value);
            body.exprs.push(etir::Expr {
                kind: etir::ExprKind::Block(self.0.into_boxed_slice()),
                ty,
                span,
            })
        }
    }

    /// Emits an assignment of `rhs` to the pattern `lhs`.
    fn assign<'tcx>(&mut self, cx: &mut BodyBuildCx<'tcx>, lhs: PatId, rhs: ExprId) {
        let etir::Expr { ty, span, kind: _ } = cx.body[rhs];
        let assign_expr = cx.push_expr(etir::ExprKind::Assign { lhs, rhs }, ty, span);
        self.0.push(assign_expr);
    }

    /// Emits an assignment of `value` to a `var`.
    fn bind<'tcx>(&mut self, cx: &mut BodyBuildCx<'tcx>, var: VarId, value: ExprId) {
        let etir::Expr { ty, span, kind: _ } = cx.body[value];

        let binding = cx.push_pat(
            etir::PatKind::Binding {
                var,
                is_primary: true,
                subpattern: None,
                input: PatternInput::Typed,
            },
            ty,
            span,
        );

        self.assign(cx, binding, value);
    }
}

struct BodyBuildCx<'tcx> {
    tcx: TyCtxt<'tcx>,
    owner: DefId,
    body: etir::Body<'tcx>,
}

impl<'tcx> BodyBuildCx<'tcx> {
    fn new(tcx: TyCtxt<'tcx>, owner: DefId) -> Self {
        BodyBuildCx { tcx, owner, body: etir::Body::new() }
    }

    fn push_expr(&mut self, kind: etir::ExprKind<'tcx>, ty: Ty<'tcx>, span: Span) -> ExprId {
        self.body.exprs.push(etir::Expr { kind, ty, span })
    }

    fn push_pat(&mut self, kind: etir::PatKind<'tcx>, ty: Ty<'tcx>, span: Span) -> PatId {
        self.body.pats.push(etir::Pat { kind, ty, span })
    }

    fn push_var(&mut self, ident: Ident) -> VarId {
        self.body.vars.push(etir::Var { ident })
    }
}

pub(crate) fn etir_body<'tcx>(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> &'tcx etir::Body<'tcx> {
    let owner = def_id.to_def_id();

    let cx = match tcx.def_kind(def_id) {
        DefKind::Impl { .. } => {
            let span = tcx.hir_expect_item(def_id).expect_impl().self_ty.span;
            BodyBuildCx::build_witness_constructor(tcx, owner, span)
        }
        DefKind::AssocTy => {
            let span = tcx.def_span(def_id);
            let trait_item = tcx
                .trait_item_of(def_id)
                .unwrap_or_else(|| span_bug!(span, "associated type witness outside a trait impl"));
            BodyBuildCx::build_assoc_type(tcx, owner, trait_item, span)
        }
        _ => {
            let body = tcx.etir_template(def_id).clone();
            let mut cx = BodyBuildCx { tcx, owner, body };
            cx.inline();
            cx.lower_witnesses();
            cx.analyze_assigned_outer_vars();
            cx
        }
    };

    tcx.arena.alloc(cx.body)
}

pub(crate) fn etir_template<'tcx>(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> &'tcx etir::Body<'tcx> {
    if tcx.is_typeck_child(def_id.to_def_id()) {
        bug!("{def_id:?} child has no ETIR body");
    }

    let mut cx = BodyBuildCx::new(tcx, def_id.to_def_id());
    let mut var_ids = FxHashMap::default();
    let (params, value) = ThirBodyCx::lower(&mut cx, &mut var_ids, def_id);

    cx.body.params = params;
    cx.body.value = value;

    tcx.arena.alloc(cx.body)
}

struct ThirBodyCx<'a, 'tcx> {
    cx: &'a mut BodyBuildCx<'tcx>,
    owner: LocalDefId,
    thir: &'a Thir<'tcx>,
    return_join: Option<JoinId>,
    break_joins: FxHashMap<region::Scope, JoinId>,
    continue_joins: FxHashMap<region::Scope, JoinId>,
    var_ids: &'a mut FxHashMap<LocalVarId, VarId>,
    reassign_count: usize,
}

impl<'a, 'tcx> ThirBodyCx<'a, 'tcx> {
    fn lower(
        cx: &mut BodyBuildCx<'tcx>,
        var_ids: &mut FxHashMap<LocalVarId, VarId>,
        owner: LocalDefId,
    ) -> (Box<[PatId]>, ExprId) {
        let (thir, root) = cx.tcx.thir_body(owner).unwrap_or_else(|guar| guar.raise_fatal());
        let thir = thir.borrow();

        let mut cx = ThirBodyCx {
            cx,
            owner,
            thir: &thir,
            return_join: None,
            break_joins: FxHashMap::default(),
            continue_joins: FxHashMap::default(),
            var_ids,
            reassign_count: 0,
        };

        let params = thir
            .params
            .iter()
            .filter(|param| param.hir_id.is_some())
            .map(|param| {
                let pat = param
                    .pat
                    .as_deref()
                    .unwrap_or_else(|| bug!("explicit parameter without a THIR pattern"));

                cx.lower_pat(pat, PatternInput::Typed)
            })
            .collect();

        let value = cx.lower_expr(root);
        let value = cx.wrap_in_joins(cx.return_join, value);

        (params, value)
    }

    fn push_loop(&mut self) -> LoopId {
        // `reassigned` vars are analyzed in a separate pass
        self.cx.body.loops.push(etir::Loop { reassigned: Box::new([]) })
    }

    fn push_join(&mut self, body: ExprId, value: Option<VarId>) -> JoinId {
        // `params` are analyzed in a separate pass
        self.cx.body.joins.push(etir::Join { body, value, params: Box::new([]) })
    }

    fn wrap_in_joins<I>(&mut self, joins: I, body: ExprId) -> ExprId
    where
        I: IntoIterator<Item = JoinId>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        joins.into_iter().rev().fold(body, |body, join| {
            let etir::Expr { ty, span, kind: _ } = self.cx.body[body];
            self.cx.push_expr(etir::ExprKind::Join { join, body }, ty, span)
        })
    }

    fn var(&mut self, id: LocalVarId) -> VarId {
        if let Some(&var) = self.var_ids.get(&id) {
            return var;
        }
        let var = self.cx.push_var(self.cx.tcx.hir_ident(id.0));
        self.var_ids.insert(id, var);
        var
    }

    fn lower_adt_repr(
        &self,
        span: Span,
        adt_def: ty::AdtDef<'tcx>,
        variant: &ty::VariantDef,
    ) -> etir::AdtRepr {
        let tcx = self.cx.tcx;
        let repr = tcx.erl_variant_repr(variant.def_id);
        match repr.kind {
            ErlReprKind::Atom => etir::AdtRepr::Atom { tag: repr.tag },
            ErlReprKind::Int => not_implemented(tcx, span, "int-represented ADT"),
            ErlReprKind::Tagged => etir::AdtRepr::Tagged { tag: repr.tag },
            ErlReprKind::Elixir => {
                etir::AdtRepr::Elixir { adt: adt_def.did(), variant: variant.def_id }
            }
            ErlReprKind::NativeRecord => {
                etir::AdtRepr::NativeRecord { record_module: repr.record_module, tag: repr.tag }
            }
            ErlReprKind::Transparent { .. } => etir::AdtRepr::Transparent {
                tag: repr.tag,
                variant: variant.def_id,
                has_siblings: adt_def.variants().len() != 1,
            },
        }
    }

    fn track_reassigns<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> (T, bool) {
        let prev = self.reassign_count;
        let result = f(self);
        let reassigned = self.reassign_count != prev;
        (result, reassigned)
    }
}

fn not_implemented_kind(tcx: TyCtxt<'_>, span: Span, what: &str, kind: impl Debug) -> ! {
    tcx.dcx().span_fatal(span, format!("{what} not yet implemented: {kind:?}"))
}

fn not_implemented(tcx: TyCtxt<'_>, span: Span, what: &str) -> ! {
    tcx.dcx().span_fatal(span, format!("{what} not yet implemented"))
}

#[derive(Clone, Copy)]
enum VarPrefix {
    Witness,
    WitnessesWhere,
    Where,
    Supertraits,
    Vtable,
    Fn,
    Type,
    Const,
    ConstFn,
}

impl VarPrefix {
    fn as_str(self) -> &'static str {
        match self {
            VarPrefix::Witness => "witness",
            VarPrefix::WitnessesWhere => "witnesses_where",
            VarPrefix::Where => "where",
            VarPrefix::Supertraits => "supertraits",
            VarPrefix::Vtable => "vtable",
            VarPrefix::Fn => "fn",
            VarPrefix::Type => "type",
            VarPrefix::Const => "const",
            VarPrefix::ConstFn => "const_fn",
        }
    }

    fn to_name(self, tcx: TyCtxt<'_>, def_id: DefId) -> Symbol {
        let def_id = tcx.trait_item_of(def_id).unwrap_or(def_id);

        let name = if let DefKind::Impl { of_trait: true } = tcx.def_kind(def_id) {
            tcx.item_name(tcx.impl_trait_id(def_id)).as_unescaped_str().to_owned()
        } else if let Some(trait_def_id) = tcx.trait_of_assoc(def_id) {
            format!(
                "{}_{}",
                tcx.item_name(trait_def_id).as_unescaped_str(),
                tcx.item_name(def_id).as_unescaped_str()
            )
        } else {
            tcx.item_name(def_id).as_unescaped_str().to_owned()
        };

        Symbol::intern(&format!("{}_{name}", self.as_str()))
    }
}

pub fn provide(providers: &mut Providers) {
    providers.queries.etir_template = etir_template;
    providers.queries.etir_body = etir_body;
}
