mod adts;
mod binary_pattern;
mod body;
mod elixir_compat;
mod loops;
mod scopes;
mod witness;

use std::cell::RefCell;
use std::path::PathBuf;

use body::{BodyCx, TranslatedBody};
use rustc_arena::DroplessArena;
use rustc_data_structures::fx::{FxHashMap, FxIndexMap};
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LocalDefId, LocalModDefId};
use rustc_hir::{BodyId, FieldDef, ItemKind, Mod};
use rustc_middle::erl::witness::WitnessResolver;
use rustc_middle::erl::{ErlModuleId, ErlReprKind, ErlVariantRepr};
use rustc_middle::ty::{self, TyCtxt};
use rustc_middle::{bug, span_bug};
use rustc_span::{Span, Symbol, kw, sym};
use scopes::Scopes;
use witness::WitnessEnv;

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

struct FnClause<'arena> {
    span: Span,
    name: Symbol,
    arity: u32,
    clause: eaf::Clause<'arena>,
    exported: bool,
    implements_trait_item: Option<DefId>,
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
        witness_env: &WitnessEnv<'arena>,
    ) -> TranslatedBody<'arena> {
        let owner = owner.to_def_id();
        let etir = self.tcx.etir_body(owner);
        let mut cx = BodyCx::new(self, owner, etir, parts, scopes, witness_env);
        cx.translate_body_with_params(&etir.params, etir.value)
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
            self.translate_body(def_id, parts, &mut Scopes::new(), &WitnessEnv::default()).value;
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

        for module_id in self.impls.impl_only_modules() {
            modules.push(self.translate_impl_only_module(*module_id));
        }

        modules
    }

    fn translate_module(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let module_id = LocalModDefId::new_unchecked(def_id);
        let (module, _span, _hir_id) = self.tcx.hir_get_module(module_id);
        let parts = self.collect_module_fns(module_id, module);
        let mod_name = self.tcx.erl_module_name(ErlModuleId::Mod(def_id.to_def_id()));
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

    fn translate_callable_clause(
        &self,
        module_id: ErlModuleId,
        def_id: LocalDefId,
        span: Span,
        body: BodyId,
        parts: &mut ModuleParts<'arena>,
    ) -> FnClause<'arena> {
        let owner = self.tcx.hir_body_owner_def_id(body);
        let mut scopes = Scopes::new();

        let witness_params = self.tcx.erl_witness_params_identity(owner.to_def_id());
        let witness_env = self.translate_witness_params(&witness_params, span, &mut scopes);

        let translated = self.translate_body(owner, parts, &mut scopes, &witness_env);
        let params: SmallVec<[_; 8]> =
            witness_env.params.iter().map(|param| param.pat).chain(translated.params).collect();

        FnClause {
            span,
            name: self.tcx.erl_fn_name(def_id),
            arity: params.len() as u32,
            clause: self.afb.clause(span, params, None, translated.value),
            exported: self.tcx.is_erl_exported_fn(module_id, def_id.to_def_id()),
            implements_trait_item: self.tcx.trait_item_of(def_id.to_def_id()),
        }
    }

    fn push_functions(
        &self,
        parts: &mut ModuleParts<'arena>,
        clauses: impl IntoIterator<Item = FnClause<'arena>>,
    ) {
        struct Function<'arena> {
            span: Span,
            name: Symbol,
            arity: u32,
            clauses: SmallVec<[eaf::Clause<'arena>; 1]>,
            exported: bool,
        }

        #[derive(PartialEq, Eq, Hash)]
        enum Key {
            TraitItem(DefId),
            Unique(usize),
        }

        let mut functions = FxIndexMap::<Key, Function<'arena>>::default();
        for (index, clause) in clauses.into_iter().enumerate() {
            let key = match clause.implements_trait_item {
                Some(trait_item) => Key::TraitItem(trait_item),
                None => Key::Unique(index),
            };
            let function = functions.entry(key).or_insert_with(|| Function {
                span: clause.span,
                name: clause.name,
                arity: clause.arity,
                clauses: SmallVec::new(),
                exported: false,
            });
            debug_assert_eq!((function.name, function.arity), (clause.name, clause.arity));
            function.clauses.push(clause.clause);
            function.exported |= clause.exported;
        }

        for function in functions.into_values() {
            let Function { span, name, arity, clauses, exported } = function;
            self.push_function(parts, span, self.afb.function(span, name, arity, clauses));
            if exported {
                parts.exports.push((name, arity));
            }
        }
    }

    fn collect_module_fns(
        &self,
        local_module_id: LocalModDefId,
        module: &Mod<'tcx>,
    ) -> ModuleParts<'arena> {
        let module_id = ErlModuleId::Mod(local_module_id.to_def_id());
        let mut parts = self.module_parts(module.spans.inner_span);

        parts.forms.push(elixir_compat::info_function(&self.afb, module.spans.inner_span, None));
        parts.exports.push((sym::__info__, 1));

        for item_id in self.tcx.hir_module_free_items(local_module_id) {
            let item = self.tcx.hir_item(item_id);
            let def_id = item.owner_id.def_id;
            match item.kind {
                ItemKind::Fn { ref sig, body, has_body: true, .. } => {
                    let clause = self
                        .translate_callable_clause(module_id, def_id, sig.span, body, &mut parts);
                    self.push_functions(&mut parts, [clause]);
                }
                ItemKind::Trait { .. } => {
                    let tcx = self.tcx;
                    for assoc_item in tcx.associated_items(def_id).in_definition_order() {
                        if assoc_item.defaultness(tcx).has_value()
                            && !assoc_item.is_impl_trait_in_trait()
                        {
                            let clause =
                                self.translate_assoc_item(module_id, assoc_item, &mut parts);
                            self.push_functions(&mut parts, [clause]);
                        }
                    }
                }
                _ => {}
            }
        }

        parts
    }

    /// Translates an associated item that has a value into a clause of its Erlang function.
    fn translate_assoc_item(
        &self,
        module_id: ErlModuleId,
        item: &ty::AssocItem,
        parts: &mut ModuleParts<'arena>,
    ) -> FnClause<'arena> {
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
                self.translate_callable_clause(module_id, def_id, span, body, parts)
            }
            ty::AssocKind::Type { .. } if item.trait_container(tcx).is_some() => {
                not_implemented!(tcx, span, "associated type default");
            }
            ty::AssocKind::Type { .. } => self.translate_assoc_type_witness(def_id, span),
        }
    }

    fn translate_struct(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let &rustc_hir::Item { span, kind: ItemKind::Struct(..), .. } =
            self.tcx.hir_expect_item(def_id)
        else {
            unreachable!("def_kind Struct must be an ItemKind::Struct");
        };

        let mod_name = self.tcx.erl_module_name(ErlModuleId::Adt(def_id.to_def_id()));
        let repr = self.tcx.erl_variant_repr(def_id.to_def_id());
        if let Some((_, span)) = self.tcx.find_erl_record_module_attr(def_id.to_def_id()) {
            not_implemented!(self.tcx, span, "`record_module` attribute", repr.kind)
        }
        let parts = self.collect_struct_fns(span, mod_name, def_id, repr);
        let record_exports =
            (repr.kind == ErlReprKind::NativeRecord).then_some(repr.tag).into_iter();
        self.assemble_module(span, mod_name, record_exports, parts)
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

            let variant_def = self.tcx.adt_def(def_id).variant_with_id(variant.def_id.to_def_id());
            let fields = self.erl_runtime_hir_fields(variant_def);
            let field_decls = fields
                .iter()
                .map(|field| self.afb.record_field_decl(field.span, field.ident, None));

            parts.forms.push(self.afb.native_record_attr(item.span, repr.tag, field_decls));
            record_exports.push(repr.tag);
            self.push_record_constructor(&mut parts, variant.span, repr.tag, &fields);
        }

        self.collect_impl_fns(ErlModuleId::Adt(def_id.to_def_id()), &mut parts);

        self.assemble_module(item.span, mod_name, record_exports, parts)
    }

    fn collect_struct_fns(
        &self,
        span: Span,
        mod_name: Symbol,
        def_id: rustc_hir::def_id::LocalDefId,
        repr: ErlVariantRepr,
    ) -> ModuleParts<'arena> {
        let mut parts = self.module_parts(span);

        let fields = self.erl_runtime_hir_fields(self.tcx.adt_def(def_id).non_enum_variant());
        let struct_info = (repr.kind == ErlReprKind::Elixir).then_some(&fields[..]);
        parts.forms.push(elixir_compat::info_function(&self.afb, span, struct_info));
        parts.exports.push((sym::__info__, 1));

        match repr.kind {
            ErlReprKind::Elixir => {
                parts
                    .forms
                    .push(elixir_compat::struct0_function(&self.afb, span, mod_name, &fields));
                parts.exports.push((sym::__struct__, 0));

                parts.forms.push(elixir_compat::struct1_function(&self.afb, span));
                parts.exports.push((sym::__struct__, 1));
            }

            ErlReprKind::NativeRecord => {
                let field_decls = fields
                    .iter()
                    .map(|field| self.afb.record_field_decl(field.span, field.ident, None));

                parts.forms.push(self.afb.native_record_attr(span, repr.tag, field_decls));
                self.push_record_constructor(&mut parts, span, repr.tag, &fields);
            }

            // These variants don't implement anything in the struct's module.
            ErlReprKind::Tagged | ErlReprKind::Atom | ErlReprKind::Transparent { .. } => {}
            ErlReprKind::Int => not_implemented!(self.tcx, span, "int-represented struct"),
        }

        self.collect_impl_fns(ErlModuleId::Adt(def_id.to_def_id()), &mut parts);

        parts
    }

    fn erl_runtime_hir_fields(
        &self,
        variant: &ty::VariantDef,
    ) -> SmallVec<[&'tcx FieldDef<'tcx>; 8]> {
        self.tcx
            .erl_runtime_fields(variant)
            .map(|(_, field)| self.tcx.hir_node_by_def_id(field.did.expect_local()).expect_field())
            .collect()
    }

    /// Emits the exported constructor of a native record declared in this module:
    /// `'$patina_constructor__Tag'(F_1, ..., F_k) -> #'Tag'{f_1 = F_1, ..., f_k = F_k}`.
    fn push_record_constructor(
        &self,
        parts: &mut ModuleParts<'arena>,
        span: Span,
        tag: Symbol,
        fields: &[&'tcx FieldDef<'tcx>],
    ) {
        let mut scopes = Scopes::new();

        let vars = fields
            .iter()
            .map(|&field| (field, scopes.fresh_var(field.ident.name).erl_name))
            .collect::<SmallVec<[_; 4]>>();

        let params = vars.iter().map(|&(field, var)| self.afb.pat_var(field.span, var));
        let record = self.afb.expr_record_create(
            span,
            None,
            tag,
            vars.iter().map(|&(field, var)| (field.ident, self.afb.expr_var(field.span, var))),
        );

        let name = record_constructor_name(tag);
        let arity = vars.len() as u32;
        let clause = self.afb.clause(span, params, None, record);
        self.push_function(parts, span, self.afb.function(span, name, arity, [clause]));
        parts.exports.push((name, arity));
    }

    fn collect_impl_fns(&self, module_id: ErlModuleId, parts: &mut ModuleParts<'arena>) {
        let Some(impls) = self.impls.impls_for(module_id) else { return };
        let tcx = self.tcx;

        let mut clauses = Vec::new();
        for (impl_def_id, imp) in impls {
            let trait_def_id = tcx.impl_opt_trait_id(*impl_def_id);

            // Inherent consts and types don't become functions, and neither do intrinsics.
            for item in tcx.associated_items(*impl_def_id).in_definition_order() {
                if (trait_def_id.is_some() || item.is_fn())
                    && !item.is_impl_trait_in_trait()
                    && tcx.intrinsic(item.def_id).is_none()
                {
                    clauses.push(self.translate_assoc_item(module_id, item, parts));
                }
            }

            if let Some(trait_def_id) = trait_def_id {
                self.emit_default_stubs(
                    module_id,
                    *impl_def_id,
                    trait_def_id,
                    imp.self_ty.span,
                    &mut clauses,
                );
            }
        }

        self.push_functions(parts, clauses);
    }

    /// The clause that returns the witnesses of the declared bounds of an associated type.
    fn translate_assoc_type_witness(&self, def_id: LocalDefId, span: Span) -> FnClause<'arena> {
        let tcx = self.tcx;
        if !tcx.generics_of(def_id).own_params.is_empty() {
            not_implemented!(tcx, span, "generic associated type");
        }

        let owner = def_id.to_def_id();
        let trait_assoc_def_id = tcx
            .trait_item_of(owner)
            .unwrap_or_else(|| span_bug!(span, "associated type witness outside a trait impl"));
        let args = tcx.erl_trait_item_args_for_impl_item(owner);

        let mut scopes = Scopes::new();
        let witness_env = self.translate_witness_params(
            &tcx.erl_witness_params_identity(owner),
            span,
            &mut scopes,
        );
        let params: SmallVec<[_; 2]> = witness_env.params.iter().map(|param| param.pat).collect();
        let mut resolver = WitnessResolver::new(tcx, owner);

        let witnesses = tcx
            .erl_assoc_type_bounds(trait_assoc_def_id)
            .iter_instantiated_copied(tcx, args)
            .map(|bound| {
                let source = resolver.witness_source(span, bound);
                self.translate_witness(span, &source, &witness_env, &mut scopes)
            })
            .collect::<SmallVec<[_; 4]>>();
        let value = self.afb.expr_tuple(span, witnesses);

        FnClause {
            span,
            name: tcx.erl_fn_name(def_id),
            arity: params.len() as u32,
            clause: self.afb.clause(span, params, None, value),
            exported: true,
            implements_trait_item: Some(trait_assoc_def_id),
        }
    }

    /// Emits a delegation stub clause for each default body (method or const) that the trait impl
    /// does not override: `Name({Tag, _, …} = W, V1, …, Vn) -> TraitMod:Name(W, V1, …, Vn)`. With
    /// the stubs, the host module provides every function of the trait.
    fn emit_default_stubs(
        &self,
        module_id: ErlModuleId,
        impl_def_id: DefId,
        trait_def_id: DefId,
        span: Span,
        clauses: &mut Vec<FnClause<'arena>>,
    ) {
        let tcx = self.tcx;
        let afb = &self.afb;
        let implemented = tcx.impl_item_implementor_ids(impl_def_id);
        let witness_element_count = self.impl_witness_element_count(impl_def_id);

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
            let witness = scopes.fresh_var(sym::witness).erl_name;
            let vars = (1..arity)
                .map(|_| scopes.fresh_var(sym::arg).erl_name)
                .collect::<SmallVec<[_; 8]>>();

            let witness_pat = self.impl_witness_pat(
                impl_def_id,
                span,
                (0..witness_element_count).map(|_| afb.pat_var(span, kw::Underscore)),
            );
            let params = [afb.pat_match(span, afb.pat_var(span, witness), witness_pat)]
                .into_iter()
                .chain(vars.iter().map(|&var| afb.pat_var(span, var)));

            let body = afb.expr_remote_call(
                span,
                afb.expr_atom(span, default_module),
                afb.expr_atom(span, name),
                [witness].iter().chain(&vars).map(|&var| afb.expr_var(span, var)),
            );

            clauses.push(FnClause {
                span,
                name,
                arity: arity as u32,
                clause: afb.clause(span, params, None, body),
                exported: tcx.is_erl_exported_fn(module_id, trait_item.def_id),
                implements_trait_item: Some(trait_item.def_id),
            });
        }
    }

    fn translate_impl_only_module(&self, module_id: ErlModuleId) -> (Symbol, eaf::Module<'arena>) {
        let mod_name = self.tcx.erl_module_name(module_id);

        let impls = self
            .impls
            .impls_for(module_id)
            .unwrap_or_else(|| bug!("impl-only module has no impls"));

        let span = impls[0].1.self_ty.span;

        let mut parts = self.module_parts(span);
        parts.forms.push(elixir_compat::info_function(&self.afb, span, None));
        parts.exports.push((sym::__info__, 1));

        self.collect_impl_fns(module_id, &mut parts);
        self.assemble_module(span, mod_name, [], parts)
    }
}

fn record_constructor_name(tag: Symbol) -> Symbol {
    Symbol::intern(&format!("$patina_constructor__{tag}"))
}
