use super::{
    AdtExpr, AdtPat, Arm, BinarySegmentExpr, BinarySegmentPat, Body, Callee, CatchArm, Expr,
    ExprId, ExprKind, FieldExpr, FieldPat, Pat, PatKind, PatRange, ReceiveAfter,
};
use crate::erl::binary_pattern::{BinarySegmentLayout, BinarySegmentSize};

/// Every `walk_*` method uses deconstruction to access fields of structs and
/// enums. This will result in a compile error if a field is added, which makes
/// it more likely the appropriate visit call will be added for it.
pub trait Visitor<'etir, 'tcx: 'etir>: Sized {
    fn etir(&self) -> &'etir Body<'tcx>;

    fn visit_expr(&mut self, expr: &'etir Expr<'tcx>) {
        walk_expr(self, expr);
    }

    fn visit_arm(&mut self, arm: &'etir Arm) {
        walk_arm(self, arm);
    }

    fn visit_catch_arm(&mut self, arm: &'etir CatchArm) {
        walk_catch_arm(self, arm);
    }

    fn visit_pat(&mut self, pat: &'etir Pat<'tcx>) {
        walk_pat(self, pat);
    }
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
                visitor.visit_expr(&visitor.etir()[expr]);
            }
        }
        Literal(_, _) => {}
        Var(_) => {}
        List { ref items, tail } => {
            if let Some(tail) = tail {
                visitor.visit_expr(&visitor.etir()[tail]);
            }
            for &item in &**items {
                visitor.visit_expr(&visitor.etir()[item]);
            }
        }
        Map { ref fields, base } => {
            if let Some(base) = base {
                visitor.visit_expr(&visitor.etir()[base]);
            }
            for &(key, value) in &**fields {
                visitor.visit_expr(&visitor.etir()[key]);
                visitor.visit_expr(&visitor.etir()[value]);
            }
        }
        Tuple { ref fields } => {
            for &field in &**fields {
                visitor.visit_expr(&visitor.etir()[field]);
            }
        }
        BinaryPattern { ref segments } => {
            for segment in &**segments {
                let BinarySegmentExpr { value, ref layout, span: _ } = *segment;
                visitor.visit_expr(&visitor.etir()[value]);
                walk_segment_layout(visitor, layout);
            }
        }
        Field { lhs, repr: _, position: _, name: _ } => visitor.visit_expr(&visitor.etir()[lhs]),
        TupleField { lhs, position: _ } => visitor.visit_expr(&visitor.etir()[lhs]),
        BinaryOp { op: _, lhs, rhs } | LogicalOp { op: _, lhs, rhs } => {
            visitor.visit_expr(&visitor.etir()[lhs]);
            visitor.visit_expr(&visitor.etir()[rhs]);
        }
        UnaryOp { op: _, arg } => visitor.visit_expr(&visitor.etir()[arg]),
        Cast { source, kind: _ } => visitor.visit_expr(&visitor.etir()[source]),
        Call { ref callee, ref args } => {
            match *callee {
                Callee::Value(callee) => visitor.visit_expr(&visitor.etir()[callee]),
                Callee::Item { .. } | Callee::Direct(_, _) => {}
            }
            for &arg in &**args {
                visitor.visit_expr(&visitor.etir()[arg]);
            }
        }
        Adt(box AdtExpr { repr: _, base, ref fields }) => {
            if let Some(base) = base {
                visitor.visit_expr(&visitor.etir()[base]);
            }
            for &FieldExpr { position: _, name: _, value } in &**fields {
                visitor.visit_expr(&visitor.etir()[value]);
            }
        }
        Closure { def_id: _, ref params, body } => {
            for &param in &**params {
                visitor.visit_pat(&visitor.etir()[param]);
            }
            visitor.visit_expr(&visitor.etir()[body]);
        }
        FnRef { item: _, arity: _, erl_arity: _ } => {}
        Const { def_id: _ } => {}
        Assign { lhs, rhs } => {
            visitor.visit_expr(&visitor.etir()[rhs]);
            visitor.visit_pat(&visitor.etir()[lhs]);
        }
        Declare { vars: _ } => {}
        Match { scrutinee, ref arms } => {
            visitor.visit_expr(&visitor.etir()[scrutinee]);
            for arm in &**arms {
                visitor.visit_arm(arm);
            }
        }
        Receive { ref arms, ref after } => {
            if let Some(ReceiveAfter { timeout, body }) = *after {
                visitor.visit_expr(&visitor.etir()[body]);
                visitor.visit_expr(&visitor.etir()[timeout]);
            }
            for arm in &**arms {
                visitor.visit_arm(arm);
            }
        }
        TryCatch { body, ref arms, after } => {
            visitor.visit_expr(&visitor.etir()[body]);
            if let Some(after) = after {
                visitor.visit_expr(&visitor.etir()[after]);
            }
            for arm in &**arms {
                visitor.visit_catch_arm(arm);
            }
        }
        Return { value } | Break { target: _, value } => {
            if let Some(value) = value {
                visitor.visit_expr(&visitor.etir()[value]);
            }
        }
        Loop { id: _, body } => visitor.visit_expr(&visitor.etir()[body]),
        Continue { target: _ } => {}
    }
}

pub fn walk_arm<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(visitor: &mut V, arm: &'etir Arm) {
    let Arm { pattern, guard, body, span: _ } = arm;
    visitor.visit_pat(&visitor.etir()[*pattern]);
    if let Some(guard) = guard {
        visitor.visit_expr(&visitor.etir()[*guard]);
    }
    visitor.visit_expr(&visitor.etir()[*body]);
}

pub fn walk_catch_arm<'etir, 'tcx: 'etir, V: Visitor<'etir, 'tcx>>(
    visitor: &mut V,
    arm: &'etir CatchArm,
) {
    let CatchArm { class, reason, stacktrace: _, guard, body, span: _ } = arm;
    visitor.visit_pat(&visitor.etir()[*class]);
    visitor.visit_pat(&visitor.etir()[*reason]);
    if let Some(guard) = guard {
        visitor.visit_expr(&visitor.etir()[*guard]);
    }
    visitor.visit_expr(&visitor.etir()[*body]);
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
                visitor.visit_pat(&visitor.etir()[subpattern]);
            }
        }
        Pin { var: _ } => {}
        Assignee { var: _ } => {}
        TermClass { class: _, subpattern } => visitor.visit_pat(&visitor.etir()[subpattern]),
        Tuple { ref fields } => {
            for &field in &**fields {
                visitor.visit_pat(&visitor.etir()[field]);
            }
        }
        Adt(box AdtPat { repr: _, ref fields, input: _ }) => {
            for &FieldPat { name: _, ty: _, pattern } in fields {
                visitor.visit_pat(&visitor.etir()[pattern]);
            }
        }
        Literal(_) => {}
        Range(box PatRange { lo, hi, end: _, ty: _, input: _ }) => {
            if let Some(lo) = lo {
                visitor.visit_expr(&visitor.etir()[lo]);
            }
            if let Some(hi) = hi {
                visitor.visit_expr(&visitor.etir()[hi]);
            }
        }
        Map { ref fields } => {
            for &(key, _) in &**fields {
                visitor.visit_expr(&visitor.etir()[key]);
            }
            for &(_, pattern) in &**fields {
                visitor.visit_pat(&visitor.etir()[pattern]);
            }
        }
        // A size can only refer to bindings of earlier segments, so it comes before its pattern.
        BinaryPattern { ref segments } => {
            for segment in &**segments {
                let BinarySegmentPat { pattern, ref layout, span: _ } = *segment;
                walk_segment_layout(visitor, layout);
                visitor.visit_pat(&visitor.etir()[pattern]);
            }
        }
        List { ref items, tail } => {
            if let Some(tail) = tail {
                visitor.visit_pat(&visitor.etir()[tail]);
            }
            for &item in &**items {
                visitor.visit_pat(&visitor.etir()[item]);
            }
        }
        Or { ref pats } => {
            for &pat in &**pats {
                visitor.visit_pat(&visitor.etir()[pat]);
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
        BinarySegmentSize::Dynamic(size) => visitor.visit_expr(&visitor.etir()[size]),
        BinarySegmentSize::Fixed(_)
        | BinarySegmentSize::OfValue
        | BinarySegmentSize::Unsized
        | BinarySegmentSize::Encoded(_) => {}
    }
}
