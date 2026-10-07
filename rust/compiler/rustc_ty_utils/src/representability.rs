use rustc_abi::VariantIdx;
use rustc_data_structures::fx::{FxHashSet, FxIndexSet};
use rustc_data_structures::graph::vec_graph::VecGraph;
use rustc_data_structures::work_queue::WorkQueue;
use rustc_hir::attrs::ReprAttr;
use rustc_hir::def::DefKind;
use rustc_index::bit_set::DenseBitSet;
use rustc_middle::query::Providers;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::{DefId, LocalDefId, LocalDefIdMap};

struct Solver {
    ids: FxIndexSet<LocalDefId>,
    /// The current [`params_in_repr`] summary of each node.
    sets: Vec<DenseBitSet<u32>>,
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers {
        recursive_transparent_variants,
        crate_params_in_repr,
        params_in_repr,
        ..*providers
    };
}

/// Returns the variants of `def_id` whose representation would be recursive if they were
/// transparent, that is, the ones whose payload leads back to `def_id` itself.
fn recursive_transparent_variants(tcx: TyCtxt<'_>, def_id: DefId) -> &[VariantIdx] {
    let mut seen = FxHashSet::default();
    tcx.arena.alloc_from_iter(potentially_transparent_payloads(tcx, def_id).filter_map(
        |(variant, payload_ty)| {
            seen.clear();
            representation_reaches_adt(tcx, payload_ty, def_id, &mut seen).then_some(variant)
        },
    ))
}

/// Whether the outer representation of `ty` can be `root`'s own, directly or by way of the
/// payloads and generic arguments it is made of.
fn representation_reaches_adt<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    root: DefId,
    seen: &mut FxHashSet<DefId>,
) -> bool {
    let ty::Adt(adt, args) = ty.kind() else { return false };
    if adt.did() == root {
        return true;
    }
    let params_in_repr = tcx.params_in_repr(adt.did());
    for (i, arg) in args.iter().enumerate() {
        if let ty::GenericArgKind::Type(ty) = arg.kind()
            && params_in_repr.contains(i as u32)
            && representation_reaches_adt(tcx, ty, root, seen)
        {
            return true;
        }
    }

    if !seen.insert(adt.did()) {
        return false;
    }

    potentially_transparent_payloads(tcx, adt.did())
        .any(|(_, payload_ty)| representation_reaches_adt(tcx, payload_ty, root, seen))
}

fn params_in_repr(tcx: TyCtxt<'_>, def_id: LocalDefId) -> DenseBitSet<u32> {
    tcx.crate_params_in_repr(())[&def_id].clone()
}

fn crate_params_in_repr(tcx: TyCtxt<'_>, (): ()) -> LocalDefIdMap<DenseBitSet<u32>> {
    let ids: FxIndexSet<_> = tcx
        .hir_crate_items(())
        .definitions()
        .filter(|&id| matches!(tcx.def_kind(id), DefKind::Struct | DefKind::Union | DefKind::Enum))
        .collect();

    let mut edges = Vec::new();
    for (index, &id) in ids.iter().enumerate() {
        for (_, payload_ty) in potentially_transparent_payloads(tcx, id.to_def_id()) {
            add_edges(payload_ty, index, &ids, &mut edges);
        }
    }

    let dependents = VecGraph::<usize>::new(ids.len(), edges);
    let sets = ids
        .iter()
        .map(|&id| DenseBitSet::new_empty(tcx.generics_of(id).own_params.len()))
        .collect();
    let mut solver = Solver { ids, sets };

    let mut worklist = WorkQueue::with_none(solver.ids.len());
    for index in 0..solver.ids.len() {
        worklist.insert(index);
    }
    while let Some(index) = worklist.pop() {
        let mut params_in_repr = DenseBitSet::new_empty(solver.sets[index].domain_size());
        for (_, payload_ty) in potentially_transparent_payloads(tcx, solver.ids[index].to_def_id())
        {
            params_in_repr_ty(tcx, payload_ty, &solver, &mut params_in_repr);
        }

        if params_in_repr != solver.sets[index] {
            solver.sets[index] = params_in_repr;
            for &d in dependents.successors(index) {
                worklist.insert(d);
            }
        }
    }
    solver.ids.iter().copied().zip(solver.sets).collect()
}

fn params_in_repr_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    solver: &Solver,
    params_in_repr: &mut DenseBitSet<u32>,
) {
    match *ty.kind() {
        ty::Adt(adt, args) => {
            // Summaries of ADTs from dependencies are already final, and cannot lead back here.
            let inner_params_in_repr = match adt.did().as_local() {
                Some(local) => &solver.sets[solver.ids.get_index_of(&local).unwrap()],
                None => tcx.params_in_repr(adt.did()),
            };
            for (i, arg) in args.iter().enumerate() {
                if let ty::GenericArgKind::Type(ty) = arg.kind() {
                    if inner_params_in_repr.contains(i as u32) {
                        params_in_repr_ty(tcx, ty, solver, params_in_repr);
                    }
                }
            }
        }
        ty::Param(param) => {
            params_in_repr.insert(param.index);
        }
        _ => {}
    }
}

/// Add dependencies on local ADTs named by `ty` or its nested generic arguments.
/// Tuple, list and map contents are not inspected.
fn add_edges(
    ty: Ty<'_>,
    from: usize,
    ids: &FxIndexSet<LocalDefId>,
    edges: &mut Vec<(usize, usize)>,
) {
    if let ty::Adt(adt, args) = ty.kind() {
        if let Some(local) = adt.did().as_local()
            && let Some(target) = ids.get_index_of(&local)
        {
            edges.push((target, from));
        }
        for arg in args.iter().filter_map(|arg| arg.as_type()) {
            add_edges(arg, from, ids, edges);
        }
    }
}

/// Variants of `def_id` that have a single field and no conflicting `#[repr(...)]`, and are
/// therefore free to be transparent. Each is paired with the identity-instantiated payload
/// type it would then adopt.
fn potentially_transparent_payloads<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
) -> impl Iterator<Item = (VariantIdx, Ty<'tcx>)> + 'tcx {
    tcx.adt_def(def_id).variants().iter_enumerated().filter_map(move |(index, variant)| {
        let (_, field) = tcx.erl_single_runtime_field(variant)?;
        match tcx.effective_erl_repr_kind_attr(variant.def_id) {
            None | Some((ReprAttr::Patina | ReprAttr::Transparent, _)) => {
                Some((index, tcx.type_of(field.did).instantiate_identity().skip_norm_wip()))
            }
            _ => None,
        }
    })
}
