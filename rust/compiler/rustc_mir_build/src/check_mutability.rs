use rustc_data_structures::fx::FxHashMap;
use rustc_errors::ErrorGuaranteed;
use rustc_hir::{BindingMode, Mutability};
use rustc_middle::middle::region;
use rustc_middle::span_bug;
use rustc_middle::thir::visit::{self, Visitor};
use rustc_middle::thir::{
    Arm, Expr, ExprId, ExprKind, LocalVarId, Pat, PatKind, Stmt, StmtKind, Thir,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::LocalDefId;
use rustc_span::{Span, Symbol};

use crate::errors::{
    AssignToImmutable, AssignToNonVariable, AssignToUpvar, InconsistentDeferredInitialization,
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

    fn insert(&mut self, var: LocalVarId, info: BindingInfo) {
        self.current_mut().insert(var, info);
    }

    fn lookup(&self, id: LocalVarId) -> Option<(usize, BindingInfo)> {
        self.0
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, frame)| frame.bound.get(&id).copied().map(|info| (index, info)))
    }

    fn set_initialized(&mut self, frame_index: usize, id: LocalVarId) {
        self.0[frame_index].bound.get_mut(&id).unwrap().initialization =
            Initialization::Initialized;
    }

    fn get(&self, frame_index: usize) -> Option<&Frame> {
        self.0.get(frame_index)
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

    fn binding_in(&self, frame_index: usize, id: LocalVarId) -> Option<BindingInfo> {
        self.frames.get(frame_index).and_then(|frame| frame.bound.get(&id)).copied()
    }
}

struct LoopFrame {
    label: region::Scope,
    breaks: Vec<Frames>,
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
    fn visit_arm(&mut self, arm: &'thir Arm<'tcx>) {
        self.visit_pat(&arm.pattern);
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

        for (frame_index, frame) in self.frames.iter_mut().enumerate() {
            for (id, info) in &mut frame.bound {
                let first = first_branch.binding_in(frame_index, *id).unwrap_or(*info);

                info.initialization = if let Some(error) = reachable.iter().find_map(|branch| {
                    let Initialization::Error(error) =
                        branch.binding_in(frame_index, *id).unwrap_or(*info).initialization
                    else {
                        return None;
                    };
                    Some(error)
                }) {
                    Initialization::Error(error)
                } else if rest_branches.iter().any(|branch| {
                    branch.binding_in(frame_index, *id).unwrap_or(*info).initialization
                        != first.initialization
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

    fn peeled_lhs(&self, mut lhs: ExprId) -> &'thir Expr<'tcx> {
        loop {
            let expr = &self.thir[lhs];
            lhs = match expr.kind {
                ExprKind::Scope { value, .. } => value,
                ExprKind::Use { source }
                | ExprKind::ByUse { expr: source, .. }
                | ExprKind::NeverToAny { source } => source,
                _ => return expr,
            };
        }
    }

    fn check_assignment(&mut self, assignment: &Expr<'tcx>, lhs: ExprId) {
        match self.peeled_lhs(lhs).kind {
            ExprKind::VarRef { id } => self.check_var_assignment(assignment.span, id),
            ExprKind::UpvarRef { var_hir_id, .. } => {
                let ident = self.tcx.hir_ident(var_hir_id.0);
                self.tcx.dcx().emit_err(AssignToUpvar {
                    assignment: assignment.span,
                    binding: ident.span,
                    name: ident.name,
                });
            }
            _ => {
                self.tcx.dcx().emit_err(AssignToNonVariable { assignment: assignment.span });
            }
        }
    }

    fn check_read(&self, usage: Span, id: LocalVarId) {
        if id.0.owner.def_id != self.owner {
            // Error recovery can leave a closure-local id in its parent's capture list.
            return;
        }
        let Some((_, info)) = self.frames.lookup(id) else {
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
        let Some((frame_index, info)) = self.frames.lookup(id) else {
            span_bug!(assignment, "assignment to unknown THIR local {id:?}")
        };

        if matches!(info.initialization, Initialization::Error(_)) {
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
            self.frames.set_initialized(frame_index, id);
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
                self.visit_expr(&self.thir[*cond]);
                let mut branches = vec![self.visit_expr_branch(*then)];
                branches.push(match else_opt {
                    Some(otherwise) => self.visit_expr_branch(*otherwise),
                    None => Branch::continuing(self.frames.clone()),
                });
                self.merge_branches(expr.span, &branches);
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
                self.check_assignment(expr, *lhs);
            }
            ExprKind::AssignOp { lhs, rhs, .. } => {
                self.visit_expr(&self.thir[*lhs]);
                self.visit_expr(&self.thir[*rhs]);
                self.check_assignment(expr, *lhs);
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
