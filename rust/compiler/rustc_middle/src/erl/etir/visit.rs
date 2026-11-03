use super::{
    AdtExpr, AdtPat, Arm, BinarySegmentExpr, BinarySegmentPat, Body, Callee, CatchArm, Expr,
    ExprId, ExprKind, FieldExpr, FieldPat, Join, JoinId, Loop, LoopId, Pat, PatId, PatKind,
    PatRange, ReceiveAfter, VarId,
};
use crate::erl::ErlCallTarget;
use crate::erl::binary_pattern::{BinarySegmentLayout, BinarySegmentSize};
use crate::ty::{GenericArgsRef, Ty};

/// Visits a body by following ids from a node to the nodes they refer to.
///
/// Importantly, this means that some expressions may be visited more than once
/// if an ExprId is copied, and some may not be visited at all if they're not
/// reachable from the root.
///
/// Every `walk_*` method uses deconstruction to access fields of structs and
/// enums. This will result in a compile error if a field is added, which makes
/// it more likely the appropriate visit call will be added for it.
pub trait Visitor<'etir, 'tcx: 'etir>: Sized {
    fn etir(&self) -> &'etir Body<'tcx>;

    fn visit_expr(&mut self, _id: ExprId, expr: &'etir Expr<'tcx>) {
        walk_expr(self, expr);
    }

    fn visit_arm(&mut self, arm: &'etir Arm) {
        walk_arm(self, arm);
    }

    fn visit_catch_arm(&mut self, arm: &'etir CatchArm) {
        walk_catch_arm(self, arm);
    }

    fn visit_pat(&mut self, _id: PatId, pat: &'etir Pat<'tcx>) {
        walk_pat(self, pat);
    }

    fn visit_expr_id(&mut self, &id: &ExprId) {
        self.visit_expr(id, &self.etir()[id]);
    }

    fn visit_pat_id(&mut self, &id: &PatId) {
        self.visit_pat(id, &self.etir()[id]);
    }

    fn visit_var_id(&mut self, _id: &VarId) {}

    fn visit_loop_id(&mut self, _id: &LoopId) {}

    fn visit_join_id(&mut self, _id: &JoinId) {}

    fn visit_ty(&mut self, _ty: &Ty<'tcx>) {}

    fn visit_args(&mut self, _args: &GenericArgsRef<'tcx>) {}

    /// Visits the join defined by a `Join` expr, followed by its body.
    fn visit_join_def(&mut self, id: &JoinId) {
        self.visit_join_id(id);
        walk_join(self, &self.etir()[*id]);
    }
}

/// Visits the ids and nodes within a single node, by walking Body arenas.
/// Importantly, unlike `Visitor` it does *not* walk the expression tree.
pub trait MutVisitor<'etir, 'tcx: 'etir>: Sized {
    fn visit_expr(&mut self, expr: &'etir mut Expr<'tcx>) {
        walk_expr_mut(self, expr);
    }

    fn visit_arm(&mut self, arm: &'etir mut Arm) {
        walk_arm_mut(self, arm);
    }

    fn visit_catch_arm(&mut self, arm: &'etir mut CatchArm) {
        walk_catch_arm_mut(self, arm);
    }

    fn visit_pat(&mut self, pat: &'etir mut Pat<'tcx>) {
        walk_pat_mut(self, pat);
    }

    fn visit_loop(&mut self, loop_: &'etir mut Loop) {
        walk_loop_mut(self, loop_);
    }

    fn visit_join(&mut self, join: &'etir mut Join) {
        walk_join_mut(self, join);
    }

    fn visit_expr_id(&mut self, _id: &mut ExprId) {}

    fn visit_pat_id(&mut self, _id: &mut PatId) {}

    fn visit_var_id(&mut self, _id: &mut VarId) {}

    fn visit_loop_id(&mut self, _id: &mut LoopId) {}

    fn visit_join_id(&mut self, _id: &mut JoinId) {}

    fn visit_ty(&mut self, _ty: &mut Ty<'tcx>) {}

    fn visit_args(&mut self, _args: &mut GenericArgsRef<'tcx>) {}

    fn visit_join_def(&mut self, id: &mut JoinId) {
        self.visit_join_id(id);
    }
}

/// Visits the parameters and the value of `body`, and through them everything they refer to.
pub fn walk_body<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(
    visitor: &mut V,
    body: &'etir Body<'tcx>,
) {
    let Body { params, value, exprs: _, pats: _, vars: _, loops: _, joins: _ } = body;
    for param in params {
        visitor.visit_pat_id(param);
    }
    visitor.visit_expr_id(value);
}

/// Visits the parameters and the value of `body`, and every node of its arenas.
pub fn walk_body_mut<'etir, 'tcx: 'etir, V: MutVisitor<'etir, 'tcx>>(
    visitor: &mut V,
    body: &'etir mut Body<'tcx>,
) {
    let Body { params, value, exprs, pats, vars: _, loops, joins } = body;
    for param in params {
        visitor.visit_pat_id(param);
    }
    visitor.visit_expr_id(value);
    for expr in exprs {
        visitor.visit_expr(expr);
    }
    for pat in pats {
        visitor.visit_pat(pat);
    }
    for loop_ in loops {
        visitor.visit_loop(loop_);
    }
    for join in joins {
        visitor.visit_join(join);
    }
}

/// Defines the `walk_*` functions of a visitor, with names ending in `$suffix`. Its ids are
/// passed by a shared reference to `Visitor`, and by a mutable reference to `MutVisitor`, so with
/// default binding modes the same body works for both.
macro_rules! make_walkers {
    ($Visitor:ident, $($mutability:ident)?; $suffix:literal) => {
        pub fn ${concat(walk_expr, $suffix)}<'etir, 'tcx: 'etir, V: $Visitor<'etir, 'tcx>>(
            visitor: &mut V,
            expr: &'etir $($mutability)? Expr<'tcx>,
        ) {
            use ExprKind::*;
            let Expr { kind, ty, span: _ } = expr;
            visitor.visit_ty(ty);
            match kind {
                Block(exprs) => {
                    for expr in exprs {
                        visitor.visit_expr_id(expr);
                    }
                }
                Literal(_, _) => {}
                Var(var) => visitor.visit_var_id(var),
                Const { def_id: _, args } => visitor.visit_args(args),
                List { items, tail } => {
                    if let Some(tail) = tail {
                        visitor.visit_expr_id(tail);
                    }
                    for item in items {
                        visitor.visit_expr_id(item);
                    }
                }
                Map { fields, base } => {
                    if let Some(base) = base {
                        visitor.visit_expr_id(base);
                    }
                    for (key, value) in fields {
                        visitor.visit_expr_id(key);
                        visitor.visit_expr_id(value);
                    }
                }
                Tuple { fields } => {
                    for field in fields {
                        visitor.visit_expr_id(field);
                    }
                }
                BinaryPattern { segments } => {
                    for BinarySegmentExpr { value, layout, span: _ } in segments {
                        visitor.visit_expr_id(value);
                        ${concat(walk_segment_layout, $suffix)}(visitor, layout);
                    }
                }
                Field { lhs, repr: _, position: _, name: _ } => visitor.visit_expr_id(lhs),
                TupleField { lhs, position: _ } => visitor.visit_expr_id(lhs),
                BinaryOp { op: _, lhs, rhs } | LogicalOp { op: _, lhs, rhs } => {
                    visitor.visit_expr_id(lhs);
                    visitor.visit_expr_id(rhs);
                }
                UnaryOp { op: _, arg } => visitor.visit_expr_id(arg),
                Cast { source, kind: _ } => visitor.visit_expr_id(source),
                Call { callee, args } => {
                    match callee {
                        Callee::Value(callee) => visitor.visit_expr_id(callee),
                        Callee::Item { target, forced_extern: _ } => {
                            ${concat(walk_call_target, $suffix)}(visitor, target)
                        }
                        Callee::Fun { .. } | Callee::Direct(_, _) => {}
                    }
                    for arg in args {
                        visitor.visit_expr_id(arg);
                    }
                }
                Adt(box AdtExpr { repr: _, base, fields }) => {
                    if let Some(base) = base {
                        visitor.visit_expr_id(base);
                    }
                    for FieldExpr { position: _, name: _, value } in fields {
                        visitor.visit_expr_id(value);
                    }
                }
                Closure { params, body } => {
                    for param in params {
                        visitor.visit_pat_id(param);
                    }
                    visitor.visit_expr_id(body);
                }
                FnItem { target, arity: _, forced_extern: _ } => {
                    ${concat(walk_call_target, $suffix)}(visitor, target)
                }
                Fun { fun: _, forced_extern: _ } => {}
                Assign { lhs, rhs } => {
                    visitor.visit_expr_id(rhs);
                    visitor.visit_pat_id(lhs);
                }
                Declare { vars } => {
                    for var in vars {
                        visitor.visit_var_id(var);
                    }
                }
                Match { scrutinee, arms } => {
                    visitor.visit_expr_id(scrutinee);
                    for arm in arms {
                        visitor.visit_arm(arm);
                    }
                }
                Receive { arms, after } => {
                    if let Some(ReceiveAfter { timeout, body }) = after {
                        visitor.visit_expr_id(body);
                        visitor.visit_expr_id(timeout);
                    }
                    for arm in arms {
                        visitor.visit_arm(arm);
                    }
                }
                TryCatch { body, arms, after } => {
                    visitor.visit_expr_id(body);
                    if let Some(after) = after {
                        visitor.visit_expr_id(after);
                    }
                    for arm in arms {
                        visitor.visit_catch_arm(arm);
                    }
                }
                Loop { id, body } => {
                    visitor.visit_loop_id(id);
                    visitor.visit_expr_id(body);
                }
                Join { join, body } => {
                    visitor.visit_expr_id(body);
                    visitor.visit_join_def(join);
                }
                Jump { target, value } => {
                    visitor.visit_join_id(target);
                    if let Some(value) = value {
                        visitor.visit_expr_id(value);
                    }
                }
            }
        }

        pub fn ${concat(walk_arm, $suffix)}<'etir, 'tcx: 'etir, V: $Visitor<'etir, 'tcx>>(
            visitor: &mut V,
            arm: &'etir $($mutability)? Arm,
        ) {
            let Arm { pattern, guard, body, span: _ } = arm;
            visitor.visit_pat_id(pattern);
            if let Some(guard) = guard {
                visitor.visit_expr_id(guard);
            }
            visitor.visit_expr_id(body);
        }

        pub fn ${concat(walk_catch_arm, $suffix)}<'etir, 'tcx: 'etir, V: $Visitor<'etir, 'tcx>>(
            visitor: &mut V,
            arm: &'etir $($mutability)? CatchArm,
        ) {
            let CatchArm { class, reason, stacktrace, guard, body, span: _ } = arm;
            visitor.visit_pat_id(class);
            visitor.visit_pat_id(reason);
            if let Some(stacktrace) = stacktrace {
                visitor.visit_var_id(stacktrace);
            }
            if let Some(guard) = guard {
                visitor.visit_expr_id(guard);
            }
            visitor.visit_expr_id(body);
        }

        pub fn ${concat(walk_pat, $suffix)}<'etir, 'tcx: 'etir, V: $Visitor<'etir, 'tcx>>(
            visitor: &mut V,
            pat: &'etir $($mutability)? Pat<'tcx>,
        ) {
            use PatKind::*;
            let Pat { kind, ty, span: _ } = pat;
            visitor.visit_ty(ty);
            match kind {
                Wild { input: _ } => {}
                Binding { var, is_primary: _, subpattern, input: _ } => {
                    visitor.visit_var_id(var);
                    if let Some(subpattern) = subpattern {
                        visitor.visit_pat_id(subpattern);
                    }
                }
                Pin { var } | Assignee { var } => visitor.visit_var_id(var),
                Const { def_id: _, args } => visitor.visit_args(args),
                TermClass { class: _, subpattern } => visitor.visit_pat_id(subpattern),
                Tuple { fields } => {
                    for field in fields {
                        visitor.visit_pat_id(field);
                    }
                }
                Adt(box AdtPat { repr: _, fields, input: _ }) => {
                    for FieldPat { name: _, ty, pattern } in fields {
                        visitor.visit_ty(ty);
                        visitor.visit_pat_id(pattern);
                    }
                }
                Literal(_) => {}
                Range(box PatRange { lo, hi, end: _, ty, input: _ }) => {
                    visitor.visit_ty(ty);
                    if let Some(lo) = lo {
                        visitor.visit_expr_id(lo);
                    }
                    if let Some(hi) = hi {
                        visitor.visit_expr_id(hi);
                    }
                }
                Map { fields } => {
                    for (key, _) in & $($mutability)? **fields {
                        visitor.visit_expr_id(key);
                    }
                    for (_, pattern) in fields {
                        visitor.visit_pat_id(pattern);
                    }
                }
                BinaryPattern { segments } => {
                    // A size can only refer to bindings of earlier segments,
                    // so it comes before its pattern.
                    for BinarySegmentPat { pattern, layout, span: _ } in segments {
                        ${concat(walk_segment_layout, $suffix)}(visitor, layout);
                        visitor.visit_pat_id(pattern);
                    }
                }
                List { items, tail } => {
                    if let Some(tail) = tail {
                        visitor.visit_pat_id(tail);
                    }
                    for item in items {
                        visitor.visit_pat_id(item);
                    }
                }
                Or { pats } => {
                    for pat in pats {
                        visitor.visit_pat_id(pat);
                    }
                }
            }
        }

        fn ${concat(walk_segment_layout, $suffix)}<'etir, 'tcx: 'etir, V: $Visitor<'etir, 'tcx>>(
            visitor: &mut V,
            layout: & $($mutability)? BinarySegmentLayout<ExprId>,
        ) {
            let BinarySegmentLayout { ty: _, size, unit: _, signed: _, endianness: _ } = layout;
            match size {
                BinarySegmentSize::Dynamic(size) => visitor.visit_expr_id(size),
                BinarySegmentSize::Fixed(_)
                | BinarySegmentSize::OfValue
                | BinarySegmentSize::Unsized
                | BinarySegmentSize::Encoded(_) => {}
            }
        }

        fn ${concat(walk_call_target, $suffix)}<'etir, 'tcx: 'etir, V: $Visitor<'etir, 'tcx>>(
            visitor: &mut V,
            target: & $($mutability)? ErlCallTarget<'tcx>,
        ) {
            match target {
                ErlCallTarget::Item { def_id: _, args } => visitor.visit_args(args),
                ErlCallTarget::Constructor { nominal_type: _, variant: _ }
                | ErlCallTarget::Extern { module: _, function: _ }
                | ErlCallTarget::Dynamic => {}
            }
        }

        pub fn ${concat(walk_join, $suffix)}<'etir, 'tcx: 'etir, V: $Visitor<'etir, 'tcx>>(
            visitor: &mut V,
            join: &'etir $($mutability)? Join,
        ) {
            let Join { body, value, params } = join;
            if let Some(value) = value {
                visitor.visit_var_id(value);
            }
            for param in params {
                visitor.visit_var_id(param);
            }
            visitor.visit_expr_id(body);
        }

        pub fn ${concat(walk_loop, $suffix)}<'etir, 'tcx: 'etir, V: $Visitor<'etir, 'tcx>>(
            visitor: &mut V,
            loop_: &'etir $($mutability)? Loop,
        ) {
            let Loop { reassigned } = loop_;
            for var in reassigned {
                visitor.visit_var_id(var);
            }
        }
    };
}

make_walkers!(Visitor, ; "");
make_walkers!(MutVisitor, mut; "_mut");
