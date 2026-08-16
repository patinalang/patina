mod adts;
mod body;
mod elixir_compat;
mod loops;
mod scopes;

use std::cell::RefCell;
use std::path::PathBuf;

use body::{BodyCx, TranslatedBody};
use rustc_arena::DroplessArena;
use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{LocalDefId, LocalModDefId};
use rustc_hir::{BodyId, FnSig, Generics, ImplItemKind, ItemKind, Mod, VariantData};
use rustc_middle::bug;
use rustc_middle::erl::ErlModuleId;
use rustc_middle::ty::TyCtxt;
use rustc_span::{Ident, Span, Symbol, sym};
use scopes::Scopes;

use crate::compile::eaf_builder::EafBuilder;
use crate::compile::impl_collector::ImplCollector;
use crate::compile::module_collector::ModuleNameCollector;
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

pub(super) struct Translator<'tcx, 'arena> {
    tcx: TyCtxt<'tcx>,
    afb: EafBuilder<'tcx, 'arena>,
    modules: ModuleNameCollector<'tcx>,
    impls: ImplCollector<'tcx>,
    translated_consts: RefCell<FxHashMap<LocalDefId, TranslatedConst<'arena>>>,
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    pub fn new(tcx: TyCtxt<'tcx>, arena: &'arena DroplessArena) -> Translator<'tcx, 'arena> {
        Translator {
            tcx,
            afb: EafBuilder::new(tcx.sess.source_map(), arena),
            modules: ModuleNameCollector::new(tcx),
            impls: ImplCollector::new(tcx),
            translated_consts: RefCell::new(FxHashMap::default()),
        }
    }

    pub(super) fn translate_body(
        &self,
        owner: LocalDefId,
        parts: &mut ModuleParts<'arena>,
        scopes: &mut Scopes,
    ) -> TranslatedBody<'arena> {
        let (thir, root) =
            self.tcx.thir_body(owner).unwrap_or_else(|guaranteed| guaranteed.raise_fatal());

        let thir = thir.borrow();
        let mut cx = BodyCx::new(self, owner, &thir, parts, scopes);

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
        let value = self.translate_body(def_id, parts, &mut Scopes::new()).value;
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

        modules
    }

    fn translate_module(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let module_id = LocalModDefId::new_unchecked(def_id);
        let (module, _span, _hir_id) = self.tcx.hir_get_module(module_id);
        let parts = self.collect_module_fns(module);
        let mod_name = self.modules.module_name(ErlModuleId::Mod(def_id.to_def_id()));
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

        let attrs = [file_attr, module_attr, compile_attr, record_export_attr, export_attr];
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

    fn translate_fun(
        &self,
        def_id: LocalDefId,
        sig: &FnSig<'tcx>,
        generics: &Generics<'tcx>,
        ident: Ident,
        body: BodyId,
        parts: &mut ModuleParts<'arena>,
    ) {
        if !generics.params.is_empty() {
            not_implemented!(self.tcx, generics.span, "function with generic parameters", generics);
        }

        let owner = self.tcx.hir_body_owner_def_id(body);
        let mut scope = Scopes::new();
        let translated = self.translate_body(owner, parts, &mut scope);

        let form =
            self.afb.simple_function(sig.span, ident.name, translated.params, translated.value);

        self.push_function(parts, sig.span, form);
        if self.tcx.is_erl_exported_fn(def_id) {
            parts.exports.push((ident.name, sig.decl.inputs.len() as u32));
        }
    }

    fn collect_module_fns(&self, module: &Mod<'tcx>) -> ModuleParts<'arena> {
        let mut parts = self.module_parts(module.spans.inner_span);

        parts.forms.push(elixir_compat::info_function(&self.afb, module.spans.inner_span, None));
        parts.exports.push((sym::__info__, 1));

        for item_id in module.item_ids {
            let item = self.tcx.hir_item(*item_id);
            if let ItemKind::Fn { ref sig, ident, generics, body, has_body } = item.kind
                && has_body
            {
                self.translate_fun(item.owner_id.def_id, sig, generics, ident, body, &mut parts);
            }
        }

        parts
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
        let parts = self.collect_struct_fns(item.span, mod_name, def_id, variant_data);
        self.assemble_module(item.span, mod_name, [], parts)
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

        let mut parts = self.module_parts(item.span);
        let mut record_exports = SmallVec::<[Symbol; 8]>::new();

        for variant in enum_def.variants {
            let variant_name = variant.ident.name;
            let variant_fields = variant
                .data
                .fields()
                .iter()
                .map(|field| self.afb.record_field_decl(field.span, field.ident, None));

            parts.forms.push(self.afb.native_record_attr(item.span, variant_name, variant_fields));
            record_exports.push(variant_name);
        }

        self.collect_impl_fns(def_id, &mut parts);

        self.assemble_module(item.span, mod_name, record_exports, parts)
    }

    fn collect_struct_fns(
        &self,
        span: Span,
        mod_name: Symbol,
        def_id: rustc_hir::def_id::LocalDefId,
        variant_data: VariantData<'tcx>,
    ) -> ModuleParts<'arena> {
        let mut parts = self.module_parts(span);

        parts.forms.push(elixir_compat::info_function(&self.afb, span, Some(variant_data)));
        parts.exports.push((sym::__info__, 1));

        parts.forms.push(elixir_compat::struct0_function(&self.afb, span, mod_name, variant_data));
        parts.exports.push((sym::__struct__, 0));

        parts.forms.push(elixir_compat::struct1_function(&self.afb, span));
        parts.exports.push((sym::__struct__, 1));

        self.collect_impl_fns(def_id, &mut parts);

        parts
    }

    fn collect_impl_fns(
        &self,
        def_id: rustc_hir::def_id::LocalDefId,
        parts: &mut ModuleParts<'arena>,
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

                    self.translate_fun(
                        impl_item.owner_id.def_id,
                        &sig,
                        impl_item.generics,
                        impl_item.ident,
                        body,
                        parts,
                    );
                }
            }
        }
    }
}
