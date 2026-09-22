use std::{mem, slice};

use rustc_ast::visit::{self, Visitor};
use rustc_ast::{self as ast, NodeId};
use rustc_ast_pretty::pprust;
use rustc_attr_parsing::AttributeParser;
use rustc_errors::DiagCtxtHandle;
use rustc_expand::base::ResolverExpand;
use rustc_hir::attrs::AttributeKind;
use rustc_session::Session;
use rustc_span::source_map::SourceMap;
use rustc_span::{Symbol, sym};

struct CollectProcMacros<'a> {
    macros: Vec<NodeId>,
    in_root: bool,
    dcx: DiagCtxtHandle<'a>,
    session: &'a Session,
    source_map: &'a SourceMap,
    is_test_crate: bool,
}

/// Finds this crate's proc macros and registers them with the resolver.
pub fn register_proc_macros(
    krate: &ast::Crate,
    sess: &Session,
    resolver: &mut dyn ResolverExpand,
    has_proc_macro_decls: bool,
    is_test_crate: bool,
    dcx: DiagCtxtHandle<'_>,
) {
    let mut collect = CollectProcMacros {
        macros: Vec::new(),
        in_root: true,
        dcx,
        session: sess,
        source_map: sess.source_map(),
        is_test_crate,
    };

    if has_proc_macro_decls {
        visit::walk_crate(&mut collect, krate);
    }

    if is_test_crate {
        return;
    }

    for id in collect.macros {
        resolver.declare_proc_macro(id);
    }
}

impl<'a> CollectProcMacros<'a> {
    fn collect(&mut self, item: &'a ast::Item, attr_name: Symbol) {
        if self.in_root && item.vis.kind.is_pub() {
            self.macros.push(item.id);
            return;
        }

        let msg = if !self.in_root {
            format!(
                "functions tagged with `#[{attr_name}]` must currently reside in the root of \
                 the crate"
            )
        } else {
            format!("functions tagged with `#[{attr_name}]` must be `pub`")
        };
        self.dcx.span_err(self.source_map.guess_head_span(item.span), msg);
    }
}

impl<'a> Visitor<'a> for CollectProcMacros<'a> {
    fn visit_item(&mut self, item: &'a ast::Item) {
        let mut found_attr: Option<&'a ast::Attribute> = None;

        for attr in &item.attrs {
            if attr.is_proc_macro_attr() {
                if let Some(prev_attr) = found_attr {
                    let prev_item = prev_attr.get_normal_item();
                    let item = attr.get_normal_item();
                    let path_str = pprust::path_to_string(&item.path);
                    let msg = if item.path.segments[0].ident.name
                        == prev_item.path.segments[0].ident.name
                    {
                        format!(
                            "only one `#[{path_str}]` attribute is allowed on any given function",
                        )
                    } else {
                        format!(
                            "`#[{}]` and `#[{}]` attributes cannot both be applied
                            to the same function",
                            path_str,
                            pprust::path_to_string(&prev_item.path),
                        )
                    };

                    self.dcx
                        .struct_span_err(attr.span, msg)
                        .with_span_label(prev_attr.span, "previous attribute here")
                        .emit();

                    return;
                }

                found_attr = Some(attr);
            }
        }

        let Some(attr) = found_attr else {
            let prev_in_root = mem::replace(&mut self.in_root, false);
            visit::walk_item(self, item);
            self.in_root = prev_in_root;
            return;
        };

        // Make sure we're checking a bare function. If we're not then we're
        // just not interested any further in this item.
        if !matches!(&item.kind, ast::ItemKind::Fn(_)) {
            // Error handled by general target checking logic
            return;
        }

        if self.is_test_crate {
            return;
        }

        let attr_name = if attr.has_name(sym::proc_macro_derive) {
            // A `#[proc_macro_derive]` that does not parse has already been
            // reported, so don't also complain about where it sits.
            let parsed = AttributeParser::parse_limited(
                self.session,
                slice::from_ref(attr),
                &[sym::proc_macro_derive],
            );
            matches!(
                parsed,
                Some(rustc_hir::Attribute::Parsed(AttributeKind::ProcMacroDerive { .. }))
            )
            .then_some(sym::proc_macro_derive)
        } else if attr.has_name(sym::proc_macro_attribute) {
            Some(sym::proc_macro_attribute)
        } else if attr.has_name(sym::proc_macro) {
            Some(sym::proc_macro)
        } else {
            None
        };
        if let Some(attr_name) = attr_name {
            self.collect(item, attr_name);
        }

        let prev_in_root = mem::replace(&mut self.in_root, false);
        visit::walk_item(self, item);
        self.in_root = prev_in_root;
    }
}
