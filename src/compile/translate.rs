mod adts;
mod body;
mod elixir_compat;

use std::path::PathBuf;

use rustc_arena::DroplessArena;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{LocalDefId, LocalModDefId};
use rustc_hir::{BodyId, FnSig, Generics, ImplItemKind, ItemKind, Mod, VariantData};
use rustc_middle::bug;
use rustc_middle::erl::ErlModuleId;
use rustc_middle::ty::TyCtxt;
use rustc_span::{Ident, Span, Symbol, sym};

use crate::compile::eaf_builder::EafBuilder;
use crate::compile::impl_collector::ImplCollector;
use crate::compile::module_collector::ModuleNameCollector;
use crate::compile::translate::body::{BodyCx, Scope, TranslatedBody};
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
}

pub(super) struct Translator<'tcx, 'arena> {
    tcx: TyCtxt<'tcx>,
    afb: EafBuilder<'tcx, 'arena>,
    modules: ModuleNameCollector<'tcx>,
    impls: ImplCollector<'tcx>,
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    pub fn new(tcx: TyCtxt<'tcx>, arena: &'arena DroplessArena) -> Translator<'tcx, 'arena> {
        Translator {
            tcx,
            afb: EafBuilder::new(tcx.sess.source_map(), arena),
            modules: ModuleNameCollector::new(tcx),
            impls: ImplCollector::new(tcx),
        }
    }

    pub(super) fn translate_body(
        &self,
        owner: LocalDefId,
        scope: &mut Scope,
    ) -> TranslatedBody<'arena> {
        let (thir, root) =
            self.tcx.thir_body(owner).unwrap_or_else(|guaranteed| guaranteed.raise_fatal());

        let thir = thir.borrow();
        let mut cx = BodyCx { translator: self, owner, thir: &thir, scope };

        let params = thir
            .params
            .iter()
            .filter(|param| param.hir_id.is_some())
            .map(|param| {
                let pat = param
                    .pat
                    .as_deref()
                    .unwrap_or_else(|| bug!("explicit parameter must have a THIR pattern"));
                cx.translate_pattern(pat)
            })
            .collect();

        let value = cx.translate_expr(root);

        TranslatedBody { params, value }
    }

    pub fn translate(&self) -> Vec<(Symbol, eaf::Module<'arena>)> {
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

        modules
    }

    fn translate_module(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let module_id = LocalModDefId::new_unchecked(def_id);
        let (module, _span, _hir_id) = self.tcx.hir_get_module(module_id);
        let (forms, exports) = self.collect_module_fns(module);
        let mod_name = self.modules.module_name(ErlModuleId::Mod(def_id.to_def_id()));
        self.assemble_module(module.spans.inner_span, mod_name, [], exports, forms)
    }

    fn assemble_module(
        &self,
        span: Span,
        mod_name: Symbol,
        record_exports: impl IntoIterator<Item = Symbol>,
        exports: impl IntoIterator<Item = (Symbol, usize)>,
        fn_forms: impl IntoIterator<Item = eaf::ModuleForm<'arena>>,
    ) -> (Symbol, eaf::Module<'arena>) {
        let file = self.file_path(span);
        let file_attr = self.afb.file_attr(span, &file.to_string_lossy());
        let module_attr = self.afb.module_attr(span, mod_name);
        let compile_attr = self.afb.wild_attr(span, sym::compile, [sym::no_auto_import]);
        let record_export_attr = self.afb.export_record_attr(span, record_exports);
        let export_attr = self.afb.export_attr(span, exports);

        let attrs = [file_attr, module_attr, compile_attr, record_export_attr, export_attr];
        (mod_name, self.afb.module(attrs.into_iter().chain(fn_forms)))
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
        forms: &mut Vec<eaf::ModuleForm<'arena>>,
        last_file: &mut Option<PathBuf>,
        span: Span,
        function: eaf::ModuleForm<'arena>,
    ) {
        // If the function is in a different file than the previous, push a `-file` attribute
        // as well.
        let file = self.file_path(span);
        if last_file.as_ref() != Some(&file) {
            forms.push(self.afb.file_attr(span, &file.to_string_lossy()));
            *last_file = Some(file);
        }
        forms.push(function);
    }

    fn translate_fun(
        &self,
        sig: &FnSig<'tcx>,
        generics: &Generics<'tcx>,
        ident: Ident,
        body: BodyId,
    ) -> eaf::ModuleForm<'arena> {
        if !generics.params.is_empty() {
            not_implemented!(self.tcx, generics.span, "function with generic parameters", generics);
        }

        let owner = self.tcx.hir_body_owner_def_id(body);
        let mut scope = Scope::new();
        let translated = self.translate_body(owner, &mut scope);

        let clause_body = self.afb.clause_body(translated.value);
        let clause = self.afb.clause(sig.span, translated.params, None, clause_body);

        let fn_name = ident.name;
        let fn_arity = sig.decl.inputs.len();

        self.afb.function(sig.span, fn_name, fn_arity, [clause])
    }

    fn collect_module_fns(
        &self,
        module: &Mod<'tcx>,
    ) -> (Vec<eaf::ModuleForm<'arena>>, SmallVec<[(Symbol, usize); 8]>) {
        let mut forms = Vec::new();
        let mut last_file = Some(self.file_path(module.spans.inner_span));
        let mut exports = SmallVec::<[(Symbol, usize); 8]>::new();

        forms.push(elixir_compat::info_function(&self.afb, module.spans.inner_span, None));
        exports.push((sym::__info__, 1));

        for item_id in module.item_ids {
            let item = self.tcx.hir_item(*item_id);
            if let ItemKind::Fn { ref sig, ident, generics, body, has_body } = item.kind
                && has_body
            {
                let function = self.translate_fun(sig, generics, ident, body);
                self.push_function(&mut forms, &mut last_file, item.span, function);

                if self.tcx.is_erl_exported_fn(item.owner_id.def_id) {
                    exports.push((ident.name, sig.decl.inputs.len()));
                }
            }
        }

        (forms, exports)
    }

    fn translate_struct(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let item = self.tcx.hir_expect_item(def_id);
        let ItemKind::Struct(_ident, generics, variant_data) = item.kind else {
            unreachable!("def_kind Struct must be an ItemKind::Struct");
        };

        if !generics.params.is_empty() {
            not_implemented!(self.tcx, generics.span, "struct with generic parameters", generics);
        }

        let mod_name = self.modules.module_name(ErlModuleId::Adt(def_id.to_def_id()));
        let (forms, exports) = self.collect_struct_fns(item.span, mod_name, def_id, variant_data);
        self.assemble_module(item.span, mod_name, [], exports, forms)
    }

    fn translate_enum(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let item = self.tcx.hir_expect_item(def_id);

        let ItemKind::Enum(_ident, generics, enum_def) = item.kind else {
            unreachable!("def_kind Enum must be an ItemKind::Enum");
        };

        if !generics.params.is_empty() {
            not_implemented!(self.tcx, generics.span, "enum with generic parameters", generics);
        }

        let mod_name = self.modules.module_name(ErlModuleId::Adt(def_id.to_def_id()));

        let mut forms = Vec::new();
        let mut last_file = Some(self.file_path(item.span));
        let mut exports = SmallVec::<[(Symbol, usize); 8]>::new();
        let mut record_exports = SmallVec::<[Symbol; 8]>::new();

        for variant in enum_def.variants {
            let variant_name = variant.ident.name;
            let variant_fields = variant
                .data
                .fields()
                .iter()
                .map(|field| self.afb.record_field_decl(field.span, field.ident, None));

            forms.push(self.afb.native_record_attr(item.span, variant_name, variant_fields));
            record_exports.push(variant_name);
        }

        self.collect_impl_fns(def_id, &mut forms, &mut last_file, &mut exports);

        self.assemble_module(item.span, mod_name, record_exports, exports, forms)
    }

    fn collect_struct_fns(
        &self,
        span: Span,
        mod_name: Symbol,
        def_id: rustc_hir::def_id::LocalDefId,
        variant_data: VariantData<'tcx>,
    ) -> (Vec<eaf::ModuleForm<'arena>>, SmallVec<[(Symbol, usize); 8]>) {
        let mut forms = Vec::new();
        let mut last_file = Some(self.file_path(span));
        let mut exports = SmallVec::<[(Symbol, usize); 8]>::new();

        forms.push(elixir_compat::info_function(&self.afb, span, Some(variant_data)));
        exports.push((sym::__info__, 1));

        forms.push(elixir_compat::struct0_function(&self.afb, span, mod_name, variant_data));
        exports.push((sym::__struct__, 0));

        forms.push(elixir_compat::struct1_function(&self.afb, span));
        exports.push((sym::__struct__, 1));

        self.collect_impl_fns(def_id, &mut forms, &mut last_file, &mut exports);

        (forms, exports)
    }

    fn collect_impl_fns(
        &self,
        def_id: rustc_hir::def_id::LocalDefId,
        forms: &mut Vec<eaf::ModuleForm<'arena>>,
        last_file: &mut Option<PathBuf>,
        exports: &mut SmallVec<[(Symbol, usize); 8]>,
    ) {
        if let Some(impls) = self.impls.impls_for(def_id.to_def_id()) {
            for imp in impls {
                if !imp.generics.params.is_empty() {
                    not_implemented!(
                        self.tcx,
                        imp.generics.span,
                        "impl with generic parameters",
                        imp.generics
                    );
                }
                for impl_item_id in imp.items {
                    let impl_item = self.tcx.hir_impl_item(*impl_item_id);
                    let ImplItemKind::Fn(sig, body) = impl_item.kind else {
                        continue;
                    };

                    let function =
                        self.translate_fun(&sig, impl_item.generics, impl_item.ident, body);
                    self.push_function(forms, last_file, impl_item.span, function);
                    if self.tcx.is_erl_exported_fn(impl_item.owner_id.def_id) {
                        exports.push((impl_item.ident.name, sig.decl.inputs.len()));
                    }
                }
            }
        }
    }
}
