//! Prepare witness objects required for generic function.
//!
//! The witness of `Ty: MyTrait` points to the impl that satisfies the bound:
//! `{Vtable, {S1, …, Sk}, {W1, …, Wn}}`.
//!
//! - `Vtable` contains a fun for each trait function in `MyTrait`.
//! - `S1 … Sk` are nested witnesses for the direct supertraits of `MyTrait`.
//! - `W1 … Wn` are nested witnesses of the where-clauses of the impl.
//!
//! This module analyzes functions to prepare the witnesses needed, and lowers
//! them as synthetic ETIR nodes so translation layer doesn't have to know.

use rustc_data_structures::fx::{FxHashMap, FxIndexMap, IndexEntry};
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_index::{IndexVec, newtype_index};
use rustc_middle::erl::etir::visit::{self, Visitor};
use rustc_middle::erl::etir::{
    Body, Callee, ErlFun, Expr, ExprId, ExprKind, FieldIdx, Pat, PatId, PatKind, PatternInput, Var,
    VarId,
};
use rustc_middle::erl::witness::SupertraitIdx;
use rustc_middle::erl::{ErlCallTarget, ErlModuleResolution};
use rustc_middle::ty::{self, TyCtxt};
use rustc_middle::{bug, span_bug};
use rustc_span::{Ident, Span, Symbol, sym};

use self::resolve::{
    WitnessArgs, WitnessBuildSite, WitnessResolver, WitnessSource, WitnessTuple, witness_params,
};
use super::ExprBlock;

mod plan;
mod resolve;

const VTABLE_INDEX: usize = 0;
const SUPERTRAITS_INDEX: usize = 1;
const WHERE_CLAUSES_INDEX: usize = 2;

newtype_index! {
    #[debug_format = "n{}"]
    struct VarNodeId {}
}

/// A witness-related var that we bind in function's preamble.
/// This can be a witness itself, but it can be a tuple of nodes, a vtable, etc.
/// `VarNode` is part of the tree that describes how we assigned these vars.
struct VarNode {
    var: VarId,
    /// guaranteed `ExprKind::Var(var)`.
    expr: ExprId,
    span: Span,
    value_source: NodeValueSource,
    /// If this is a tuple: its elements can also be nodes. `usize` is the tuple index from 0.
    elements: Vec<(usize, VarNodeId)>,
}

enum NodeValueSource {
    Param,
    Expr(ExprId),
    Element,
}

/// `VarNodeKey` is the dedup key for `VarNode`.
/// We hold a hashtable `FxHashMap<VarNodeKey, VarNode>`, so if we want any particular value
/// (e.g. witness), and we already assigned it to a var, we reuse that var.
#[derive(PartialEq, Eq, Hash)]
enum VarNodeKey<'tcx> {
    Witness(ty::TraitRef<'tcx>),
    Impl { impl_def_id: DefId, where_clauses: Box<[VarNodeId]> },
    Element(VarNodeId, usize),
    Tuple(Box<[VarNodeId]>),
    AssocTypeBounds { of: VarNodeId, def_id: DefId, bounds: Box<[VarNodeId]> },
    AssocConst { def_id: DefId, args: ty::GenericArgsRef<'tcx> },
}

struct Collect<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    owner: DefId,
    body: &'a Body<'tcx>,
    resolver: &'a mut WitnessResolver<'tcx>,
    /// Every item reference, with its target resolved from the owner of the body.
    sites: FxIndexMap<ExprId, (ErlCallTarget<'tcx>, WitnessArgs<'tcx>)>,
}

impl<'a, 'tcx: 'a> Visitor<'a, 'tcx> for Collect<'a, 'tcx> {
    fn etir(&self) -> &'a Body<'tcx> {
        self.body
    }

    fn visit_expr(&mut self, expr_id: ExprId, expr: &'a Expr<'tcx>) {
        visit::walk_expr(self, expr);

        let (ExprKind::Call { callee: Callee::Item { target, .. }, .. }
        | ExprKind::FnItem { target, .. }) = expr.kind
        else {
            return;
        };
        let IndexEntry::Vacant(site) = self.sites.entry(expr_id) else { return };

        let target = self.tcx.erl_resolve_trait_item_target(self.owner, target);
        let witnesses = match target {
            ErlCallTarget::Item { def_id, args } => {
                self.resolver.require_witness_args(expr.span, def_id, args)
            }
            ErlCallTarget::Extern { .. } => WitnessArgs::default(),
            ErlCallTarget::Constructor { .. } | ErlCallTarget::Dynamic => {
                span_bug!(expr.span, "item reference to {target:?}")
            }
        };
        site.insert((target, witnesses));
    }
}

pub(super) fn lower<'tcx>(tcx: TyCtxt<'tcx>, owner: DefId, body: Body<'tcx>) -> Body<'tcx> {
    let mut resolver = WitnessResolver::new(tcx, owner, WitnessBuildSite::User);

    let sites = {
        let mut collect = Collect {
            tcx,
            owner,
            body: &body,
            resolver: &mut resolver,
            sites: FxIndexMap::default(),
        };
        visit::visit_expr_id(&mut collect, body.value);
        collect.sites
    };

    let witnesses = resolver.resolve_witnesses();

    let mut lower = Lower::new(tcx, owner, body, witnesses, tcx.def_span(owner));

    for (id, (target, witnesses)) in sites {
        let span = lower.body[id].span;
        let kind = std::mem::replace(&mut lower.body.exprs[id].kind, ExprKind::Block(Box::new([])));
        lower.body.exprs[id].kind = match kind {
            ExprKind::Call { callee: Callee::Item { forced_extern, .. }, args }
                if witnesses.is_empty() =>
            {
                let fun = lower.witnessless_erl_fun(target, args.len() as u32, span);
                ExprKind::Call { callee: Callee::Fun { fun, forced_extern }, args }
            }
            ExprKind::Call { callee: Callee::Item { forced_extern, .. }, args: _ }
                if lower.is_assoc_const(target) =>
            {
                lower.lower_assoc_const(target, &witnesses, span, forced_extern)
            }
            ExprKind::Call { callee: Callee::Item { forced_extern, .. }, args } => {
                lower.lower_call(target, &witnesses, args, span, forced_extern)
            }
            ExprKind::FnItem { arity, forced_extern, .. } if witnesses.is_empty() => {
                let fun = lower.witnessless_erl_fun(target, arity, span);
                ExprKind::Fun { fun, forced_extern }
            }
            ExprKind::FnItem { arity, forced_extern, .. } => {
                lower.lower_fn_item(target, &witnesses, arity, span, forced_extern)
            }
            kind => span_bug!(span, "item site is not an item reference: {kind:?}"),
        };
    }
    lower.finish()
}

pub(super) fn build_witness_constructor<'tcx>(
    tcx: TyCtxt<'tcx>,
    owner: DefId,
    span: Span,
) -> Body<'tcx> {
    let mut resolver = WitnessResolver::new(tcx, owner, WitnessBuildSite::WitnessConstructor);

    let trait_ref = tcx.impl_trait_ref(owner).instantiate_identity().skip_normalization();
    let supertraits =
        tcx.erl_trait_supertraits(trait_ref.def_id).iter_instantiated_copied(tcx, trait_ref.args);
    let supertraits: Vec<_> =
        supertraits.map(|bound| resolver.require_witness(span, bound)).collect();

    let witnesses = resolver.resolve_witnesses();
    let mut lower = Lower::new(tcx, owner, Body::new(), witnesses, span);

    let supertraits = supertraits
        .into_iter()
        .map(|bound| {
            let node = lower.witness(bound, span);
            lower.var_nodes[node].expr
        })
        .collect();

    let vtable = lower.vtable(owner, trait_ref.def_id, span);
    let supertraits = lower.expr(ExprKind::Tuple { fields: supertraits }, span);
    let bounds = match lower.bounds {
        Some(node) => lower.var_nodes[node].expr,
        None => lower.expr(ExprKind::Tuple { fields: Box::new([]) }, span),
    };

    lower.body.value = lower.witness_tuple(vtable, supertraits, bounds, span);
    lower.finish()
}

pub(super) fn build_assoc_type<'tcx>(
    tcx: TyCtxt<'tcx>,
    owner: DefId,
    trait_item: DefId,
    span: Span,
) -> Body<'tcx> {
    let args = tcx.erl_trait_item_args_for_impl_item(owner);
    let bounds = tcx.erl_assoc_type_bounds(trait_item).iter_instantiated_copied(tcx, args);

    let mut resolver = WitnessResolver::new(tcx, owner, WitnessBuildSite::WitnessConstructor);
    let bounds: Vec<_> = bounds.map(|bound| resolver.require_witness(span, bound)).collect();

    let witnesses = resolver.resolve_witnesses();
    let mut lower = Lower::new(tcx, owner, Body::new(), witnesses, span);

    let bounds = bounds
        .into_iter()
        .map(|bound| {
            let node = lower.witness(bound, span);
            lower.var_nodes[node].expr
        })
        .collect();

    lower.body.value = lower.expr(ExprKind::Tuple { fields: bounds }, span);
    lower.finish()
}

struct Lower<'tcx> {
    tcx: TyCtxt<'tcx>,
    owner: DefId,
    body: Body<'tcx>,
    witnesses: FxHashMap<ty::TraitRef<'tcx>, WitnessSource<'tcx>>,
    self_witness: Option<VarNodeId>,
    bounds: Option<VarNodeId>,
    keys: FxHashMap<VarNodeKey<'tcx>, VarNodeId>,
    var_nodes: IndexVec<VarNodeId, VarNode>,
}

impl<'tcx> Lower<'tcx> {
    fn new(
        tcx: TyCtxt<'tcx>,
        owner: DefId,
        body: Body<'tcx>,
        witnesses: FxHashMap<ty::TraitRef<'tcx>, WitnessSource<'tcx>>,
        span: Span,
    ) -> Self {
        let mut lower = Self {
            tcx,
            owner,
            body,
            witnesses,
            self_witness: None,
            bounds: None,
            keys: FxHashMap::default(),
            var_nodes: IndexVec::new(),
        };

        let params = witness_params(tcx, owner, ty::GenericArgs::identity_for_item(tcx, owner));

        let self_bound_pat = params.self_bound.map(|bound| {
            let name = witness_var_name(tcx, bound.skip_normalization());
            let node = lower.var_node(name, NodeValueSource::Param, span);
            lower.self_witness = Some(node);
            lower.binding_pat(lower.var_nodes[node].var, span)
        });

        let bounds_pat = (!params.bounds.is_empty()).then(|| {
            let name = var_name(tcx, VarPrefix::WitnessesWhere, owner);
            let node = lower.var_node(name, NodeValueSource::Param, span);
            lower.bounds = Some(node);
            lower.binding_pat(lower.var_nodes[node].var, span)
        });

        lower.body.params =
            self_bound_pat.into_iter().chain(bounds_pat).chain(lower.body.params).collect();

        lower
    }

    fn finish(mut self) -> Body<'tcx> {
        let mut prologue = ExprBlock::default();
        for node in self.var_nodes.indices() {
            match self.var_nodes[node].value_source {
                NodeValueSource::Element => continue,
                NodeValueSource::Param => {}
                NodeValueSource::Expr(rhs) => self.bind(&mut prologue, node, rhs),
            }
            // We bind tuple elements in `finish`, because only now we know which ones are needed.
            self.bind_tuple_elements(&mut prologue, node)
        }

        let value = self.body.value;
        self.body.value = prologue.emit(&mut self.body, value);
        self.body
    }

    fn vtable(&mut self, impl_def_id: DefId, trait_def_id: DefId, span: Span) -> ExprId {
        let tcx = self.tcx;
        let implementors = tcx.impl_item_implementor_ids(impl_def_id);

        let fields = tcx
            .erl_vtable_items(trait_def_id)
            .map(|item| {
                let &def_id = implementors.get(&item.def_id).unwrap_or(&item.def_id);

                let module = tcx.erl_definition_module(def_id).unwrap_or_else(|| {
                    span_bug!(span, "`{}` has no definition module", tcx.def_path_str(def_id))
                });

                let fun = ErlFun {
                    module: ErlModuleResolution::Static(module),
                    name: tcx.erl_fn_name(def_id),
                    arity: tcx.erl_fn_arity(def_id) as u32,
                };
                self.expr(ExprKind::Fun { fun, forced_extern: true }, span)
            })
            .collect();

        self.expr(ExprKind::Tuple { fields }, span)
    }

    fn expr(&mut self, kind: ExprKind<'tcx>, span: Span) -> ExprId {
        self.body.exprs.push(Expr { kind, ty: self.tcx.types.term, span })
    }

    fn binding_pat(&mut self, var: VarId, span: Span) -> PatId {
        self.body.pats.push(Pat {
            kind: PatKind::Binding {
                var,
                is_primary: true,
                subpattern: None,
                input: PatternInput::Typed,
            },
            ty: self.tcx.types.term,
            span,
        })
    }

    fn var_node(&mut self, name: Symbol, source: NodeValueSource, span: Span) -> VarNodeId {
        let var = self.body.vars.push(Var { ident: Ident::new(name, span) });
        let value = self.expr(ExprKind::Var(var), span);
        self.var_nodes.push(VarNode {
            var,
            expr: value,
            span,
            value_source: source,
            elements: Vec::new(),
        })
    }

    fn intern(
        &mut self,
        key: VarNodeKey<'tcx>,
        build: impl FnOnce(&mut Self) -> VarNodeId,
    ) -> VarNodeId {
        if let Some(&node) = self.keys.get(&key) {
            return node;
        }
        let node = build(self);
        self.keys.insert(key, node);
        node
    }

    fn element(&mut self, tuple: VarNodeId, index: usize, name: Symbol, span: Span) -> VarNodeId {
        self.intern(VarNodeKey::Element(tuple, index), |lower| {
            let node = lower.var_node(name, NodeValueSource::Element, span);
            lower.var_nodes[tuple].elements.push((index, node));
            node
        })
    }

    fn tuple(&mut self, fields: Vec<VarNodeId>, name: Symbol, span: Span) -> VarNodeId {
        self.intern(VarNodeKey::Tuple(fields.clone().into_boxed_slice()), |lower| {
            let values = fields.iter().map(|&field| lower.var_nodes[field].expr).collect();
            let rhs = lower.expr(ExprKind::Tuple { fields: values }, span);
            lower.var_node(name, NodeValueSource::Expr(rhs), span)
        })
    }

    fn witness(&mut self, bound: ty::TraitRef<'tcx>, span: Span) -> VarNodeId {
        self.intern(VarNodeKey::Witness(bound), |lower| {
            ensure_sufficient_stack(|| lower.witness_node(bound, span))
        })
    }

    fn witness_node(&mut self, bound: ty::TraitRef<'tcx>, span: Span) -> VarNodeId {
        let name = witness_var_name(self.tcx, bound);
        match self.witnesses[&bound].clone() {
            WitnessSource::SelfParam => self.self_witness.unwrap(),
            WitnessSource::Element(tuple_source, index) => {
                let tuple = match tuple_source {
                    WitnessTuple::ItemBounds => self.bounds.unwrap(),
                    WitnessTuple::ImplWhereClauses => {
                        let name =
                            var_name(self.tcx, VarPrefix::Where, self.tcx.parent(self.owner));
                        self.element(self.self_witness.unwrap(), WHERE_CLAUSES_INDEX, name, span)
                    }
                    WitnessTuple::Supertraits { of: of_bound } => {
                        let of = self.witness(of_bound, span);
                        let name = var_name(self.tcx, VarPrefix::Supertraits, of_bound.def_id);
                        self.element(of, SUPERTRAITS_INDEX, name, span)
                    }
                    WitnessTuple::AssocTypeBounds { of, assoc_def_id, bounds } => {
                        let of = self.witness(of, span);
                        let bounds = self.witnesses(bounds, span);
                        self.assoc_type_bounds(of, assoc_def_id, bounds, span)
                    }
                };
                self.element(tuple, index, name, span)
            }
            WitnessSource::ImplCall { impl_def_id, where_clauses } => {
                let where_clauses = self.witnesses(where_clauses, span);
                let key =
                    VarNodeKey::Impl { impl_def_id, where_clauses: where_clauses.clone().into() };

                self.intern(key, |lower| {
                    let expr = lower.impl_witness_call(impl_def_id, where_clauses, span);
                    lower.var_node(name, NodeValueSource::Expr(expr), span)
                })
            }
            WitnessSource::ImplBuild { impl_def_id, supertraits, where_clauses } => {
                let where_clauses = self.witnesses(where_clauses, span);
                let key =
                    VarNodeKey::Impl { impl_def_id, where_clauses: where_clauses.clone().into() };

                self.intern(key, |lower| {
                    let expr =
                        lower.impl_witness(impl_def_id, bound, supertraits, where_clauses, span);
                    lower.var_node(name, NodeValueSource::Expr(expr), span)
                })
            }
        }
    }

    fn witnesses(
        &mut self,
        bounds: impl IntoIterator<Item = ty::TraitRef<'tcx>>,
        span: Span,
    ) -> Vec<VarNodeId> {
        bounds.into_iter().map(|bound| self.witness(bound, span)).collect()
    }

    /// `{Vtable, Supertraits, WhereClauses}`.
    fn impl_witness(
        &mut self,
        impl_def_id: DefId,
        bound: ty::TraitRef<'tcx>,
        supertraits: IndexVec<SupertraitIdx, ty::TraitRef<'tcx>>,
        where_clauses: Vec<VarNodeId>,
        span: Span,
    ) -> ExprId {
        let vtable = self.vtable(impl_def_id, bound.def_id, span);

        let supertraits = self.witnesses(supertraits, span);
        let supertraits = supertraits.into_iter().map(|node| self.var_nodes[node].expr).collect();
        let supertraits = self.expr(ExprKind::Tuple { fields: supertraits }, span);

        let where_clauses =
            where_clauses.into_iter().map(|node| self.var_nodes[node].expr).collect();
        let where_clauses = self.expr(ExprKind::Tuple { fields: where_clauses }, span);

        self.witness_tuple(vtable, supertraits, where_clauses, span)
    }

    /// `{Vtable, Supertraits, WhereClauses}`
    fn witness_tuple(
        &mut self,
        vtable: ExprId,
        supertraits: ExprId,
        where_clauses: ExprId,
        span: Span,
    ) -> ExprId {
        let mut fields = [vtable; 3];
        fields[VTABLE_INDEX] = vtable;
        fields[SUPERTRAITS_INDEX] = supertraits;
        fields[WHERE_CLAUSES_INDEX] = where_clauses;
        self.expr(ExprKind::Tuple { fields: Box::new(fields) }, span)
    }

    /// $patina_witness call
    fn impl_witness_call(
        &mut self,
        impl_def_id: DefId,
        where_clauses: Vec<VarNodeId>,
        span: Span,
    ) -> ExprId {
        let args = if where_clauses.is_empty() {
            Box::default()
        } else {
            let name = var_name(self.tcx, VarPrefix::WitnessesWhere, impl_def_id);
            let tuple = self.tuple(where_clauses, name, span);
            Box::from_iter([self.var_nodes[tuple].expr])
        };

        let host = self
            .tcx
            .erl_definition_module(impl_def_id)
            .unwrap_or_else(|| span_bug!(span, "impl {impl_def_id:?} has no witness host"));

        let fun = ErlFun {
            module: ErlModuleResolution::Static(host),
            name: self.tcx.erl_impl_member_name(impl_def_id, "$patina_witness"),
            arity: args.len() as u32,
        };

        self.expr(ExprKind::Call { callee: Callee::Fun { fun, forced_extern: false }, args }, span)
    }

    /// Different bound indices have different sources, but share one call.
    fn assoc_type_bounds(
        &mut self,
        of: VarNodeId,
        def_id: DefId,
        bounds: Vec<VarNodeId>,
        span: Span,
    ) -> VarNodeId {
        let key = VarNodeKey::AssocTypeBounds { of, def_id, bounds: bounds.clone().into() };
        self.intern(key, |lower| {
            let bounds = (!bounds.is_empty()).then(|| {
                let name = var_name(lower.tcx, VarPrefix::WitnessesWhere, def_id);
                lower.tuple(bounds, name, span)
            });
            let args: Box<[_]> =
                [of].into_iter().chain(bounds).map(|node| lower.var_nodes[node].expr).collect();

            let callee_node = lower.dispatch(of, def_id, span);
            let callee = Callee::Value(lower.var_nodes[callee_node].expr);

            let expr = lower.expr(ExprKind::Call { callee, args }, span);

            let name = var_name(lower.tcx, VarPrefix::Where, lower.tcx.parent(def_id));
            lower.var_node(name, NodeValueSource::Expr(expr), span)
        })
    }

    fn dispatch(&mut self, witness: VarNodeId, item: DefId, span: Span) -> VarNodeId {
        let trait_def_id = self.tcx.parent(item);
        let (slot, entry) = self
            .tcx
            .erl_vtable_items(trait_def_id)
            .enumerate()
            .find(|(_, entry)| entry.def_id == item)
            .unwrap_or_else(|| span_bug!(span, "{item:?} has no vtable slot"));
        let slot_name = match entry.kind {
            ty::AssocKind::Fn { .. } => var_name(self.tcx, VarPrefix::Fn, item),
            ty::AssocKind::Type { .. } => var_name(self.tcx, VarPrefix::Type, item),
            ty::AssocKind::Const { .. } => var_name(self.tcx, VarPrefix::ConstFn, item),
        };

        let vtable_name = var_name(self.tcx, VarPrefix::Vtable, trait_def_id);
        let vtable = self.element(witness, VTABLE_INDEX, vtable_name, span);
        self.element(vtable, slot, slot_name, span)
    }

    fn witnessless_erl_fun(&self, target: ErlCallTarget<'tcx>, arity: u32, span: Span) -> ErlFun {
        let (module, name) = self.target_module_and_name(target, span);
        let module =
            module.unwrap_or_else(|| span_bug!(span, "`{name}` dispatches without a self witness"));
        ErlFun { module, name, arity }
    }

    fn target_module_and_name(
        &self,
        target: ErlCallTarget<'tcx>,
        span: Span,
    ) -> (Option<ErlModuleResolution>, Symbol) {
        let module = self.tcx.erl_module_for_target(self.owner, target);
        let name = match target {
            ErlCallTarget::Item { def_id, .. } => self.tcx.erl_fn_name(def_id),
            ErlCallTarget::Extern { function, .. } => self.tcx.item_name(function),
            ErlCallTarget::Constructor { .. } | ErlCallTarget::Dynamic => {
                span_bug!(span, "item reference to {target:?}")
            }
        };
        (module, name)
    }

    fn lower_call(
        &mut self,
        target: ErlCallTarget<'tcx>,
        witnesses: &WitnessArgs<'tcx>,
        args: Box<[ExprId]>,
        span: Span,
        forced_extern: bool,
    ) -> ExprKind<'tcx> {
        let (module, name) = self.target_module_and_name(target, span);

        let self_witness = witnesses.self_witness.map(|bound| self.witness(bound, span));

        let bounds = (!witnesses.bounds.is_empty()).then(|| {
            let fields = witnesses.bounds.iter().map(|&bound| self.witness(bound, span)).collect();
            let name = match target {
                ErlCallTarget::Item { def_id, .. } => {
                    var_name(self.tcx, VarPrefix::WitnessesWhere, def_id)
                }
                _ => sym::witnesses_where,
            };
            self.tuple(fields, name, span)
        });

        let witness_nodes: Vec<_> = self_witness.into_iter().chain(bounds).collect();

        let callee = match module {
            Some(module) => {
                let arity = (witness_nodes.len() + args.len()) as u32;
                Callee::Fun { fun: ErlFun { module, name, arity }, forced_extern }
            }
            // The callee dispatches through its self witness.
            None => {
                let ErlCallTarget::Item { def_id, .. } = target else { bug!("non-item dispatch") };
                let fun = self.dispatch(self_witness.unwrap(), def_id, span);
                Callee::Value(self.var_nodes[fun].expr)
            }
        };

        let witnesses = witness_nodes.iter().map(|&node| self.var_nodes[node].expr);
        ExprKind::Call { callee, args: witnesses.chain(args).collect() }
    }

    fn is_assoc_const(&self, target: ErlCallTarget<'tcx>) -> bool {
        matches!(
            target,
            ErlCallTarget::Item { def_id, .. }
                if matches!(self.tcx.def_kind(def_id), DefKind::AssocConst { .. })
        )
    }

    /// Lowers an associated const call into the witness prologue. From here on it will be a Var.
    fn lower_assoc_const(
        &mut self,
        target: ErlCallTarget<'tcx>,
        witnesses: &WitnessArgs<'tcx>,
        span: Span,
        forced_extern: bool,
    ) -> ExprKind<'tcx> {
        let ErlCallTarget::Item { def_id, args } = target else { bug!("non-item const") };
        let node = self.intern(VarNodeKey::AssocConst { def_id, args }, |lower| {
            let call_kind = lower.lower_call(target, witnesses, Box::new([]), span, forced_extern);
            let call = lower.expr(call_kind, span);
            let name = var_name(lower.tcx, VarPrefix::Const, def_id);
            lower.var_node(name, NodeValueSource::Expr(call), span)
        });
        ExprKind::Var(self.var_nodes[node].var)
    }

    /// Lowers a fn item as a closure over already-bound witnesses and its dispatch fun.
    fn lower_fn_item(
        &mut self,
        target: ErlCallTarget<'tcx>,
        witnesses: &WitnessArgs<'tcx>,
        arity: u32,
        span: Span,
        forced_extern: bool,
    ) -> ExprKind<'tcx> {
        let (params, args): (Vec<_>, Vec<_>) = (0..arity)
            .map(|_| {
                let var = self.body.vars.push(Var { ident: Ident::new(sym::arg, span) });
                (self.binding_pat(var, span), self.expr(ExprKind::Var(var), span))
            })
            .collect();

        let call = self.lower_call(target, witnesses, args.into(), span, forced_extern);
        let body = self.expr(call, span);
        ExprKind::Closure { params: params.into(), body }
    }

    fn bind(&mut self, prologue: &mut ExprBlock, node: VarNodeId, rhs: ExprId) {
        let span = self.var_nodes[node].span;
        let lhs = self.binding_pat(self.var_nodes[node].var, span);
        let assign = self.expr(ExprKind::Assign { lhs, rhs }, span);
        prologue.0.push(assign);
    }

    fn bind_tuple_elements(&mut self, prologue: &mut ExprBlock, tuple: VarNodeId) {
        let mut elements = self.var_nodes[tuple].elements.clone();
        // When we call `element(N, Tuple)`, the later `element(N-K, Tuple)` are free.
        elements.sort_by_key(|&(index, _)| std::cmp::Reverse(index));

        let span = self.var_nodes[tuple].span;
        let tuple = self.var_nodes[tuple].expr;

        for &(index, node) in &elements {
            let field = ExprKind::TupleField { lhs: tuple, position: FieldIdx::from_usize(index) };
            let value = self.expr(field, span);
            self.bind(prologue, node, value);
        }

        for (_, node) in elements {
            self.bind_tuple_elements(prologue, node);
        }
    }
}

/// Puts the trait name into witness variable name for debuggability.
fn witness_var_name(tcx: TyCtxt<'_>, bound: ty::TraitRef<'_>) -> Symbol {
    var_name(tcx, VarPrefix::Witness, bound.def_id)
}

fn var_name(tcx: TyCtxt<'_>, prefix: VarPrefix, def_id: DefId) -> Symbol {
    let def_id = tcx.trait_item_of(def_id).unwrap_or(def_id);

    let name = if let DefKind::Impl { of_trait: true } = tcx.def_kind(def_id) {
        tcx.item_name(tcx.impl_trait_id(def_id)).to_string()
    } else if let Some(trait_def_id) = tcx.trait_of_assoc(def_id) {
        format!("{}_{}", tcx.item_name(trait_def_id), tcx.item_name(def_id))
    } else {
        tcx.item_name(def_id).to_string()
    };

    Symbol::intern(&format!("{}_{name}", prefix.as_str()))
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
}
