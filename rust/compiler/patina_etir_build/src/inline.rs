//! Inlines named constants and calls to `#[inline]` functions, by grafting copies of their
//! templates. Constants get lowered to a var.

use std::ops::Range;

use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::erl::ErlCallTarget;
use rustc_middle::erl::etir::{Callee, Expr, ExprId, ExprKind, Pat, PatId, PatKind, VarId};
use rustc_middle::span_bug;
use rustc_middle::ty::GenericArgsRef;
use rustc_span::{Ident, Span};

use crate::graft::Graft;
use crate::{BodyBuildCx, ExprBlock, VarPrefix};

impl<'tcx> BodyBuildCx<'tcx> {
    pub(super) fn inline(&mut self) {
        self.body.value = with_expr_block!(self, |const_bindings| {
            let exprs = ExprId::ZERO..self.body.exprs.next_index();
            let pats = PatId::ZERO..self.body.pats.next_index();
            Inliner::new(self, &mut const_bindings).inline(exprs, pats);
            self.body.value
        });
    }
}

struct Inliner<'a, 'tcx> {
    cx: &'a mut BodyBuildCx<'tcx>,
    stack: Vec<DefId>,
    const_vars: FxHashMap<(DefId, GenericArgsRef<'tcx>), VarId>,
    const_bindings: &'a mut ExprBlock,
}

impl<'a, 'tcx> Inliner<'a, 'tcx> {
    fn new(cx: &'a mut BodyBuildCx<'tcx>, const_bindings: &'a mut ExprBlock) -> Self {
        Self { stack: vec![cx.owner], cx, const_vars: FxHashMap::default(), const_bindings }
    }

    fn inline(&mut self, exprs: Range<ExprId>, pats: Range<PatId>) {
        for id in exprs {
            let Expr { ref kind, ty: _, span } = self.cx.body[id];

            let inline_kind = match *kind {
                ExprKind::Const { def_id, args } => {
                    let var = self.const_var(def_id, args, span);
                    Some(ExprKind::Var(var))
                }
                ExprKind::Call {
                    callee:
                        Callee::Item {
                            target: ErlCallTarget::Item { def_id, args: inline_args },
                            forced_extern: false,
                        },
                    ref args,
                } => {
                    let call_args = args.clone();
                    self.inline_call(def_id, inline_args, &call_args)
                }
                _ => None,
            };

            if let Some(inline_kind) = inline_kind {
                self.cx.body.exprs[id].kind = inline_kind;
            }
        }

        for id in pats {
            let Pat { ref kind, ty: _, span } = self.cx.body[id];

            let inline_kind = match *kind {
                PatKind::Const { def_id, args } => {
                    let var = self.const_var(def_id, args, span);
                    Some(PatKind::Pin { var })
                }
                _ => None,
            };

            if let Some(inline_kind) = inline_kind {
                self.cx.body.pats[id].kind = inline_kind;
            }
        }
    }

    fn inline_call(
        &mut self,
        def_id: DefId,
        inline_args: GenericArgsRef<'tcx>,
        call_args: &[ExprId],
    ) -> Option<ExprKind<'tcx>> {
        let tcx = self.cx.tcx;
        let Some((def_id, inline_args)) =
            tcx.erl_try_resolve_item(self.cx.owner, def_id, inline_args)
        else {
            // If we can't resolve a specific item, we can't inline a call to it.
            // For example, if we need to call a trait function through a witness,
            // then we need runtime information to know what to call.
            return None;
        };

        // Break the inline loop by keeping the original call.
        if !tcx.has_inline_attr(def_id) || self.stack.contains(&def_id) {
            return None;
        }

        let value = self.graft_and_recursively_inline(def_id, inline_args, call_args);
        Some(ExprKind::Block(Box::new([value])))
    }

    fn const_var(&mut self, def_id: DefId, args: GenericArgsRef<'tcx>, span: Span) -> VarId {
        if let Some(&var) = self.const_vars.get(&(def_id, args)) {
            return var;
        }

        if self.stack.contains(&def_id) {
            span_bug!(span, "cycle between constants should've been reported as a query cycle");
        }

        let value = self.graft_and_recursively_inline(def_id, args, &[]);

        let name = VarPrefix::Const.to_name(self.cx.tcx, def_id);
        let var = self.cx.push_var(Ident::new(name, span));
        self.const_bindings.bind(self.cx, var, value);

        self.const_vars.insert((def_id, args), var);
        var
    }

    fn graft_and_recursively_inline(
        &mut self,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
        call_args: &[ExprId],
    ) -> ExprId {
        let template = self.cx.tcx.etir_template(def_id).clone();

        let Graft { value, exprs, pats } = self.cx.graft(template, args, call_args);

        self.stack.push(def_id);
        self.inline(exprs, pats);
        self.stack.pop();

        value
    }
}
