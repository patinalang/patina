mod adts;
mod binary_pattern;
mod body;
mod elixir_compat;
mod loops;
mod scopes;

use std::path::PathBuf;

use body::{BodyCx, TranslatedBody};
use rustc_arena::DroplessArena;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{LocalDefId, LocalModDefId};
use rustc_hir::{FieldDef, ItemKind, Mod};
use rustc_middle::erl::{ErlImplFn, ErlModuleId, ErlReprKind, ErlVariantRepr};
use rustc_middle::ty::{self, TyCtxt};
use rustc_middle::{bug, span_bug};
use rustc_span::{Span, Symbol, sym};
use scopes::Scopes;

use crate::compile::eaf_builder::EafBuilder;
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

struct Function<'arena> {
    span: Span,
    name: Symbol,
    clause: eaf::Clause<'arena>,
    exported: bool,
}

pub(super) struct Translator<'tcx, 'arena> {
    tcx: TyCtxt<'tcx>,
    afb: EafBuilder<'tcx, 'arena>,
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    pub fn new(tcx: TyCtxt<'tcx>, arena: &'arena DroplessArena) -> Translator<'tcx, 'arena> {
        Translator { tcx, afb: EafBuilder::new(tcx.sess.source_map(), arena) }
    }

    fn translate_body(&self, owner: LocalDefId, scopes: &mut Scopes) -> TranslatedBody<'arena> {
        let etir = self.tcx.etir_body(owner);
        let mut cx = BodyCx::new(self, owner.to_def_id(), etir, scopes);
        cx.translate_body_with_params(&etir.params, etir.value)
    }

    pub fn translate(&self) -> Vec<(Symbol, eaf::Module<'arena>)> {
        self.tcx
            .erl_local_modules(())
            .keys()
            .map(|&module_id| match module_id {
                ErlModuleId::Mod(def_id) => self.translate_module(def_id.expect_local()),

                ErlModuleId::Adt(def_id) => match self.tcx.def_kind(def_id) {
                    DefKind::Struct => self.translate_struct(def_id.expect_local()),
                    DefKind::Enum => self.translate_enum(def_id.expect_local()),
                    kind => bug!("Erlang module for a {kind:?}"),
                },

                ErlModuleId::Prim(_)
                | ErlModuleId::AdtImplHost(..)
                | ErlModuleId::PrimImplHost(..)
                | ErlModuleId::BlanketImplHost(_) => self.translate_impl_only_module(module_id),

                ErlModuleId::Extern(_) => bug!("local Erlang module {module_id:?} is extern"),
            })
            .collect()
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

    fn push_function(&self, parts: &mut ModuleParts<'arena>, function: Function<'arena>) {
        let Function { span, name, clause, exported } = function;
        let arity = clause.patterns.len() as u32;

        // If the function is in a different file than the previous, push a `-file` attribute
        // as well.
        let file = self.file_path(span);
        if parts.last_file.as_ref() != Some(&file) {
            parts.forms.push(self.afb.file_attr(span, &file.to_string_lossy()));
            parts.last_file = Some(file);
        }
        parts.forms.push(self.afb.function(span, name, arity, [clause]));
        if exported {
            parts.exports.push((name, arity));
        }
    }

    fn translate_callable(
        &self,
        owner: LocalDefId,
        name: Symbol,
        span: Span,
        exported: bool,
    ) -> Function<'arena> {
        let translated = self.translate_body(owner, &mut Scopes::new());
        let clause = self.afb.clause(span, translated.params, None, translated.value);

        Function { span, name, clause, exported }
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
                ItemKind::Fn { ref sig, has_body: true, .. } => {
                    let function = self.translate_callable(
                        def_id,
                        self.tcx.erl_fn_name(def_id),
                        sig.span,
                        self.tcx.is_erl_exported_fn(module_id, def_id.to_def_id()),
                    );
                    self.push_function(&mut parts, function);
                }
                ItemKind::Trait { .. } => {
                    let tcx = self.tcx;
                    for assoc_item in tcx.associated_items(def_id).in_definition_order() {
                        if assoc_item.defaultness(tcx).has_value()
                            && !assoc_item.is_impl_trait_in_trait()
                        {
                            let function = self.translate_assoc_item(module_id, assoc_item);
                            self.push_function(&mut parts, function);
                        }
                    }
                }
                _ => {}
            }
        }

        parts
    }

    /// Translates an associated item that has a value into its Erlang function.
    fn translate_assoc_item(
        &self,
        module_id: ErlModuleId,
        item: &ty::AssocItem,
    ) -> Function<'arena> {
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
                let exported = tcx.is_erl_exported_fn(module_id, def_id.to_def_id());
                self.translate_callable(def_id, tcx.erl_fn_name(def_id), span, exported)
            }
            ty::AssocKind::Type { .. } if item.trait_container(tcx).is_some() => {
                not_implemented!(tcx, span, "associated type default");
            }
            ty::AssocKind::Type { .. } => {
                self.translate_callable(def_id, tcx.erl_fn_name(def_id), span, true)
            }
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

        let function = Function {
            span,
            name: record_constructor_name(tag),
            clause: self.afb.clause(span, params, None, record),
            exported: true,
        };
        self.push_function(parts, function);
    }

    fn collect_impl_fns(&self, module_id: ErlModuleId, parts: &mut ModuleParts<'arena>) {
        let tcx = self.tcx;
        let Some(impls) = tcx.erl_local_modules(()).get(&module_id) else { return };

        for &impl_def_id in *impls {
            for impl_fn in tcx.erl_impl_fns(impl_def_id) {
                let function = match impl_fn {
                    ErlImplFn::Item(item) => self.translate_assoc_item(module_id, item),
                    ErlImplFn::Witness => self.translate_callable(
                        impl_def_id,
                        tcx.erl_impl_member_name(impl_def_id.to_def_id(), "$patina_witness"),
                        self.impl_self_ty_span(impl_def_id),
                        true,
                    ),
                };
                self.push_function(parts, function);
            }
        }
    }

    fn translate_impl_only_module(&self, module_id: ErlModuleId) -> (Symbol, eaf::Module<'arena>) {
        let mod_name = self.tcx.erl_module_name(module_id);

        let span = match self.tcx.erl_local_modules(()).get(&module_id) {
            Some(&&[first, ..]) => self.impl_self_ty_span(first),
            _ => bug!("impl-only module has no impls"),
        };

        let mut parts = self.module_parts(span);
        parts.forms.push(elixir_compat::info_function(&self.afb, span, None));
        parts.exports.push((sym::__info__, 1));

        self.collect_impl_fns(module_id, &mut parts);
        self.assemble_module(span, mod_name, [], parts)
    }

    fn impl_self_ty_span(&self, impl_def_id: LocalDefId) -> Span {
        self.tcx.hir_expect_item(impl_def_id).expect_impl().self_ty.span
    }
}

fn record_constructor_name(tag: Symbol) -> Symbol {
    Symbol::intern(&format!("$patina_constructor__{tag}"))
}
