use std::collections::HashMap;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use convert_case::{Case, Casing};
use rustc_arena::DroplessArena;
use rustc_ast::{BindingMode, LitKind};
use rustc_errors::DiagCtxtHandle;
use rustc_hir::def::Res;
use rustc_hir::{
    BodyId, Expr, ExprKind, FnSig, Generics, HirId, Item, ItemKind, Mod, Pat, PatKind, QPath, Stmt,
    StmtKind,
};
use rustc_middle::ty::{TyCtxt, Visibility};
use rustc_span::Ident;
use rustc_span::source_map::SourceMap;

use crate::compile::eaf_builder::{self, EafBuilder};
use crate::erlang::abstract_format as eaf;
use crate::{EmitStage, erlang};

const COMPILE_ERL_CODE: &str = "\
[InputFilename, OutputFilename] = init:get_plain_arguments(),
{ok, Bin} = file:read_file(InputFilename),
Forms = binary_to_term(Bin),
{ok, _Mod, Beam} = compile:noenv_forms(Forms, []),
ok = file:write_file(OutputFilename, Beam),
erlang:halt()";

const RENDER_ABSTR_ERL_CODE: &str = "\
[F] = init:get_plain_arguments(),
{ok,Bin} = file:read_file(F),
Forms = binary_to_term(Bin),
io:put_chars([erl_pp:form(Fm) || Fm <- Forms]),
erlang:halt()";

fn compile_beam(
    dcx: DiagCtxtHandle<'_>,
    out_dir: &Path,
    module_name: &str,
    data: &[u8],
) -> PathBuf {
    let abstract_filename = out_dir.join(format!("{module_name}.eaf"));
    let beam_filename = out_dir.join(format!("{module_name}.beam"));

    let mut file = File::create(&abstract_filename).unwrap_or_else(|e| {
        dcx.fatal(format!("failed to create file: {e}"));
    });
    file.write_all(data).unwrap_or_else(|e| {
        dcx.fatal(format!("failed to write file: {e}"));
    });

    let output = Command::new("erl")
        .args(["-noshell", "-eval", COMPILE_ERL_CODE, "-extra"])
        .arg(&abstract_filename)
        .arg(&beam_filename)
        .output()
        .unwrap_or_else(|e| {
            dcx.fatal(format!("failed to run erl: {e}"));
        });

    if output.status.success() {
        beam_filename
    } else {
        dcx.fatal(format!("erl backend failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
}

static RENDER_ABSTR_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn render_abstr(dcx: DiagCtxtHandle<'_>, eaf_bytes: &[u8]) -> String {
    let counter = RENDER_ABSTR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let scratch = std::env::temp_dir().join(format!(
        "patina-render-abstr-{}-{}.eaf",
        std::process::id(),
        counter
    ));
    let mut file = File::create(&scratch).unwrap_or_else(|e| {
        dcx.fatal(format!("failed to write file: {e}"));
    });
    file.write_all(eaf_bytes).unwrap_or_else(|e| {
        dcx.fatal(format!("failed to write file: {e}"));
    });

    let output = Command::new("erl")
        .args(["-noshell", "-eval", RENDER_ABSTR_ERL_CODE, "-extra"])
        .arg(&scratch)
        .output()
        .unwrap_or_else(|e| {
            dcx.fatal(format!("failed to run erl: {e}"));
        });

    let _ = std::fs::remove_file(&scratch);

    if output.status.success() {
        String::from_utf8(output.stdout).unwrap_or_else(|e| {
            dcx.fatal(format!("erl output was not valid UTF-8: {e}"));
        })
    } else {
        dcx.fatal(format!("erl backend failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
}

struct Translator<'tcx, 'arena> {
    tcx: TyCtxt<'tcx>,
    source_map: &'tcx SourceMap,
    afb: EafBuilder<'tcx>,
    arena: &'arena DroplessArena,
}

#[allow(unused)]
struct Fn<'tcx> {
    item: &'tcx Item<'tcx>,
    sig: &'tcx FnSig<'tcx>,
    ident: &'tcx Ident,
    generics: &'tcx Generics<'tcx>,
    body: &'tcx BodyId,
    has_body: bool,
}

struct Scope<'arena> {
    vars: HashMap<HirId, PatinaVar>,
    var_vsn: usize,
    arena: &'arena DroplessArena,
}

impl<'arena> Scope<'arena> {
    fn new(arena: &'arena DroplessArena) -> Self {
        Self { vars: HashMap::new(), var_vsn: 0, arena }
    }

    fn get_var(&self, id: HirId) -> Option<&PatinaVar> {
        self.vars.get(&id)
    }

    fn add_var(&mut self, id: HirId, is_mut: bool, ident: Ident) -> &'arena str {
        self.vars.insert(id, PatinaVar { is_mut, ident, version: self.var_vsn });
        let var_name = self.arena.alloc_str(&format!("_var_{}", self.var_vsn));
        self.var_vsn += 1;
        var_name
    }
}

#[allow(unused)]
struct PatinaVar {
    is_mut: bool,
    ident: Ident,
    version: usize,
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    fn translate(&self) -> Vec<(&'arena str, eaf::Module<'arena>)> {
        let root_module = self.tcx.hir_root_module();
        vec![self.translate_module(root_module)]
    }

    fn translate_module(&self, module: &Mod<'tcx>) -> (&'arena str, eaf::Module<'arena>) {
        let dcx = self.tcx.dcx();
        let filename = self
            .source_map
            .span_to_filename(module.spans.inner_span)
            .into_local_path()
            .unwrap_or_else(|| {
                dcx.span_bug(module.spans.inner_span, "module has no filename");
            });

        let mod_name = self.mod_name(&filename);
        let span = module.spans.inner_span;

        let file_attr = self.afb.file_attr(span, &filename.to_string_lossy());
        let module_attr = self.afb.module_attr(span, mod_name);
        let compile_attr = self.afb.wild_attr(span, "compile", vec!["no_auto_import"]);

        let pub_fns = self
            .module_funs(module)
            .filter(|fun| self.is_exported(fun))
            .map(|fun| (fun.ident.as_str(), fun.sig.decl.inputs.len()))
            .collect::<Vec<_>>();

        let export_attr = self.afb.export_attr(span, pub_fns);

        let mut forms = vec![file_attr, module_attr, compile_attr, export_attr];

        for fun in self.module_funs(module) {
            if fun.has_body {
                forms.push(self.translate_fun(fun));
            }
        }

        (mod_name, self.afb.module(forms))
    }

    fn translate_fun(&self, fun: Fn<'tcx>) -> eaf::ModuleForm<'arena> {
        let mut scope = Scope::new(self.arena);

        let body = self.tcx.hir_body(*fun.body);

        let mut function_args = Vec::new();

        for param in body.params {
            let pat = self.translate_pat(param.pat, &mut scope);
            function_args.push(pat);
        }

        let clause_body = self.afb.clause_body(self.translate_expr(body.value, &mut scope));
        let clause = self.afb.clause(fun.sig.span, function_args, clause_body);

        let fn_name = self.arena.alloc_str(fun.ident.as_str());
        let fn_arity = fun.sig.decl.inputs.len();

        self.afb.function(fun.sig.span, fn_name, fn_arity, vec![clause])
    }

    fn translate_expr(
        &self,
        expr: &'tcx Expr<'tcx>,
        scope: &mut Scope<'arena>,
    ) -> eaf::Expr<'arena> {
        let dcx = self.tcx.dcx();
        let afb = &self.afb;

        match expr.kind {
            ExprKind::Block(block, label) => {
                if label.is_some() {
                    dcx.span_fatal(expr.span, "labels are not supported");
                }

                let mut af_exprs = Vec::new();
                for stmt in block.stmts {
                    if let Some(af_expr) = self.translate_stmt(stmt, scope) {
                        af_exprs.push(af_expr);
                    }
                }
                if let Some(tail) = block.expr {
                    af_exprs.push(self.translate_expr(tail, scope));
                }

                afb.expr_block(expr.span, af_exprs)
            }

            ExprKind::Lit(lit) => match lit.node {
                LitKind::Int(val) => afb.expr_integer(lit.span, val.get()),
                LitKind::Str(symbol, _style) => afb.expr_binary_string(lit.span, symbol.as_str()),
                _ => dcx.span_fatal(
                    lit.span,
                    format!("literal kind {:?} not yet implemented", lit.node),
                ),
            },

            ExprKind::Path(QPath::Resolved(
                None,
                rustc_hir::Path { res: Res::Local(hir_id), .. },
            )) => {
                let var = scope.get_var(*hir_id).unwrap_or_else(|| {
                    dcx.span_bug(expr.span, format!("variable {hir_id:?} not in scope"));
                });
                let name = self.arena.alloc_str(&format!("_var_{}", var.version));
                afb.expr_var(expr.span, name)
            }

            ExprKind::Call(callee, args) => {
                if let Some((path_segments, target_fun_segment)) = callee.call_segments() {
                    let module = self.arena.alloc_str(
                        &path_segments
                            .iter()
                            .map(|segment| segment.ident.as_str())
                            .collect::<Vec<_>>()
                            .join("."),
                    );

                    let fn_name = self.arena.alloc_str(target_fun_segment.ident.as_str());

                    // Temporary before we actually ship runtime modules (like `patina_fmt` could've been)
                    if let ("patina_fmt", "display" | "debug") = (module, fn_name) {
                        return self.translate_format_render(expr, fn_name, &args[0], scope);
                    }

                    let translated_args =
                        args.iter().map(|arg| self.translate_expr(arg, scope)).collect::<Vec<_>>();

                    afb.expr_remote_call(
                        expr.span,
                        afb.expr_atom(callee.span, module),
                        afb.expr_atom(target_fun_segment.ident.span, fn_name),
                        translated_args,
                    )
                } else {
                    dcx.span_fatal(
                        expr.span,
                        format!("non-extern-mod function calls not yet implemented: {callee:?}"),
                    );
                }
            }

            ExprKind::Array(elements) => self.afb.expr_list(
                expr.span,
                elements.iter().map(|element| self.translate_expr(element, scope)),
            ),

            ExprKind::Cast(expr, _ty) => self.translate_expr(expr, scope),

            ExprKind::Tup(elements) => {
                let elements = elements
                    .iter()
                    .map(|element| self.translate_expr(element, scope))
                    .collect::<Vec<_>>();
                afb.expr_tuple(expr.span, elements)
            }

            _ => dcx.span_fatal(
                expr.span,
                format!("expression kind {:?} not yet implemented", expr.kind),
            ),
        }
    }

    fn translate_format_render(
        &self,
        expr: &'tcx Expr<'tcx>,
        fn_name: &str,
        arg: &'tcx Expr<'tcx>,
        scope: &mut Scope<'arena>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        let typeck = self.tcx.typeck(arg.hir_id.owner.def_id);
        let ty = typeck.expr_ty(arg);

        let translated = self.translate_expr(arg, scope);

        if fn_name == "display" && ty.is_str() {
            return translated;
        }

        let format_str = self.afb.expr_string(expr.span, "~tp");
        let args_list = self.afb.expr_list(expr.span, [translated]);

        afb.expr_remote_call(
            expr.span,
            afb.expr_atom(expr.span, "io_lib"),
            afb.expr_atom(expr.span, "format"),
            vec![format_str, args_list],
        )
    }

    fn translate_stmt(
        &self,
        stmt: &'tcx Stmt<'tcx>,
        scope: &mut Scope<'arena>,
    ) -> Option<eaf::Expr<'arena>> {
        let dcx = self.tcx.dcx();
        let afb = &self.afb;

        match stmt.kind {
            StmtKind::Let(let_stmt) => {
                let rhs = {
                    let init = let_stmt.init?;
                    self.translate_expr(init, scope)
                };
                let lhs = self.translate_pat(let_stmt.pat, scope);
                Some(afb.expr_match(stmt.span, lhs, rhs))
            }

            StmtKind::Semi(expr) | StmtKind::Expr(expr) => Some(self.translate_expr(expr, scope)),

            _ => dcx.span_fatal(
                stmt.span,
                format!("statement kind {:?} not yet implemented", stmt.kind),
            ),
        }
    }

    fn translate_pat(
        &self,
        pat: &'tcx Pat<'tcx>,
        scope: &mut Scope<'arena>,
    ) -> eaf::Pattern<'arena> {
        let dcx = self.tcx.dcx();
        let afb = &self.afb;

        match pat.kind {
            PatKind::Wild => afb.pat_var(pat.span, "_"),

            PatKind::Binding(binding_mode, id, ident, subpat) => {
                let is_mut = match binding_mode {
                    BindingMode::NONE => false,
                    BindingMode::MUT => true,
                    _ => dcx.span_fatal(
                        pat.span,
                        format!("binding mode {:?} not yet implemented", binding_mode),
                    ),
                };

                if subpat.is_some() {
                    dcx.span_fatal(pat.span, "binding with subpattern not yet implemented");
                }

                let var_name = scope.add_var(id, is_mut, ident);
                afb.pat_var(pat.span, var_name)
            }

            _ => {
                dcx.span_fatal(pat.span, format!("pattern kind {:?} not yet implemented", pat.kind))
            }
        }
    }

    fn mod_name(&self, filename: &Path) -> &'arena str {
        let dcx = self.tcx.dcx();
        let mod_name = filename
            .file_stem()
            .unwrap_or_else(|| dcx.bug("module filename is not a valid path"))
            .to_str()
            .unwrap_or_else(|| dcx.bug("module filename basename is not valid UTF-8"))
            .to_case(Case::Pascal);

        self.arena.alloc_str(&format!("Elixir.{mod_name}"))
    }

    fn is_exported(&self, fun: &Fn<'tcx>) -> bool {
        match self.tcx.visibility(fun.item.owner_id.def_id) {
            Visibility::Public => true,
            Visibility::Restricted(restricted_id) => {
                restricted_id
                    != self.tcx.parent_module_from_def_id(fun.item.owner_id.def_id).to_def_id()
            }
        }
    }

    fn module_funs(&self, module: &Mod<'tcx>) -> impl Iterator<Item = Fn<'tcx>> {
        module.item_ids.iter().flat_map(|item_id| {
            let item = self.tcx.hir_item(*item_id);
            if let ItemKind::Fn { sig, ident, generics, body, has_body } = &item.kind {
                Some(Fn { item, sig, ident, generics, body, has_body: *has_body })
            } else {
                None
            }
        })
    }
}

pub(super) fn translate(tcx: TyCtxt<'_>, stage: EmitStage, out_dir: &Path) {
    let dcx = tcx.sess.dcx();
    let source_map = tcx.sess.source_map();
    let arena = DroplessArena::default();

    let afb = EafBuilder::new(source_map);
    let translator = Translator { tcx, source_map, afb, arena: &arena };

    let modules = translator.translate();

    match stage {
        EmitStage::Eaf => {
            for (mod_name, module) in &modules {
                let bytes = erlang::to_bytes(module).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to serialize module: {e}"));
                });
                let eaf_path = out_dir.join(format!("{mod_name}.eaf"));
                let mut file = File::create(&eaf_path).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write file: {e}"));
                });
                file.write_all(&bytes).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write file: {e}"));
                });
            }
        }
        EmitStage::Abstr => {
            for (mod_name, module) in &modules {
                let bytes = erlang::to_bytes(module).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to serialize module: {e}"));
                });
                let mut rendered = render_abstr(dcx, &bytes);
                if !rendered.ends_with('\n') {
                    rendered.push('\n');
                }
                let abstr_path = out_dir.join(format!("{mod_name}.abstr"));
                let mut file = File::create(&abstr_path).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write file: {e}"));
                });
                file.write_all(rendered.as_bytes()).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write file: {e}"));
                });
            }
        }
        EmitStage::Beam => {
            for (mod_name, module) in &modules {
                let bytes = erlang::to_bytes(module).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to serialize module: {e}"));
                });
                let _beam_path = compile_beam(dcx, out_dir, mod_name, &bytes);
            }
        }
    }
}
