use rustc_data_structures::fx::FxHashMap;
use rustc_errors::ErrorGuaranteed;
use rustc_hir::{BindingMode, Mutability};
use rustc_middle::middle::region;
use rustc_middle::span_bug;
use rustc_middle::thir::visit::{self, Visitor};
use rustc_middle::thir::{
    Arm, ArmId, Expr, ExprId, ExprKind, LocalVarId, LogicalOp, Pat, PatKind, Stmt, StmtKind, Thir,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::LocalDefId;
use rustc_span::{Span, Symbol};

use crate::errors::{
    AssignAcrossTryBoundary, AssignToImmutable, AssignToUpvar, InconsistentDeferredInitialization,
    UseOfUninitialized,
};

#[derive(Clone, Copy)]
struct BindingInfo {
    span: Span,
    name: Symbol,
    mutability: Mutability,
    initialization: Initialization,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Initialization {
    Uninitialized,
    Initialized,
    Error(ErrorGuaranteed),
}

#[derive(Clone, Default)]
struct Frame {
    bound: FxHashMap<LocalVarId, BindingInfo>,
    boundary: Option<Boundary>,
}

impl Frame {
    fn insert(&mut self, var: LocalVarId, info: BindingInfo) {
        self.bound.insert(var, info);
    }
}

#[derive(Clone)]
struct Frames(Vec<Frame>);

impl Frames {
    fn new() -> Frames {
        Frames(vec![Frame::default()])
    }

    fn current_mut(&mut self) -> &mut Frame {
        self.0.last_mut().unwrap()
    }

    fn push_boundary(&mut self, boundary: Boundary) {
        self.0.push(Frame { bound: FxHashMap::default(), boundary: Some(boundary) });
    }

    fn pop_boundary(&mut self) {
        self.0.pop();
    }

    fn current_boundary(&self) -> Option<Boundary> {
        self.0.last().and_then(|frame| frame.boundary)
    }

    fn insert(&mut self, var: LocalVarId, info: BindingInfo) {
        self.current_mut().insert(var, info);
    }

    fn lookup(&self, id: LocalVarId) -> Option<BindingInfo> {
        self.0.iter().rev().find_map(|frame| frame.bound.get(&id).copied())
    }

    fn is_bound_in_current_frame(&self, id: LocalVarId) -> bool {
        self.0.last().unwrap().bound.contains_key(&id)
    }

    fn set_initialized(&mut self, id: LocalVarId) {
        self.current_mut().bound.get_mut(&id).unwrap().initialization = Initialization::Initialized;
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Frame> {
        self.0.iter_mut()
    }
}

struct Branch {
    frames: Frames,
    diverges: bool,
}

impl Branch {
    fn continuing(frames: Frames) -> Branch {
        Branch { frames, diverges: false }
    }

    fn binding(&self, id: LocalVarId) -> Option<BindingInfo> {
        self.frames.lookup(id)
    }
}

struct LoopFrame {
    label: region::Scope,
    breaks: Vec<Frames>,
}

#[derive(Clone, Copy)]
enum Boundary {
    Try,
    Catch,
    After,
}

impl Boundary {
    fn name(self) -> &'static str {
        match self {
            Boundary::Try => "try",
            Boundary::Catch => "catch",
            Boundary::After => "after",
        }
    }
}

pub(crate) fn check_mutability(tcx: TyCtxt<'_>, def: LocalDefId) {
    let Ok((thir, expr)) = tcx.thir_body(def) else { return };
    let thir = thir.borrow();
    let mut visitor = MutabilityVisitor {
        tcx,
        owner: def,
        thir: &thir,
        frames: Frames::new(),
        loops: Vec::new(),
        pattern_initialized: true,
    };

    for param in &thir.params {
        if let Some(param_pat) = param.pat.as_deref() {
            visitor.visit_pat(param_pat);
        }
    }
    visitor.visit_expr(&thir[expr]);
}

struct MutabilityVisitor<'thir, 'tcx> {
    tcx: TyCtxt<'tcx>,
    owner: LocalDefId,
    thir: &'thir Thir<'tcx>,
    frames: Frames,
    loops: Vec<LoopFrame>,
    pattern_initialized: bool,
}

impl<'thir, 'tcx> MutabilityVisitor<'thir, 'tcx> {
    fn with_boundary(&mut self, boundary: Boundary, f: impl FnOnce(&mut Self)) {
        self.frames.push_boundary(boundary);
        f(self);
        self.frames.pop_boundary();
    }

    fn visit_boundary_expr(&mut self, boundary: Boundary, expr: ExprId) {
        self.with_boundary(boundary, |this| this.visit_expr(&this.thir[expr]));
    }

    fn visit_catch(&mut self, arms: &[ArmId]) {
        self.with_boundary(Boundary::Catch, |this| {
            for arm in arms {
                let arm = &this.thir[*arm];
                this.capture_branch(|this| this.visit_arm(arm));
            }
        });
    }

    fn visit_arm(&mut self, arm: &'thir Arm<'tcx>) {
        self.visit_pat(&arm.pattern);
        if let Some(stacktrace) = arm.stacktrace.as_deref() {
            self.visit_pat(stacktrace);
        }
        if let Some(guard) = arm.guard {
            self.visit_expr(&self.thir[guard]);
        }
        self.visit_expr(&self.thir[arm.body]);
    }

    fn capture_branch<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> (Frames, T) {
        let incoming = self.frames.clone();
        let result = f(self);
        let outgoing = std::mem::replace(&mut self.frames, incoming);
        (outgoing, result)
    }

    fn visit_expr_branch(&mut self, expr: ExprId) -> Branch {
        let (frames, _) = self.capture_branch(|s| {
            s.visit_expr(&s.thir[expr]);
        });
        Branch { frames, diverges: self.expr_always_diverges(expr) }
    }

    fn visit_arm_branch(&mut self, arm: &'thir Arm<'tcx>) -> Branch {
        let (frames, _) = self.capture_branch(|s| {
            s.visit_arm(arm);
        });
        Branch { frames, diverges: self.expr_always_diverges(arm.body) }
    }

    /// The operands of `expr`'s `&&` chain, in order.
    /// We don't care about `||` because `let` can only happen as an operand of `&&`.
    fn gather_and_operands(&self, mut expr: ExprId, operands: &mut Vec<ExprId>) {
        while let ExprKind::Scope { value, .. } = self.thir[expr].kind {
            expr = value;
        }

        if let ExprKind::LogicalOp { op: LogicalOp::And, lhs, rhs } = self.thir[expr].kind {
            self.gather_and_operands(lhs, operands);
            self.gather_and_operands(rhs, operands);
        } else {
            operands.push(expr);
        }
    }

    fn visit_loop(&mut self, span: Span, label: region::Scope, body: ExprId) {
        let (_, loop_frame) = self.capture_branch(|s| {
            s.loops.push(LoopFrame { label, breaks: Vec::new() });
            s.visit_expr(&s.thir[body]);
            s.loops.pop().unwrap()
        });

        let branches = loop_frame.breaks.into_iter().map(Branch::continuing).collect::<Vec<_>>();

        self.merge_branches(span, &branches);
    }

    fn merge_branches(&mut self, branch_span: Span, branches: &[Branch]) {
        let reachable = branches.iter().filter(|branch| !branch.diverges).collect::<Vec<_>>();

        let Some((first_branch, rest_branches)) = reachable.split_first() else { return };

        for frame in self.frames.iter_mut() {
            for (id, info) in &mut frame.bound {
                let first = first_branch.binding(*id).unwrap_or(*info);

                info.initialization = if let Some(error) = reachable.iter().find_map(|branch| {
                    let Initialization::Error(error) =
                        branch.binding(*id).unwrap_or(*info).initialization
                    else {
                        return None;
                    };
                    Some(error)
                }) {
                    Initialization::Error(error)
                } else if rest_branches.iter().any(|branch| {
                    branch.binding(*id).unwrap_or(*info).initialization != first.initialization
                }) {
                    let error = self.tcx.dcx().emit_err(InconsistentDeferredInitialization {
                        branch: branch_span,
                        binding: info.span,
                        name: info.name,
                    });

                    Initialization::Error(error)
                } else {
                    first.initialization
                }
            }
        }
    }

    fn expr_always_diverges(&self, mut expr: ExprId) -> bool {
        let mut hir_id = None;
        loop {
            expr = match self.thir[expr].kind {
                ExprKind::Scope { value, hir_id: id, .. } => {
                    hir_id = Some(id);
                    value
                }
                ExprKind::Use { source }
                | ExprKind::ByUse { expr: source, .. }
                | ExprKind::NeverToAny { source }
                | ExprKind::PointerCoercion { source, .. }
                | ExprKind::PlaceTypeAscription { source, .. }
                | ExprKind::ValueTypeAscription { source, .. } => source,
                _ => break,
            };
        }

        hir_id.is_some_and(|hir_id| self.tcx.typeck(self.owner).is_always_diverging_expr(hir_id))
    }

    fn check_assignee(&mut self, assignment: Span, var: LocalVarId) {
        if self.tcx.upvars_mentioned(self.owner).is_some_and(|upvars| upvars.contains_key(&var.0)) {
            let ident = self.tcx.hir_ident(var.0);
            self.tcx.dcx().emit_err(AssignToUpvar {
                assignment,
                binding: ident.span,
                name: ident.name,
            });
        } else {
            self.check_var_assignment(assignment, var);
        }
    }

    fn check_read(&self, usage: Span, id: LocalVarId) {
        if id.0.owner.def_id != self.owner {
            // Error recovery can leave a closure-local id in its parent's capture list.
            return;
        }
        let Some(info) = self.frames.lookup(id) else {
            span_bug!(usage, "read from unknown THIR local {id:?}")
        };
        if info.initialization == Initialization::Uninitialized {
            self.tcx.dcx().emit_err(UseOfUninitialized {
                usage,
                binding: info.span,
                name: info.name,
            });
        }
    }

    fn check_var_assignment(&mut self, assignment: Span, id: LocalVarId) {
        let Some(info) = self.frames.lookup(id) else {
            span_bug!(assignment, "assignment to unknown THIR local {id:?}")
        };

        if !self.frames.is_bound_in_current_frame(id) {
            let boundary = self.frames.current_boundary().unwrap_or_else(|| {
                span_bug!(assignment, "assignment crossed a non-boundary mutability frame")
            });
            self.tcx.dcx().emit_err(AssignAcrossTryBoundary {
                assignment,
                binding: info.span,
                name: info.name,
                boundary: boundary.name(),
            });
        } else if matches!(info.initialization, Initialization::Error(_)) {
            // The inconsistent branch diagnostic is enough; avoid cascading errors.
        } else if info.mutability == Mutability::Not
            && info.initialization == Initialization::Initialized
        {
            self.tcx.dcx().emit_err(AssignToImmutable {
                assignment,
                binding: info.span,
                name: info.name,
            });
        } else {
            self.frames.set_initialized(id);
        }
    }
}

impl<'thir, 'tcx> Visitor<'thir, 'tcx> for MutabilityVisitor<'thir, 'tcx> {
    fn thir(&self) -> &'thir Thir<'tcx> {
        self.thir
    }

    fn visit_pat(&mut self, pat: &'thir Pat<'tcx>) {
        if let PatKind::Binding { name, mode: BindingMode(_, mutability), var, is_primary, .. } =
            pat.kind
            && is_primary
        {
            let info = BindingInfo {
                span: pat.span,
                name,
                mutability,
                initialization: if self.pattern_initialized {
                    Initialization::Initialized
                } else {
                    Initialization::Uninitialized
                },
            };
            self.frames.insert(var, info);
        } else if let PatKind::Pin { var } = pat.kind {
            self.check_read(pat.span, var);
        } else if let PatKind::Assignee { var } = pat.kind {
            self.check_assignee(pat.span, var);
        }
        visit::walk_pat(self, pat);
    }

    fn visit_stmt(&mut self, stmt: &'thir Stmt<'tcx>) {
        let StmtKind::Let { pattern, initializer, else_block, .. } = &stmt.kind else {
            return visit::walk_stmt(self, stmt);
        };

        if let Some(initializer) = initializer {
            self.visit_expr(&self.thir[*initializer]);
        }

        let before_pattern = self.frames.clone();
        let old = self.pattern_initialized;
        self.pattern_initialized = initializer.is_some();
        self.visit_pat(pattern);
        self.pattern_initialized = old;

        if let Some(else_block) = else_block {
            let success = std::mem::replace(&mut self.frames, before_pattern);
            self.visit_block(&self.thir[*else_block]);
            self.frames = success;
        }
    }

    fn visit_expr(&mut self, expr: &'thir Expr<'tcx>) {
        match &expr.kind {
            ExprKind::Scope { region_scope, value, .. } => {
                let mut label = *region_scope;
                let mut value = *value;
                while let ExprKind::Scope { region_scope, value: inner, .. } = self.thir[value].kind
                {
                    label = region_scope;
                    value = inner;
                }
                if let ExprKind::Loop { body } = self.thir[value].kind {
                    self.visit_loop(expr.span, label, body);
                } else {
                    self.visit_expr(&self.thir[value]);
                }
            }
            ExprKind::If { cond, then, else_opt, .. } => {
                let mut operands = Vec::new();
                self.gather_and_operands(*cond, &mut operands);

                let (then_frames, failures) = self.capture_branch(|s| {
                    let failures = operands
                        .iter()
                        .map(|&operand| {
                            s.visit_expr(&s.thir[operand]);
                            Branch {
                                frames: s.frames.clone(),
                                diverges: s.expr_always_diverges(operand),
                            }
                        })
                        .collect::<Vec<_>>();

                    s.visit_expr(&s.thir[*then]);
                    failures
                });

                let then_branch =
                    Branch { frames: then_frames, diverges: self.expr_always_diverges(*then) };

                self.merge_branches(expr.span, &failures);

                let else_branch = match else_opt {
                    Some(otherwise) => self.visit_expr_branch(*otherwise),
                    None => Branch::continuing(self.frames.clone()),
                };

                self.merge_branches(expr.span, &[then_branch, else_branch]);
            }
            ExprKind::LogicalOp { lhs, rhs, .. } => {
                self.visit_expr(&self.thir[*lhs]);
                let branches =
                    [self.visit_expr_branch(*rhs), Branch::continuing(self.frames.clone())];
                self.merge_branches(expr.span, &branches);
            }
            ExprKind::Match { scrutinee, arms, .. } => {
                self.visit_expr(&self.thir[*scrutinee]);
                let branches = arms
                    .iter()
                    .map(|arm| self.visit_arm_branch(&self.thir[*arm]))
                    .collect::<Vec<_>>();
                self.merge_branches(expr.span, &branches);
            }
            ExprKind::Receive { arms, timeout } => {
                if let Some(timeout) = timeout {
                    self.visit_expr(&self.thir[timeout.expr]);
                }
                let mut branches = arms
                    .iter()
                    .map(|arm| self.visit_arm_branch(&self.thir[*arm]))
                    .collect::<Vec<_>>();
                if let Some(timeout) = timeout {
                    branches.push(self.visit_expr_branch(timeout.body));
                }
                self.merge_branches(expr.span, &branches);
            }
            ExprKind::TryCatch { body, arms, after } => {
                self.visit_boundary_expr(Boundary::Try, *body);
                self.visit_catch(arms);
                if let Some(after) = after {
                    self.visit_boundary_expr(Boundary::After, *after);
                }
            }
            ExprKind::Break { label, value } => {
                if let Some(value) = value {
                    self.visit_expr(&self.thir[*value]);
                }
                if let Some(loop_frame) =
                    self.loops.iter_mut().rev().find(|loop_frame| loop_frame.label == *label)
                {
                    loop_frame.breaks.push(self.frames.clone());
                }
            }
            ExprKind::Assign { lhs, rhs } => {
                self.visit_expr(&self.thir[*rhs]);
                self.visit_pat(lhs);
            }
            ExprKind::VarRef { id } => self.check_read(expr.span, *id),
            ExprKind::Closure(closure) => {
                for upvar in &closure.upvars {
                    self.visit_expr(&self.thir[*upvar]);
                }
            }
            _ => visit::walk_expr(self, expr),
        }
    }
}
