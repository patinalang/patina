use std::fmt::Debug;

use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::erl::ErlReprKind;
use rustc_middle::erl::etir::{self, ExprId, JoinId, LoopId, PatId, PatternInput, VarId};
use rustc_middle::middle::region;
use rustc_middle::thir::{LocalVarId, Thir};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_middle::util::Providers;
use rustc_middle::{bug, span_bug};
use rustc_span::Span;

mod assigned_outer_vars;
mod expr;
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
}

struct BodyBuildCx<'tcx> {
    tcx: TyCtxt<'tcx>,
    body: etir::Body<'tcx>,
    var_ids: FxHashMap<LocalVarId, VarId>,
    loop_ids: FxHashMap<region::Scope, LoopId>,
    join_ids: FxHashMap<ExprId, JoinId>,
}

pub(crate) fn etir_body<'tcx>(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> &'tcx etir::Body<'tcx> {
    match tcx.def_kind(def_id) {
        DefKind::Impl { .. } => {
            let span = tcx.hir_expect_item(def_id).expect_impl().self_ty.span;
            let body = witness::build_witness_constructor(tcx, def_id.to_def_id(), span);
            return tcx.arena.alloc(body);
        }
        DefKind::AssocTy => {
            let span = tcx.def_span(def_id);
            let trait_item = tcx
                .trait_item_of(def_id)
                .unwrap_or_else(|| span_bug!(span, "associated type witness outside a trait impl"));
            let body = witness::build_assoc_type(tcx, def_id.to_def_id(), trait_item, span);
            return tcx.arena.alloc(body);
        }
        _ => {}
    }

    if tcx.is_typeck_child(def_id.to_def_id()) {
        bug!("{def_id:?} child has no ETIR body");
    }

    let mut cx = BodyBuildCx {
        tcx,
        body: etir::Body::new(),
        var_ids: FxHashMap::default(),
        loop_ids: FxHashMap::default(),
        join_ids: FxHashMap::default(),
    };

    let (params, value) = ThirBodyCx::lower(&mut cx, def_id);

    cx.body.params = params;
    cx.body.value = value;

    let mut body = witness::lower(tcx, def_id.to_def_id(), cx.body);
    assigned_outer_vars::analyze(&mut body);

    tcx.arena.alloc(body)
}

struct ThirBodyCx<'a, 'tcx> {
    cx: &'a mut BodyBuildCx<'tcx>,
    owner: LocalDefId,
    thir: &'a Thir<'tcx>,
}

impl<'a, 'tcx> ThirBodyCx<'a, 'tcx> {
    fn lower(cx: &mut BodyBuildCx<'tcx>, owner: LocalDefId) -> (Box<[PatId]>, ExprId) {
        let (thir, root) = cx.tcx.thir_body(owner).unwrap_or_else(|guar| guar.raise_fatal());
        let thir = thir.borrow();

        let mut cx = ThirBodyCx { cx, owner, thir: &thir };

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

        (params, value)
    }

    fn push_expr(&mut self, kind: etir::ExprKind<'tcx>, ty: Ty<'tcx>, span: Span) -> ExprId {
        self.cx.body.exprs.push(etir::Expr { kind, ty, span })
    }

    fn push_pat(&mut self, kind: etir::PatKind<'tcx>, ty: Ty<'tcx>, span: Span) -> PatId {
        self.cx.body.pats.push(etir::Pat { kind, ty, span })
    }

    fn push_loop(&mut self, label: region::Scope, outer_vars: Box<[etir::LoopOuterVar]>) -> LoopId {
        let loop_id = self.cx.body.loops.push(etir::Loop { outer_vars });
        self.cx.loop_ids.insert(label, loop_id);
        loop_id
    }

    fn var(&mut self, id: LocalVarId) -> VarId {
        let tcx = self.cx.tcx;
        let vars = &mut self.cx.body.vars;
        *self
            .cx
            .var_ids
            .entry(id)
            .or_insert_with(|| vars.push(etir::Var { ident: tcx.hir_ident(id.0) }))
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
}

fn not_implemented_kind(tcx: TyCtxt<'_>, span: Span, what: &str, kind: impl Debug) -> ! {
    tcx.dcx().span_fatal(span, format!("{what} not yet implemented: {kind:?}"))
}

fn not_implemented(tcx: TyCtxt<'_>, span: Span, what: &str) -> ! {
    tcx.dcx().span_fatal(span, format!("{what} not yet implemented"))
}

pub fn provide(providers: &mut Providers) {
    providers.queries.etir_body = etir_body;
}
