/*!

# typeck

The type checker is responsible for:

1. Determining the type of each expression.
2. Resolving methods and traits.
3. Guaranteeing that most type rules are met. ("Most?", you say, "why most?"
   Well, dear reader, read on.)

The main entry point is [`check_crate()`]. Type checking operates in
several major phases:

1. The collect phase first passes over all items and determines their
   type, without examining their "innards".

2. Variance inference then runs to compute the variance of each parameter.

3. Coherence checks for overlapping or orphaned impls.

4. Finally, the check phase then checks function bodies and so forth.
   Within the check phase, we check each function body one at a time
   (bodies of function expressions are checked as part of the
   containing function). Inference is used to supply types wherever
   they are unknown. The actual checking of a function itself has
   several phases (check, regionck, writeback), as discussed in the
   documentation for the [`check`] module.

The type checker is defined into various submodules which are documented
independently:

- hir_ty_lowering: lowers type-system entities from the [HIR][hir] to the
  [`rustc_middle::ty`] representation.

- collect: computes the types of each top-level item and enters them into
  the `tcx.types` table for later use.

- coherence: enforces coherence rules, builds some tables.

- variance: variance inference

- outlives: outlives inference

- check: walks over function bodies and type checks them, inferring types for
  local variables, type parameters, etc as necessary.

- infer: finds the types to use for each type variable such that
  all subtyping and assignment constraints are met. In essence, the check
  module specifies the constraints, and the infer module solves them.

## Note

This API is completely unstable and subject to change.

*/

// tidy-alphabetical-start
#![feature(default_field_values)]
#![feature(gen_blocks)]
#![feature(iter_intersperse)]
#![feature(never_type)]
#![feature(slice_partition_dedup)]
#![feature(try_blocks)]
#![feature(unwrap_infallible)]
// tidy-alphabetical-end

// These are used by Clippy.
pub mod check;

pub mod autoderef;
mod check_unused;
mod coherence;
mod collect;
mod constrained_generic_params;
mod delegation;
pub mod errors;
pub mod hir_ty_lowering;
pub mod hir_wf_check;
mod impl_wf_check;
mod outlives;
mod variance;

pub use errors::NoVariantNamed;
use rustc_hir as hir;
use rustc_hir::def::{DefKind, Res};
use rustc_middle::mir::interpret::GlobalId;
use rustc_middle::query::Providers;
use rustc_middle::ty::{Const, Ty, TyCtxt};
use rustc_middle::{middle, ty};
use rustc_span::ErrorGuaranteed;
use rustc_trait_selection::traits;

pub use crate::collect::suggest_impl_trait;
use crate::hir_ty_lowering::HirTyLowerer;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SupportedConstBody {
    Literal,
    Composite,
}

fn supported_const_body(tcx: TyCtxt<'_>, def_id: hir::def_id::LocalDefId) -> SupportedConstBody {
    let body = tcx.hir_body_owned_by(def_id);
    if let Some(span) = unsupported_const_value_expr(tcx, body.value) {
        tcx.dcx().emit_err(errors::UnsupportedConstValueExpr { span });
        return SupportedConstBody::Composite;
    }

    match body.value.kind {
        hir::ExprKind::Lit(_)
        | hir::ExprKind::Unary(hir::UnOp::Neg, hir::Expr { kind: hir::ExprKind::Lit(_), .. }) => {
            SupportedConstBody::Literal
        }
        _ => SupportedConstBody::Composite,
    }
}

fn unsupported_const_value_expr(tcx: TyCtxt<'_>, expr: &hir::Expr<'_>) -> Option<rustc_span::Span> {
    match expr.kind {
        hir::ExprKind::Lit(_) => None,

        hir::ExprKind::Unary(hir::UnOp::Neg, inner) => {
            (!matches!(inner.kind, hir::ExprKind::Lit(_))).then_some(expr.span)
        }

        hir::ExprKind::Array(items, tail) => items
            .iter()
            .find_map(|item| unsupported_const_value_expr(tcx, item))
            .or_else(|| match tail {
                hir::ArrayTailExpr::Tail(tail) => unsupported_const_value_expr(tcx, tail),
                hir::ArrayTailExpr::None | hir::ArrayTailExpr::NoneWithError(_) => None,
            }),

        hir::ExprKind::Tup(items) => {
            items.iter().find_map(|item| unsupported_const_value_expr(tcx, item))
        }

        hir::ExprKind::Map(fields, tail) => fields
            .iter()
            .find_map(|field| {
                unsupported_const_value_expr(tcx, field.key)
                    .or_else(|| unsupported_const_value_expr(tcx, field.value))
            })
            .or_else(|| match tail {
                hir::MapTailExpr::None | hir::MapTailExpr::NoneWithError(_) => None,
                hir::MapTailExpr::Base(base) => unsupported_const_value_expr(tcx, base),
            }),

        hir::ExprKind::Path(ref qpath) => {
            let typeck_results = tcx.typeck(expr.hir_id.owner.def_id);
            (!matches!(
                typeck_results.qpath_res(qpath, expr.hir_id),
                Res::Def(DefKind::Const { .. }, def_id) if def_id.is_local()
            ))
            .then_some(expr.span)
        }

        // This wrapper is introduced by lowering and is not a user-authored block.
        hir::ExprKind::DropTemps(inner) => unsupported_const_value_expr(tcx, inner),

        _ => Some(expr.span),
    }
}

/// Adds query implementations to the [Providers] vtable, see [`rustc_middle::query`]
pub fn provide(providers: &mut Providers) {
    collect::provide(providers);
    coherence::provide(providers);
    check::provide(providers);
    *providers = Providers {
        check_unused_traits: check_unused::check_unused_traits,
        diagnostic_hir_wf_check: hir_wf_check::diagnostic_hir_wf_check,
        inferred_outlives_crate: outlives::inferred_outlives_crate,
        inferred_outlives_of: outlives::inferred_outlives_of,
        inherit_sig_for_delegation_item: delegation::inherit_sig_for_delegation_item,
        enforce_impl_non_lifetime_params_are_constrained:
            impl_wf_check::enforce_impl_non_lifetime_params_are_constrained,
        crate_variances: variance::crate_variances,
        variances_of: variance::variances_of,
        ..*providers
    };
}

pub fn check_crate(tcx: TyCtxt<'_>) {
    let _prof_timer = tcx.sess.timer("type_check_crate");

    tcx.sess.time("coherence_checking", || {
        // When discarding query call results, use an explicit type to indicate
        // what we are intending to discard, to help future type-based refactoring.
        type R = Result<(), ErrorGuaranteed>;

        let _: R = tcx.ensure_result().check_type_wf(());

        for &trait_def_id in tcx.all_local_trait_impls(()).keys() {
            let _: R = tcx.ensure_result().coherent_trait(trait_def_id);
        }
        // these queries are executed for side-effects (error reporting):
        let _: R = tcx.ensure_result().crate_inherent_impls_validity_check(());
        let _: R = tcx.ensure_result().crate_inherent_impls_overlap_check(());
    });

    tcx.par_hir_body_owners(|item_def_id| {
        let def_kind = tcx.def_kind(item_def_id);
        // Make sure we evaluate all static and (non-associated) const items, even if unused.
        // If any of these fail to evaluate, we do not want this crate to pass compilation.
        match def_kind {
            DefKind::Static { .. } => {
                tcx.ensure_ok().eval_static_initializer(item_def_id);
                check::maybe_check_static_with_link_section(tcx, item_def_id);
            }
            DefKind::Const { .. }
                if !tcx.generics_of(item_def_id).own_requires_monomorphization()
                    && !tcx.is_type_const(item_def_id) =>
            {
                // FIXME(generic_const_items): Passing empty instead of identity args is fishy but
                //                             seems to be fine for now. Revisit this!
                let instance = ty::Instance::new_raw(item_def_id.into(), ty::GenericArgs::empty());
                if supported_const_body(tcx, item_def_id) == SupportedConstBody::Literal {
                    let cid = GlobalId { instance, promoted: None };
                    let typing_env = ty::TypingEnv::fully_monomorphized();
                    tcx.ensure_ok().eval_to_const_value_raw(typing_env.as_query_input(cid));
                }
            }
            _ => (),
        }
        // Skip `AnonConst`s because we feed their `type_of`.
        // Also skip items for which typeck forwards to parent typeck.
        if !(matches!(def_kind, DefKind::AnonConst) || def_kind.is_typeck_child()) {
            tcx.ensure_ok().typeck(item_def_id);
        }
        // Ensure we generate the new `DefId` before finishing `check_crate`.
        // Afterwards we freeze the list of `DefId`s.
        if tcx.needs_coroutine_by_move_body_def_id(item_def_id.to_def_id()) {
            tcx.ensure_done().coroutine_by_move_body_def_id(item_def_id);
        }
    });

    if tcx.features().rustc_attrs() {
        tcx.sess.time("dumping_rustc_attr_data", || {
            outlives::dump::inferred_outlives(tcx);
            variance::dump::variances(tcx);
            collect::dump::opaque_hidden_types(tcx);
            collect::dump::predicates_and_item_bounds(tcx);
            collect::dump::def_parents(tcx);
            collect::dump::vtables(tcx);
        });
    }

    tcx.ensure_ok().check_unused_traits(());
}

/// Lower a [`hir::Ty`] to a [`Ty`].
///
/// <div class="warning">
///
/// This function is **quasi-deprecated**. It can cause ICEs if called inside of a body
/// (of a function or constant) and especially if it contains inferred types (`_`).
///
/// It's used in rustdoc and Clippy.
///
/// </div>
pub fn lower_ty<'tcx>(tcx: TyCtxt<'tcx>, hir_ty: &hir::Ty<'tcx>) -> Ty<'tcx> {
    // In case there are any projections, etc., find the "environment"
    // def-ID that will be used to determine the traits/predicates in
    // scope. This is derived from the enclosing item-like thing.
    let env_def_id = tcx.hir_get_parent_item(hir_ty.hir_id);
    collect::ItemCtxt::new(tcx, env_def_id.def_id)
        .lowerer()
        .lower_ty_maybe_return_type_notation(hir_ty)
}

/// This is for rustdoc.
// FIXME(const_generics): having special methods for rustdoc in `rustc_hir_analysis` is cursed
pub fn lower_const_arg_for_rustdoc<'tcx>(
    tcx: TyCtxt<'tcx>,
    hir_ct: &hir::ConstArg<'tcx>,
    ty: Ty<'tcx>,
) -> Const<'tcx> {
    let env_def_id = tcx.hir_get_parent_item(hir_ct.hir_id);
    collect::ItemCtxt::new(tcx, env_def_id.def_id).lowerer().lower_const_arg(hir_ct, ty)
}
