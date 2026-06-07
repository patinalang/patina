use std::fmt::Write;
use std::iter;

use rustc_data_structures::fx::FxHashSet;
use rustc_errors::codes::*;
use rustc_errors::{Applicability, Diag, MultiSpan, pluralize, struct_span_code_err};
use rustc_hir as hir;
use rustc_hir::def::{DefKind, Res};
use rustc_middle::bug;
use rustc_middle::queries::TaggedQueryKey;
use rustc_middle::query::Cycle;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::{DefId, LocalDefId};
use rustc_span::{ErrorGuaranteed, Span};

// Default cycle handler used for all queries that don't use the `handle_cycle_error` query
// modifier.
pub(crate) fn default(err: Diag<'_>) -> ! {
    let guar = err.emit();
    guar.raise_fatal()
}

pub(crate) fn fn_sig<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    _: Cycle<'tcx>,
    err: Diag<'_>,
) -> ty::EarlyBinder<'tcx, ty::PolyFnSig<'tcx>> {
    let guar = err.delay_as_bug();

    let err = Ty::new_error(tcx, guar);

    let arity = if let Some(node) = tcx.hir_get_if_local(def_id)
        && let Some(sig) = node.fn_sig()
    {
        sig.decl.inputs.len()
    } else {
        tcx.dcx().abort_if_errors();
        unreachable!()
    };

    ty::EarlyBinder::bind(ty::Binder::dummy(
        tcx.mk_fn_sig_safe_rust_abi(std::iter::repeat_n(err, arity), err),
    ))
}

pub(crate) fn check_representability<'tcx>(
    tcx: TyCtxt<'tcx>,
    _key: LocalDefId,
    cycle: Cycle<'tcx>,
    _err: Diag<'_>,
) {
    check_representability_inner(tcx, cycle);
}

pub(crate) fn check_representability_adt_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    _key: Ty<'tcx>,
    cycle: Cycle<'tcx>,
    _err: Diag<'_>,
) {
    check_representability_inner(tcx, cycle);
}

fn check_representability_inner<'tcx>(tcx: TyCtxt<'tcx>, cycle: Cycle<'tcx>) -> ! {
    let mut item_and_field_ids = Vec::new();
    let mut representable_ids = FxHashSet::default();
    for frame in &cycle.frames {
        if let TaggedQueryKey::check_representability(def_id) = frame.tagged_key
            && tcx.def_kind(def_id) == DefKind::Field
        {
            let field_id: LocalDefId = def_id;
            let parent_id = tcx.parent(field_id.to_def_id());
            let item_id = match tcx.def_kind(parent_id) {
                DefKind::Variant => tcx.parent(parent_id),
                _ => parent_id,
            };
            item_and_field_ids.push((item_id.expect_local(), field_id));
        }
    }
    for frame in &cycle.frames {
        if let TaggedQueryKey::check_representability_adt_ty(key) = frame.tagged_key
            && let Some(adt) = key.ty_adt_def()
            && let Some(def_id) = adt.did().as_local()
            && !item_and_field_ids.iter().any(|&(id, _)| id == def_id)
        {
            representable_ids.insert(def_id);
        }
    }
    // We used to continue here, but the cycle error printed next is actually less useful than
    // the error produced by `recursive_type_error`.
    let guar = recursive_type_error(tcx, item_and_field_ids, &representable_ids);
    guar.raise_fatal()
}

pub(crate) fn variances_of<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    _cycle: Cycle<'tcx>,
    err: Diag<'_>,
) -> &'tcx [ty::Variance] {
    let _guar = err.delay_as_bug();
    let n = tcx.generics_of(def_id).own_params.len();
    tcx.arena.alloc_from_iter(iter::repeat_n(ty::Bivariant, n))
}

// item_and_field_ids should form a cycle where each field contains the
// type in the next element in the list
fn recursive_type_error(
    tcx: TyCtxt<'_>,
    mut item_and_field_ids: Vec<(LocalDefId, LocalDefId)>,
    representable_ids: &FxHashSet<LocalDefId>,
) -> ErrorGuaranteed {
    const ITEM_LIMIT: usize = 5;

    // Rotate the cycle so that the item with the lowest span is first
    let start_index = item_and_field_ids
        .iter()
        .enumerate()
        .min_by_key(|&(_, &(id, _))| tcx.def_span(id))
        .unwrap()
        .0;
    item_and_field_ids.rotate_left(start_index);

    let cycle_len = item_and_field_ids.len();
    let show_cycle_len = cycle_len.min(ITEM_LIMIT);

    let mut err_span = MultiSpan::from_spans(
        item_and_field_ids[..show_cycle_len]
            .iter()
            .map(|(id, _)| tcx.def_span(id.to_def_id()))
            .collect(),
    );
    let mut suggestion = Vec::with_capacity(show_cycle_len * 2);
    for i in 0..show_cycle_len {
        let (_, field_id) = item_and_field_ids[i];
        let (next_item_id, _) = item_and_field_ids[(i + 1) % cycle_len];
        // Find the span(s) that contain the next item in the cycle
        let hir::Node::Field(field) = tcx.hir_node_by_def_id(field_id) else {
            bug!("expected field")
        };
        let mut found = Vec::new();
        find_item_ty_spans(tcx, field.ty, next_item_id, &mut found, representable_ids);

        // Couldn't find the type. Maybe it's behind a type alias?
        // In any case, we'll just suggest boxing the whole field.
        if found.is_empty() {
            found.push(field.ty.span);
        }

        for span in found {
            err_span.push_span_label(span, "recursive without indirection");
            // FIXME(compiler-errors): This suggestion might be erroneous if Box is shadowed
            suggestion.push((span.shrink_to_lo(), "Box<".to_string()));
            suggestion.push((span.shrink_to_hi(), ">".to_string()));
        }
    }
    let items_list = {
        let mut s = String::new();
        for (i, &(item_id, _)) in item_and_field_ids.iter().enumerate() {
            let path = tcx.def_path_str(item_id);
            write!(&mut s, "`{path}`").unwrap();
            if i == (ITEM_LIMIT - 1) && cycle_len > ITEM_LIMIT {
                write!(&mut s, " and {} more", cycle_len - 5).unwrap();
                break;
            }
            if cycle_len > 1 && i < cycle_len - 2 {
                s.push_str(", ");
            } else if cycle_len > 1 && i == cycle_len - 2 {
                s.push_str(" and ")
            }
        }
        s
    };
    struct_span_code_err!(
        tcx.dcx(),
        err_span,
        E0072,
        "recursive type{} {} {} infinite size",
        pluralize!(cycle_len),
        items_list,
        pluralize!("has", cycle_len),
    )
    .with_multipart_suggestion(
        "insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle",
        suggestion,
        Applicability::HasPlaceholders,
    )
    .emit()
}

fn find_item_ty_spans(
    tcx: TyCtxt<'_>,
    ty: &hir::Ty<'_>,
    needle: LocalDefId,
    spans: &mut Vec<Span>,
    seen_representable: &FxHashSet<LocalDefId>,
) {
    match ty.kind {
        hir::TyKind::Path(hir::QPath::Resolved(_, path)) => {
            if let Res::Def(kind, def_id) = path.res
                && matches!(kind, DefKind::Enum | DefKind::Struct | DefKind::Union)
            {
                let check_params = def_id.as_local().is_none_or(|def_id| {
                    if def_id == needle {
                        spans.push(ty.span);
                    }
                    seen_representable.contains(&def_id)
                });
                if check_params && let Some(args) = path.segments.last().unwrap().args {
                    let params_in_repr = tcx.params_in_repr(def_id);
                    // the domain size check is needed because the HIR may not be well-formed at this point
                    for (i, arg) in args.args.iter().enumerate().take(params_in_repr.domain_size())
                    {
                        if let hir::GenericArg::Type(ty) = arg
                            && params_in_repr.contains(i as u32)
                        {
                            find_item_ty_spans(
                                tcx,
                                ty.as_unambig_ty(),
                                needle,
                                spans,
                                seen_representable,
                            );
                        }
                    }
                }
            }
        }
        hir::TyKind::Array(ty, _) => find_item_ty_spans(tcx, ty, needle, spans, seen_representable),
        hir::TyKind::Tup(tys) => {
            tys.iter().for_each(|ty| find_item_ty_spans(tcx, ty, needle, spans, seen_representable))
        }
        _ => {}
    }
}
