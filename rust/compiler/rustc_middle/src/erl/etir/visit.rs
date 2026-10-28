use super::{
    AdtExpr, AdtPat, Arm, BinarySegmentExpr, BinarySegmentPat, Body, Callee, CatchArm, Expr,
    ExprId, ExprKind, FieldExpr, FieldPat, Pat, PatId, PatKind, PatRange, ReceiveAfter,
};
use crate::erl::binary_pattern::{BinarySegmentLayout, BinarySegmentSize};

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
}

pub fn visit_expr_id<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(visitor: &mut V, id: ExprId) {
    visitor.visit_expr(id, &visitor.etir()[id]);
}

pub fn visit_pat_id<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(visitor: &mut V, id: PatId) {
    visitor.visit_pat(id, &visitor.etir()[id]);
}

pub fn walk_expr<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(
    visitor: &mut V,
    expr: &'etir Expr<'tcx>,
) {
    use ExprKind::*;
    let Expr { kind, ty: _, span: _ } = expr;
    match *kind {
        Block(ref exprs) => {
            for &expr in &**exprs {
                visit_expr_id(visitor, expr);
            }
        }
        Literal(_, _) => {}
        Var(_) => {}
        List { ref items, tail } => {
            if let Some(tail) = tail {
                visit_expr_id(visitor, tail);
            }
            for &item in &**items {
                visit_expr_id(visitor, item);
            }
        }
        Map { ref fields, base } => {
            if let Some(base) = base {
                visit_expr_id(visitor, base);
            }
            for &(key, value) in &**fields {
                visit_expr_id(visitor, key);
                visit_expr_id(visitor, value);
            }
        }
        Tuple { ref fields } => {
            for &field in &**fields {
                visit_expr_id(visitor, field);
            }
        }
        BinaryPattern { ref segments } => {
            for segment in &**segments {
                let BinarySegmentExpr { value, ref layout, span: _ } = *segment;
                visit_expr_id(visitor, value);
                walk_segment_layout(visitor, layout);
            }
        }
        Field { lhs, repr: _, position: _, name: _ } => visit_expr_id(visitor, lhs),
        TupleField { lhs, position: _ } => visit_expr_id(visitor, lhs),
        BinaryOp { op: _, lhs, rhs } | LogicalOp { op: _, lhs, rhs } => {
            visit_expr_id(visitor, lhs);
            visit_expr_id(visitor, rhs);
        }
        UnaryOp { op: _, arg } => visit_expr_id(visitor, arg),
        Cast { source, kind: _ } => visit_expr_id(visitor, source),
        Call { ref callee, ref args } => {
            match *callee {
                Callee::Value(callee) => visit_expr_id(visitor, callee),
                Callee::Item { .. } | Callee::Fun { .. } | Callee::Direct(_, _) => {}
            }
            for &arg in &**args {
                visit_expr_id(visitor, arg);
            }
        }
        Adt(box AdtExpr { repr: _, base, ref fields }) => {
            if let Some(base) = base {
                visit_expr_id(visitor, base);
            }
            for &FieldExpr { position: _, name: _, value } in &**fields {
                visit_expr_id(visitor, value);
            }
        }
        Closure { ref params, body } => {
            for &param in &**params {
                visit_pat_id(visitor, param);
            }
            visit_expr_id(visitor, body);
        }
        FnItem { target: _, arity: _, forced_extern: _ } => {}
        Fun { fun: _, forced_extern: _ } => {}
        Const { def_id: _ } => {}
        Assign { lhs, rhs } => {
            visit_expr_id(visitor, rhs);
            visit_pat_id(visitor, lhs);
        }
        Declare { vars: _ } => {}
        Match { scrutinee, ref arms } => {
            visit_expr_id(visitor, scrutinee);
            for arm in &**arms {
                visitor.visit_arm(arm);
            }
        }
        Receive { ref arms, ref after } => {
            if let Some(ReceiveAfter { timeout, body }) = *after {
                visit_expr_id(visitor, body);
                visit_expr_id(visitor, timeout);
            }
            for arm in &**arms {
                visitor.visit_arm(arm);
            }
        }
        TryCatch { body, ref arms, after } => {
            visit_expr_id(visitor, body);
            if let Some(after) = after {
                visit_expr_id(visitor, after);
            }
            for arm in &**arms {
                visitor.visit_catch_arm(arm);
            }
        }
        Loop { id: _, body } => visit_expr_id(visitor, body),
        Join { join, body } => {
            visit_expr_id(visitor, body);
            visit_expr_id(visitor, visitor.etir()[join].body);
        }
        Jump { target: _, value } => {
            if let Some(value) = value {
                visit_expr_id(visitor, value);
            }
        }
    }
}

pub fn walk_arm<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(visitor: &mut V, arm: &'etir Arm) {
    let Arm { pattern, guard, body, span: _ } = arm;
    visit_pat_id(visitor, *pattern);
    if let Some(guard) = guard {
        visit_expr_id(visitor, *guard);
    }
    visit_expr_id(visitor, *body);
}

pub fn walk_catch_arm<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(
    visitor: &mut V,
    arm: &'etir CatchArm,
) {
    let CatchArm { class, reason, stacktrace: _, guard, body, span: _ } = arm;
    visit_pat_id(visitor, *class);
    visit_pat_id(visitor, *reason);
    if let Some(guard) = guard {
        visit_expr_id(visitor, *guard);
    }
    visit_expr_id(visitor, *body);
}

pub fn walk_pat<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(
    visitor: &mut V,
    pat: &'etir Pat<'tcx>,
) {
    use PatKind::*;
    let Pat { kind, ty: _, span: _ } = pat;
    match *kind {
        Wild { input: _ } => {}
        Binding { var: _, is_primary: _, subpattern, input: _ } => {
            if let Some(subpattern) = subpattern {
                visit_pat_id(visitor, subpattern);
            }
        }
        Pin { var: _ } => {}
        Assignee { var: _ } => {}
        TermClass { class: _, subpattern } => visit_pat_id(visitor, subpattern),
        Tuple { ref fields } => {
            for &field in &**fields {
                visit_pat_id(visitor, field);
            }
        }
        Adt(box AdtPat { repr: _, ref fields, input: _ }) => {
            for &FieldPat { name: _, ty: _, pattern } in fields {
                visit_pat_id(visitor, pattern);
            }
        }
        Literal(_) => {}
        Range(box PatRange { lo, hi, end: _, ty: _, input: _ }) => {
            if let Some(lo) = lo {
                visit_expr_id(visitor, lo);
            }
            if let Some(hi) = hi {
                visit_expr_id(visitor, hi);
            }
        }
        Map { ref fields } => {
            for &(key, _) in &**fields {
                visit_expr_id(visitor, key);
            }
            for &(_, pattern) in &**fields {
                visit_pat_id(visitor, pattern);
            }
        }
        // A size can only refer to bindings of earlier segments, so it comes before its pattern.
        BinaryPattern { ref segments } => {
            for segment in &**segments {
                let BinarySegmentPat { pattern, ref layout, span: _ } = *segment;
                walk_segment_layout(visitor, layout);
                visit_pat_id(visitor, pattern);
            }
        }
        List { ref items, tail } => {
            if let Some(tail) = tail {
                visit_pat_id(visitor, tail);
            }
            for &item in &**items {
                visit_pat_id(visitor, item);
            }
        }
        Or { ref pats } => {
            for &pat in &**pats {
                visit_pat_id(visitor, pat);
            }
        }
    }
}

fn walk_segment_layout<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(
    visitor: &mut V,
    layout: &BinarySegmentLayout<ExprId>,
) {
    let BinarySegmentLayout { ty: _, size, unit: _, signed: _, endianness: _ } = *layout;
    match size {
        BinarySegmentSize::Dynamic(size) => visit_expr_id(visitor, size),
        BinarySegmentSize::Fixed(_)
        | BinarySegmentSize::OfValue
        | BinarySegmentSize::Unsized
        | BinarySegmentSize::Encoded(_) => {}
    }
}
