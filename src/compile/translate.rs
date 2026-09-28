mod adts;
mod binary_pattern;
mod body;
mod elixir_compat;
mod loops;
mod scopes;

use std::cell::RefCell;
use std::path::PathBuf;

use body::{BodyCx, TranslatedBody};
use rustc_arena::DroplessArena;
use rustc_data_structures::fx::{FxHashMap, FxHashSet, FxIndexMap, FxIndexSet};
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LocalDefId, LocalModDefId};
use rustc_hir::{BodyId, ItemKind, Mod, VariantData};
use rustc_middle::erl::{
    ErlCallTarget, ErlModuleId, ErlModuleResolution, ErlReprKind, ErlVariantRepr, ErlWitnessSlot,
};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_middle::{bug, span_bug};
use rustc_span::{Span, Symbol, kw, sym};
use scopes::Scopes;

use crate::compile::eaf_builder::EafBuilder;
use crate::compile::impl_collector::ImplCollector;
use crate::erlang::abstract_format as eaf;

#[macro_export]
macro_rules! span_fatal {
    ($tcx:expr, $span:expr, $($arg:tt)*) => {
        $tcx.sess.dcx().span_fatal($span, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! not_implemented {
    ($tcx:expr, $span:expr, $name:literal, $kind:expr) => {
        span_fatal!($tcx, $span, "{} not yet implemented: {:?}", $name, $kind)
    };
    ($tcx:expr, $span:expr, $name:literal) => {
        span_fatal!($tcx, $span, "{} not yet implemented", $name)
    };
}

#[derive(Default)]
pub(crate) struct ModuleParts<'arena> {
    pub forms: Vec<eaf::ModuleForm<'arena>>,
    pub exports: SmallVec<[(Symbol, u32); 16]>,
    last_file: Option<PathBuf>,
}

#[derive(Clone, Copy)]
enum TranslatedConst<'arena> {
    Translating,
    Value(eaf::Expr<'arena>),
}

enum Witness {
    Param(ty::ParamTy),
    ModuleWithSubtypes { module: Symbol, subtypes: Vec<Witness> },
    Projection { name: Symbol, module: Option<Symbol>, witnesses: Vec<Witness> },
}

pub(super) struct Translator<'tcx, 'arena> {
    tcx: TyCtxt<'tcx>,
    afb: EafBuilder<'tcx, 'arena>,
    impls: ImplCollector<'tcx>,
    translated_consts: RefCell<FxHashMap<LocalDefId, TranslatedConst<'arena>>>,
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    pub fn new(tcx: TyCtxt<'tcx>, arena: &'arena DroplessArena) -> Translator<'tcx, 'arena> {
        Translator {
            tcx,
            afb: EafBuilder::new(tcx.sess.source_map(), arena),
            impls: ImplCollector::new(tcx),
            translated_consts: RefCell::new(FxHashMap::default()),
        }
    }

    pub(super) fn translate_body(
        &self,
        owner: LocalDefId,
        parts: &mut ModuleParts<'arena>,
        scopes: &mut Scopes,
        witness_vars: &FxIndexMap<ty::ParamTy, Symbol>,
    ) -> TranslatedBody<'arena> {
        let (thir, root) =
            self.tcx.thir_body(owner).unwrap_or_else(|guaranteed| guaranteed.raise_fatal());

        let thir = thir.borrow();
        let mut cx = BodyCx::new(self, owner, &thir, parts, scopes, witness_vars);

        let mut params = SmallVec::new();

        for param in thir.params.iter() {
            let Some(_hir_id) = param.hir_id else { continue };

            let pat = param
                .pat
                .as_deref()
                .unwrap_or_else(|| bug!("explicit parameter must have a THIR pattern"));

            params.push(cx.translate_single_pattern(pat));
        }

        let value = cx.translate_expr(root);

        TranslatedBody { params, value }
    }

    pub(super) fn translate_const(
        &self,
        span: Span,
        def_id: LocalDefId,
        parts: &mut ModuleParts<'arena>,
    ) -> eaf::Expr<'arena> {
        match self.translated_consts.borrow().get(&def_id) {
            Some(TranslatedConst::Value(value)) => return *value,
            Some(TranslatedConst::Translating) => span_fatal!(
                self.tcx,
                span,
                "recursive named constant `{}`",
                self.tcx.item_name(def_id)
            ),
            None => {}
        }

        self.translated_consts.borrow_mut().insert(def_id, TranslatedConst::Translating);
        let value =
            self.translate_body(def_id, parts, &mut Scopes::new(), &FxIndexMap::default()).value;
        self.translated_consts.borrow_mut().insert(def_id, TranslatedConst::Value(value));
        value
    }

    fn translate_consts(&self) {
        let mut parts = ModuleParts::default();

        for owner in self.tcx.hir_crate_items(()).owners() {
            let def_id = owner.def_id;
            match self.tcx.def_kind(def_id) {
                DefKind::Const { .. }
                    if !self.tcx.generics_of(def_id).own_requires_monomorphization()
                        && !self.tcx.is_type_const(def_id) =>
                {
                    self.translate_const(self.tcx.def_span(def_id), def_id, &mut parts);
                }
                _ => {}
            }
        }

        if !parts.forms.is_empty() || !parts.exports.is_empty() {
            bug!("named const translation emitted module forms or exports");
        }
    }

    pub fn translate(&self) -> Vec<(Symbol, eaf::Module<'arena>)> {
        self.translate_consts();

        let mut modules = Vec::new();

        for owner in self.tcx.hir_crate_items(()).owners() {
            let def_id = owner.def_id;

            let form = match self.tcx.def_kind(def_id) {
                DefKind::Mod => self.translate_module(def_id),
                DefKind::Struct => self.translate_struct(def_id),
                DefKind::Enum => self.translate_enum(def_id),
                _ => continue,
            };

            modules.push(form);
        }

        for module_id in self.impls.prim_modules() {
            modules.push(self.translate_prim_module(*module_id));
        }

        modules
    }

    fn translate_module(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let module_id = LocalModDefId::new_unchecked(def_id);
        let (module, _span, _hir_id) = self.tcx.hir_get_module(module_id);
        let erl_module_id = ErlModuleId::Mod(def_id.to_def_id());
        let parts = self.collect_module_fns(erl_module_id, module);
        let mod_name = self.tcx.erl_module_name(erl_module_id);
        self.assemble_module(module.spans.inner_span, mod_name, [], parts)
    }

    fn assemble_module(
        &self,
        span: Span,
        mod_name: Symbol,
        record_exports: impl IntoIterator<Item = Symbol>,
        parts: ModuleParts<'arena>,
    ) -> (Symbol, eaf::Module<'arena>) {
        let file = self.file_path(span);
        let file_attr = self.afb.file_attr(span, &file.to_string_lossy());
        let module_attr = self.afb.module_attr(span, mod_name);
        let compile_attr = self.afb.wild_attr(span, sym::compile, [sym::no_auto_import]);
        let record_export_attr = self.afb.export_record_attr(span, record_exports);
        let export_attr = self.afb.export_attr(span, parts.exports);

        // Workaround for https://github.com/erlang/otp/issues/11587 :
        // `erlang:is_record/3` cannot be used in guards if it's not imported.
        let import_attr = self.afb.import_attr(span, sym::erlang, [(sym::is_record, 3)]);

        let attrs =
            [file_attr, module_attr, compile_attr, import_attr, record_export_attr, export_attr];

        (mod_name, self.afb.module(attrs.into_iter().chain(parts.forms)))
    }

    fn module_parts(&self, span: Span) -> ModuleParts<'arena> {
        ModuleParts { last_file: Some(self.file_path(span)), ..Default::default() }
    }

    /// The file that contains the outermost macro call site of `span`.
    fn file_path(&self, span: Span) -> PathBuf {
        let span = span.source_callsite();
        self.tcx
            .sess
            .source_map()
            .span_to_filename(span)
            .into_local_path()
            .unwrap_or_else(|| span_fatal!(self.tcx, span, "no filename found"))
    }

    fn push_function(
        &self,
        parts: &mut ModuleParts<'arena>,
        span: Span,
        function: eaf::ModuleForm<'arena>,
    ) {
        // If the function is in a different file than the previous, push a `-file` attribute
        // as well.
        let file = self.file_path(span);
        if parts.last_file.as_ref() != Some(&file) {
            parts.forms.push(self.afb.file_attr(span, &file.to_string_lossy()));
            parts.last_file = Some(file);
        }
        parts.forms.push(function);
    }

    fn translate_callable(
        &self,
        module_id: ErlModuleId,
        def_id: LocalDefId,
        span: Span,
        body: BodyId,
        parts: &mut ModuleParts<'arena>,
    ) {
        let owner = self.tcx.hir_body_owner_def_id(body);
        let mut scope = Scopes::new();

        let (mut params, witness_vars) =
            self.translate_witness_params(def_id, span, [], &mut scope);

        let translated = self.translate_body(owner, parts, &mut scope, &witness_vars);
        params.extend(translated.params);

        let name = self.tcx.erl_fn_name(def_id);
        let arity = params.len() as u32;

        let form = self.afb.simple_function(span, name, params, translated.value);

        self.push_function(parts, span, form);
        if self.tcx.is_erl_exported_fn(module_id, def_id.to_def_id()) {
            parts.exports.push((name, arity));
        }
    }

    /// Returns the witness parameter patterns and their type-parameter variable bindings.
    fn translate_witness_params(
        &self,
        def_id: LocalDefId,
        span: Span,
        extra_params: impl IntoIterator<Item = ty::ParamTy>,
        scopes: &mut Scopes,
    ) -> (SmallVec<[eaf::Pattern<'arena>; 8]>, FxIndexMap<ty::ParamTy, Symbol>) {
        let tcx = self.tcx;
        let def_id = def_id.to_def_id();
        let afb = &self.afb;

        let witness_vars = tcx
            .erl_witness_params(def_id)
            .into_iter()
            // Also give variables to `extra_params`, if the witness parameters can bind them.
            // The witness function of an associated type needs these variables. The value of
            // the type can use unbounded parameters.
            .chain(extra_params)
            .collect::<FxIndexSet<_>>()
            .into_iter()
            .map(|param| (param, scopes.fresh_var(param.name).erl_name))
            .collect::<FxIndexMap<_, _>>();

        let params = match tcx.trait_impl_of_assoc(def_id) {
            Some(impl_def_id) => {
                self.impl_item_witness_pats(def_id, impl_def_id, span, &witness_vars, scopes)
            }
            // Elsewhere, including in a default body, each witness binds to the variable of
            // its type parameter.
            None => tcx
                .erl_witness_layout(def_id)
                .into_iter()
                .map(|slot| {
                    let name = witness_vars.get(&slot.param()).unwrap_or_else(|| {
                        bug!("no witness variable for `{}` of {def_id:?}", slot.param())
                    });
                    afb.pat_var(span, *name)
                })
                .collect(),
        };

        (params, witness_vars)
    }

    /// The witness parameter patterns of the trait impl item `def_id`. Callers pass the
    /// witnesses of the trait item it implements. The patterns recover the impl's generics
    /// from them.
    fn impl_item_witness_pats(
        &self,
        def_id: DefId,
        impl_def_id: DefId,
        span: Span,
        witness_vars: &FxIndexMap<ty::ParamTy, Symbol>,
        scopes: &mut Scopes,
    ) -> SmallVec<[eaf::Pattern<'arena>; 8]> {
        let tcx = self.tcx;
        let afb = &self.afb;
        let parent_generic_count = tcx.generics_of(def_id).parent_count as u32;

        let trait_args = tcx
            .impl_opt_trait_ref(impl_def_id)
            .unwrap_or_else(|| bug!("trait impl item {def_id:?} without a trait ref"))
            .skip_binder()
            .args;

        // The trait item's own type parameters match the impl item's by position.
        fn own_type_params(tcx: TyCtxt<'_>, def_id: DefId) -> impl Iterator<Item = ty::ParamTy> {
            tcx.generics_of(def_id)
                .own_params
                .iter()
                .filter(|param| matches!(param.kind, ty::GenericParamDefKind::Type { .. }))
                .map(ty::ParamTy::for_def)
        }
        let trait_item = tcx.erl_fn_item(def_id);
        let impl_params = own_type_params(tcx, trait_item)
            .zip(own_type_params(tcx, def_id))
            .collect::<FxHashMap<_, _>>();

        let mut params = SmallVec::new();
        let mut self_type_params = FxHashSet::default();

        for slot in tcx.erl_witness_layout(def_id) {
            let pat = match slot {
                // The self-type witness, destructured to recover the impl's own generics that
                // are fixed by the self type.
                // E.g. `impl<T: Debug> Debug for Vec<T>` binds `T` from `{_, {TMod}}`.
                ErlWitnessSlot::TraitSelf(_) => {
                    let self_ty =
                        tcx.type_of(impl_def_id).instantiate_identity().skip_normalization();
                    self.witness_subtypes_pat(
                        self_ty,
                        witness_vars,
                        parent_generic_count,
                        span,
                        &mut self_type_params,
                    )
                    .unwrap_or_else(|| {
                        let name = scopes.fresh_var(kw::SelfUpper).erl_name;
                        afb.pat_var(span, name)
                    })
                }

                ErlWitnessSlot::TraitParam(trait_param) => {
                    let arg_ty = trait_args.type_at(trait_param.index as usize);
                    match arg_ty.kind() {
                        // Already recovered from the self-type witness.
                        // They are still part of the function arity because dynamic callers
                        // (calling through a trait) can't know beforehand which params are
                        // repeated on the self-type.
                        ty::Param(param) if self_type_params.contains(param) => {
                            afb.pat_var(span, kw::Underscore)
                        }
                        ty::Param(param)
                            if param.index < parent_generic_count
                                && let Some(&name) = witness_vars.get(param) =>
                        {
                            afb.pat_var(span, name)
                        }
                        ty::Param(_) => {
                            let name = scopes.fresh_var(trait_param.name).erl_name;
                            afb.pat_var(span, name)
                        }
                        _ => {
                            let name = scopes.fresh_var(trait_param.name).erl_name;
                            let trait_pat = afb.pat_var(span, name);
                            let witness_pat =
                                self.witness_pat(def_id, span, arg_ty, witness_vars, scopes);
                            afb.pat_match(span, trait_pat, witness_pat)
                        }
                    }
                }

                // Callers pass witnesses for the trait item's bounds. An impl item with fewer
                // bounds ignores the extra witnesses.
                ErlWitnessSlot::ItemParam(trait_item_param) => {
                    let param = impl_params.get(&trait_item_param).unwrap_or_else(|| {
                        bug!("{def_id:?} has no type parameter for `{trait_item_param}`")
                    });
                    match witness_vars.get(param) {
                        Some(&name) => afb.pat_var(span, name),
                        None => afb.pat_var(span, kw::Underscore),
                    }
                }
            };
            params.push(pat);
        }

        params
    }

    fn witness_pat(
        &self,
        owner: DefId,
        span: Span,
        ty: Ty<'tcx>,
        witness_vars: &FxIndexMap<ty::ParamTy, Symbol>,
        scopes: &mut Scopes,
    ) -> eaf::Pattern<'arena> {
        fn render<'tcx, 'arena>(
            witness: Witness,
            tcx: TyCtxt<'tcx>,
            afb: &EafBuilder<'tcx, 'arena>,
            span: Span,
            witness_vars: &FxIndexMap<ty::ParamTy, Symbol>,
            scopes: &mut Scopes,
        ) -> eaf::Pattern<'arena> {
            match witness {
                Witness::Param(param) => match witness_vars.get(&param) {
                    Some(&name) => afb.pat_var(span, name),
                    None => {
                        let name = scopes.fresh_var(param.name).erl_name;
                        afb.pat_var(span, name)
                    }
                },
                Witness::ModuleWithSubtypes { module, subtypes } => {
                    let subtype_pats = subtypes
                        .into_iter()
                        .map(|subtype| render(subtype, tcx, afb, span, witness_vars, scopes));
                    let subtypes_pat = afb.pat_tuple(span, subtype_pats);
                    afb.pat_tuple(span, [afb.pat_atom(span, module), subtypes_pat])
                }
                Witness::Projection { name, .. } => {
                    not_implemented!(tcx, span, "associated type in a witness pattern", name)
                }
            }
        }

        let witness = self.build_witness(owner, span, ty);
        render(witness, self.tcx, &self.afb, span, witness_vars, scopes)
    }

    fn build_witness(&self, owner: DefId, span: Span, ty: Ty<'tcx>) -> Witness {
        let tcx = self.tcx;
        // A concrete projection becomes its type here, so it needs no call at runtime.
        let ty = tcx.erl_normalize(owner, ty);
        match ty.kind() {
            ty::Param(param_ty) => Witness::Param(*param_ty),

            ty::Alias(alias) if let ty::AliasTyKind::Projection { def_id } = alias.kind => {
                if !alias.trait_ref_and_own_args(tcx).1.is_empty() {
                    not_implemented!(tcx, span, "witness for a generic associated type", ty);
                }

                let witnesses = tcx
                    .erl_call_witness_tys(def_id, alias.args)
                    .into_iter()
                    .map(|witness_ty| self.build_witness(owner, span, witness_ty))
                    .collect();

                // `None` dispatches through the receiver's witness at runtime.
                let target = ErlCallTarget::Item { def_id, args: alias.args };
                let module = match tcx.erl_module_for_target(owner, target) {
                    Some(ErlModuleResolution::Static(module_id)) => {
                        Some(tcx.erl_module_name(module_id))
                    }
                    Some(ErlModuleResolution::Dynamic) => {
                        not_implemented!(tcx, span, "witness for a dynamic type", ty)
                    }
                    None => None,
                };

                Witness::Projection { name: tcx.erl_fn_name(def_id), module, witnesses }
            }

            _ => {
                let module_id = match tcx.erl_module_for_type(owner, ty) {
                    Some(ErlModuleResolution::Static(module_id)) => module_id,
                    Some(ErlModuleResolution::Dynamic) => {
                        not_implemented!(tcx, span, "witness for a dynamic type", ty)
                    }
                    None => not_implemented!(tcx, span, "witness for a type with no module", ty),
                };

                let subtypes = tcx
                    .subtypes_of(ty)
                    .into_iter()
                    .map(|subty| self.build_witness(owner, span, subty))
                    .collect();

                Witness::ModuleWithSubtypes { module: tcx.erl_module_name(module_id), subtypes }
            }
        }
    }

    /// The expression that constructs `witness` at runtime.
    fn materialize_witness(
        &self,
        span: Span,
        witness: Witness,
        witness_vars: &FxIndexMap<ty::ParamTy, Symbol>,
        scopes: &mut Scopes,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;
        match witness {
            Witness::Param(param) => match witness_vars.get(&param) {
                Some(&var) => afb.expr_var(span, var),
                None => afb.expr_atom(span, sym::dollar_patina_unbounded_type),
            },
            Witness::ModuleWithSubtypes { module, subtypes } => {
                let subtype_exprs = subtypes
                    .into_iter()
                    .map(|subtype| self.materialize_witness(span, subtype, witness_vars, scopes))
                    .collect::<SmallVec<[_; 4]>>();
                let subtypes_expr_tuple = afb.expr_tuple(span, subtype_exprs);
                afb.expr_tuple(span, [afb.expr_atom(span, module), subtypes_expr_tuple])
            }
            Witness::Projection { name, module, witnesses } => {
                let mut exprs = witnesses
                    .into_iter()
                    .map(|witness| self.materialize_witness(span, witness, witness_vars, scopes))
                    .collect::<SmallVec<[_; 4]>>()
                    .into_iter();

                match module {
                    Some(module) => afb.expr_remote_call(
                        span,
                        afb.expr_atom(span, module),
                        afb.expr_atom(span, name),
                        exprs,
                    ),
                    None => {
                        let receiver = exprs.next().unwrap_or_else(|| {
                            span_bug!(span, "associated type `{name}` has no receiver witness")
                        });
                        self.dispatch_call(span, name, receiver, exprs, scopes)
                    }
                }
            }
        }
    }

    /// Call `name` through `receiver_witness`.
    fn dispatch_call(
        &self,
        span: Span,
        name: Symbol,
        receiver_witness: eaf::Expr<'arena>,
        args: impl IntoIterator<Item = eaf::Expr<'arena>>,
        scopes: &mut Scopes,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        let (binding, receiver_witness) = match receiver_witness {
            eaf::Expr::Var(..) => (None, receiver_witness),
            _ => {
                let var = scopes.fresh_var(sym::witness).erl_name;
                let binding = afb.expr_match(span, afb.pat_var(span, var), receiver_witness);
                (Some(binding), afb.expr_var(span, var))
            }
        };

        let module = afb.expr_remote_call(
            span,
            afb.expr_atom(span, sym::erlang),
            afb.expr_atom(span, sym::element),
            [afb.expr_int(span, 1), receiver_witness],
        );
        let call = afb.expr_remote_call(
            span,
            module,
            afb.expr_atom(span, name),
            [receiver_witness].into_iter().chain(args),
        );

        match binding {
            Some(binding) => afb.expr_block(span, [binding, call]),
            None => call,
        }
    }

    /// The pattern for a single witness subtype slot, or `None` when the slot binds nothing
    /// and should be a wildcard.
    /// `self_type_params` gathers the generic parameters that are fixed by the self type.
    fn witness_self_type_subtype_pat(
        &self,
        ty: Ty<'tcx>,
        witness_vars: &FxIndexMap<ty::ParamTy, Symbol>,
        parent_generic_count: u32,
        span: Span,
        self_type_params: &mut FxHashSet<ty::ParamTy>,
    ) -> Option<eaf::Pattern<'arena>> {
        match ty.kind() {
            ty::Param(param)
                if param.index < parent_generic_count
                    && let Some(&name) = witness_vars.get(param) =>
            {
                self_type_params.insert(*param);
                Some(self.afb.pat_var(span, name))
            }
            ty::Param(_) => None,
            _ => self.witness_subtypes_pat(
                ty,
                witness_vars,
                parent_generic_count,
                span,
                self_type_params,
            ),
        }
    }

    /// The witness pattern `{_, {..subtype pats}}` for `ty`, binding every generic fixed by `ty`
    /// that needs a witness. `None` when no subtype binds anything, in which case the whole
    /// witness can be ignored.
    fn witness_subtypes_pat(
        &self,
        ty: Ty<'tcx>,
        witness_vars: &FxIndexMap<ty::ParamTy, Symbol>,
        parent_generic_count: u32,
        span: Span,
        self_type_params: &mut FxHashSet<ty::ParamTy>,
    ) -> Option<eaf::Pattern<'arena>> {
        let afb = &self.afb;

        let mut has_any_subtype_pat = false;

        let subtype_pats = self
            .tcx
            .subtypes_of(ty)
            .into_iter()
            .map(|subty| {
                self.witness_self_type_subtype_pat(
                    subty,
                    witness_vars,
                    parent_generic_count,
                    span,
                    self_type_params,
                )
            })
            .inspect(|pat| has_any_subtype_pat |= pat.is_some())
            .map(|pat| pat.unwrap_or_else(|| afb.pat_var(span, kw::Underscore)));

        let subtype_tuple_pat = afb.pat_tuple(span, subtype_pats);

        has_any_subtype_pat
            .then(|| afb.pat_tuple(span, [afb.pat_var(span, kw::Underscore), subtype_tuple_pat]))
    }

    fn collect_module_fns(
        &self,
        module_id: ErlModuleId,
        module: &Mod<'tcx>,
    ) -> ModuleParts<'arena> {
        let mut parts = self.module_parts(module.spans.inner_span);

        parts.forms.push(elixir_compat::info_function(&self.afb, module.spans.inner_span, None));
        parts.exports.push((sym::__info__, 1));

        for item_id in module.item_ids {
            let item = self.tcx.hir_item(*item_id);
            let def_id = item.owner_id.def_id;
            match item.kind {
                ItemKind::Fn { ref sig, body, has_body: true, .. } => {
                    self.translate_callable(module_id, def_id, sig.span, body, &mut parts);
                }
                ItemKind::Trait { .. } => {
                    let tcx = self.tcx;
                    for assoc_item in tcx.associated_items(def_id).in_definition_order() {
                        if assoc_item.defaultness(tcx).has_value()
                            && !assoc_item.is_impl_trait_in_trait()
                        {
                            self.translate_assoc_item(module_id, assoc_item, &mut parts);
                        }
                    }
                }
                _ => {}
            }
        }

        parts
    }

    /// Translates an associated item that has a value into an Erlang function.
    fn translate_assoc_item(
        &self,
        module_id: ErlModuleId,
        item: &ty::AssocItem,
        parts: &mut ModuleParts<'arena>,
    ) {
        let tcx = self.tcx;
        let def_id = item.def_id.expect_local();

        // A method's function starts at its signature, any other item's at its definition.
        let span = match tcx.hir_node_by_def_id(def_id).fn_sig() {
            Some(sig) => sig.span,
            None => tcx.def_span(def_id),
        };

        match item.kind {
            _ if item.is_type_const() => {
                not_implemented!(tcx, span, "associated type const");
            }
            ty::AssocKind::Fn { .. } | ty::AssocKind::Const { .. } => {
                let body = tcx.hir_body_owned_by(def_id).id();
                self.translate_callable(module_id, def_id, span, body, parts);
            }
            ty::AssocKind::Type { .. } if item.trait_container(tcx).is_some() => {
                not_implemented!(tcx, span, "associated type default");
            }
            ty::AssocKind::Type { .. } => self.translate_assoc_type_witness(def_id, span, parts),
        }
    }

    fn translate_struct(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let item = self.tcx.hir_expect_item(def_id);
        let ItemKind::Struct(_ident, _generics, variant_data) = item.kind else {
            unreachable!("def_kind Struct must be an ItemKind::Struct");
        };

        let mod_name = self.tcx.erl_module_name(ErlModuleId::Adt(def_id.to_def_id()));
        let repr = self.tcx.erl_variant_repr(def_id.to_def_id());
        if let Some((_, span)) = self.tcx.find_erl_record_module_attr(def_id.to_def_id()) {
            not_implemented!(self.tcx, span, "`record_module` attribute", repr.kind)
        }
        let parts = self.collect_struct_fns(item.span, mod_name, def_id, variant_data, repr);
        let record_exports =
            (repr.kind == ErlReprKind::NativeRecord).then_some(repr.tag).into_iter();
        self.assemble_module(item.span, mod_name, record_exports, parts)
    }

    fn translate_enum(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let item = self.tcx.hir_expect_item(def_id);

        let ItemKind::Enum(_ident, _generics, enum_def) = item.kind else {
            unreachable!("def_kind Enum must be an ItemKind::Enum");
        };

        let mod_name = self.tcx.erl_module_name(ErlModuleId::Adt(def_id.to_def_id()));

        let mut parts = self.module_parts(item.span);
        let mut record_exports = SmallVec::<[Symbol; 8]>::new();

        for variant in enum_def.variants {
            let repr = self.tcx.erl_variant_repr(variant.def_id.to_def_id());
            match repr.kind {
                // fall through to the loop impl
                ErlReprKind::NativeRecord => (),
                // These variants don't implement anything in the enum's module.
                ErlReprKind::Atom | ErlReprKind::Tagged | ErlReprKind::Transparent { .. } => {
                    continue;
                }
                ErlReprKind::Int => {
                    not_implemented!(self.tcx, variant.span, "int-represented enum variant")
                }
                ErlReprKind::Elixir => {
                    span_bug!(variant.span, "Elixir repr attribute on enum variant")
                }
            }

            if let Some((_, span)) =
                self.tcx.find_erl_record_module_attr(variant.def_id.to_def_id())
            {
                not_implemented!(self.tcx, span, "`record_module` attribute", repr.kind)
            }

            let variant_fields = variant
                .data
                .fields()
                .iter()
                .map(|field| self.afb.record_field_decl(field.span, field.ident, None));

            parts.forms.push(self.afb.native_record_attr(item.span, repr.tag, variant_fields));
            record_exports.push(repr.tag);
        }

        self.collect_impl_fns(ErlModuleId::Adt(def_id.to_def_id()), &mut parts);

        self.assemble_module(item.span, mod_name, record_exports, parts)
    }

    fn collect_struct_fns(
        &self,
        span: Span,
        mod_name: Symbol,
        def_id: rustc_hir::def_id::LocalDefId,
        variant_data: VariantData<'tcx>,
        repr: ErlVariantRepr,
    ) -> ModuleParts<'arena> {
        let mut parts = self.module_parts(span);

        let struct_info = (repr.kind == ErlReprKind::Elixir).then_some(variant_data);
        parts.forms.push(elixir_compat::info_function(&self.afb, span, struct_info));
        parts.exports.push((sym::__info__, 1));

        match repr.kind {
            ErlReprKind::Elixir => {
                parts.forms.push(elixir_compat::struct0_function(
                    &self.afb,
                    span,
                    mod_name,
                    variant_data,
                ));
                parts.exports.push((sym::__struct__, 0));

                parts.forms.push(elixir_compat::struct1_function(&self.afb, span));
                parts.exports.push((sym::__struct__, 1));
            }

            ErlReprKind::NativeRecord => {
                let fields = variant_data
                    .fields()
                    .iter()
                    .map(|field| self.afb.record_field_decl(field.span, field.ident, None));

                parts.forms.push(self.afb.native_record_attr(span, repr.tag, fields));
            }

            // These variants don't implement anything in the struct's module.
            ErlReprKind::Tagged | ErlReprKind::Atom | ErlReprKind::Transparent { .. } => {}
            ErlReprKind::Int => not_implemented!(self.tcx, span, "int-represented struct"),
        }

        self.collect_impl_fns(ErlModuleId::Adt(def_id.to_def_id()), &mut parts);

        parts
    }

    fn collect_impl_fns(&self, module_id: ErlModuleId, parts: &mut ModuleParts<'arena>) {
        let Some(impls) = self.impls.impls_for(module_id) else { return };

        let mut seen_traits = FxHashSet::default();
        for (impl_def_id, imp) in impls {
            let trait_def_id = self.tcx.impl_opt_trait_id(*impl_def_id);
            if let Some(trait_def_id) = trait_def_id
                && !seen_traits.insert(trait_def_id)
            {
                not_implemented!(
                    self.tcx,
                    imp.self_ty.span,
                    "multiple impls of one trait for one type"
                );
            }

            // Inherent consts and types don't become functions.
            for item in self.tcx.associated_items(*impl_def_id).in_definition_order() {
                if (trait_def_id.is_some() || item.is_fn()) && !item.is_impl_trait_in_trait() {
                    self.translate_assoc_item(module_id, item, parts);
                }
            }

            if let Some(trait_def_id) = trait_def_id {
                self.emit_default_stubs(
                    module_id,
                    *impl_def_id,
                    trait_def_id,
                    imp.self_ty.span,
                    parts,
                );
            }
        }
    }

    /// Emits the function that returns the witness of an associated type in a trait impl.
    /// For example: `'Crate::Trait::type::Item'(SelfWitness) -> {'Elixir.Patina.Crate.Foo', {}}`.
    fn translate_assoc_type_witness(
        &self,
        def_id: LocalDefId,
        span: Span,
        parts: &mut ModuleParts<'arena>,
    ) {
        let tcx = self.tcx;
        if !tcx.generics_of(def_id).own_params.is_empty() {
            not_implemented!(tcx, span, "generic associated type");
        }

        let value_ty = tcx.type_of(def_id).instantiate_identity().skip_normalization();
        let value_params = value_ty.walk().filter_map(|arg| match arg.as_type()?.kind() {
            ty::Param(param) => Some(*param),
            _ => None,
        });

        let mut scopes = Scopes::new();
        let (params, witness_vars) =
            self.translate_witness_params(def_id, span, value_params, &mut scopes);

        let witness = self.build_witness(def_id.to_def_id(), span, value_ty);
        let value = self.materialize_witness(span, witness, &witness_vars, &mut scopes);

        let name = tcx.erl_fn_name(def_id);
        let arity = params.len() as u32;

        self.push_function(parts, span, self.afb.simple_function(span, name, params, value));
        parts.exports.push((name, arity));
    }

    /// Emits a delegation stub for each default body (method or const) that the trait impl does
    /// not override: `Name(V1, ..., Vn) -> TraitMod:Name(V1, ..., Vn)`. With the stubs, the
    /// impl's module provides every function of the trait.
    fn emit_default_stubs(
        &self,
        module_id: ErlModuleId,
        impl_def_id: DefId,
        trait_def_id: DefId,
        span: Span,
        parts: &mut ModuleParts<'arena>,
    ) {
        let tcx = self.tcx;
        let afb = &self.afb;
        let implemented = tcx.impl_item_implementor_ids(impl_def_id);

        for trait_item in tcx.associated_items(trait_def_id).in_definition_order() {
            if !trait_item.defaultness(tcx).has_value()
                || trait_item.is_impl_trait_in_trait()
                || implemented.contains_key(&trait_item.def_id)
            {
                continue;
            }

            match trait_item.kind {
                ty::AssocKind::Fn { .. } | ty::AssocKind::Const { .. } => {}
                ty::AssocKind::Type { .. } => {
                    not_implemented!(tcx, span, "associated type default", trait_item.def_id)
                }
            }

            let name = tcx.erl_fn_name(trait_item.def_id);
            let arity = tcx.erl_fn_arity(trait_item.def_id);
            let default_module_id =
                tcx.erl_definition_module(trait_item.def_id).unwrap_or_else(|| {
                    span_bug!(span, "default body `{name}` has no definition module")
                });
            let default_module = tcx.erl_module_name(default_module_id);

            let mut scopes = Scopes::new();
            let vars = (0..arity)
                .map(|_| scopes.fresh_var(sym::arg).erl_name)
                .collect::<SmallVec<[_; 8]>>();

            let body = afb.expr_remote_call(
                span,
                afb.expr_atom(span, default_module),
                afb.expr_atom(span, name),
                vars.iter().map(|&var| afb.expr_var(span, var)),
            );
            let params = vars.iter().map(|&var| afb.pat_var(span, var));

            self.push_function(parts, span, afb.simple_function(span, name, params, body));

            if tcx.is_erl_exported_fn(module_id, trait_item.def_id) {
                parts.exports.push((name, arity as u32));
            }
        }
    }

    fn translate_prim_module(&self, module_id: ErlModuleId) -> (Symbol, eaf::Module<'arena>) {
        let mod_name = self.tcx.erl_module_name(module_id);

        let impls = self
            .impls
            .impls_for(module_id)
            .unwrap_or_else(|| bug!("primitive module has no impls"));

        let span = impls[0].1.self_ty.span;

        let mut parts = self.module_parts(span);
        parts.forms.push(elixir_compat::info_function(&self.afb, span, None));
        parts.exports.push((sym::__info__, 1));

        self.collect_impl_fns(module_id, &mut parts);
        self.assemble_module(span, mod_name, [], parts)
    }
}
