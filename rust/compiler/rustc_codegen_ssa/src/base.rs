use std::sync::Arc;

use rustc_data_structures::unord::UnordMap;
use rustc_hir::attrs::OptimizeAttr;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_hir::find_attr;
use rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs;
use rustc_middle::middle::exported_symbols;
use rustc_middle::query::Providers;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::{self};

use crate::{CodegenLintLevels, CrateInfo};

impl CrateInfo {
    pub fn new(tcx: TyCtxt<'_>, target_cpu: String) -> CrateInfo {
        let crate_types = tcx.crate_types().to_vec();
        let exported_symbols = crate_types.iter().map(|&c| (c, Vec::new())).collect();
        let linked_symbols = crate_types.iter().map(|&c| (c, Vec::new())).collect();
        let local_crate_name = tcx.crate_name(LOCAL_CRATE);
        let windows_subsystem = find_attr!(tcx, crate, WindowsSubsystem(kind) => *kind);

        // This list is used when generating the command line to pass through to
        // system linker. The linker expects undefined symbols on the left of the
        // command line to be defined in libraries on the right, not the other way
        // around. For more info, see some comments in the add_used_library function
        // below.
        //
        // In order to get this left-to-right dependency ordering, we use the reverse
        // postorder of all crates putting the leaves at the rightmost positions.
        let mut compiler_builtins = None;
        let mut used_crates: Vec<_> = tcx
            .postorder_cnums(())
            .iter()
            .rev()
            .copied()
            .filter(|&cnum| {
                let link = !tcx.crate_dep_kind(cnum).macros_only();
                if link && tcx.is_compiler_builtins(cnum) {
                    compiler_builtins = Some(cnum);
                    return false;
                }
                link
            })
            .collect();
        // `compiler_builtins` are always placed last to ensure that they're linked correctly.
        used_crates.extend(compiler_builtins);

        let crates = tcx.crates(());
        let n_crates = crates.len();
        let mut info = CrateInfo {
            target_cpu,
            target_features: tcx.global_backend_features(()).clone(),
            crate_types,
            exported_symbols,
            linked_symbols,
            local_crate_name,
            compiler_builtins,
            profiler_runtime: None,
            is_no_builtins: Default::default(),
            native_libraries: Default::default(),
            used_libraries: tcx.native_libraries(LOCAL_CRATE).iter().map(Into::into).collect(),
            crate_name: UnordMap::with_capacity(n_crates),
            used_crates,
            used_crate_source: UnordMap::with_capacity(n_crates),
            dependency_formats: Arc::clone(tcx.dependency_formats(())),
            windows_subsystem,
            natvis_debugger_visualizers: Default::default(),
            lint_levels: CodegenLintLevels::from_tcx(tcx),
            metadata_symbol: exported_symbols::metadata_symbol_name(tcx),
        };

        info.native_libraries.reserve(n_crates);

        for &cnum in crates.iter() {
            info.native_libraries
                .insert(cnum, tcx.native_libraries(cnum).iter().map(Into::into).collect());
            info.crate_name.insert(cnum, tcx.crate_name(cnum));

            let used_crate_source = tcx.used_crate_source(cnum);
            info.used_crate_source.insert(cnum, Arc::clone(used_crate_source));
            if tcx.is_profiler_runtime(cnum) {
                info.profiler_runtime = Some(cnum);
            }
            if tcx.is_no_builtins(cnum) {
                info.is_no_builtins.insert(cnum);
            }
        }

        info
    }
}

pub(crate) fn provide(providers: &mut Providers) {
    providers.backend_optimization_level = |tcx, cratenum| {
        let for_speed = match tcx.sess.opts.optimize {
            // If globally no optimisation is done, #[optimize] has no effect.
            //
            // This is done because if we ended up "upgrading" to `-O2` here, we’d populate the
            // pass manager and it is likely that some module-wide passes (such as inliner or
            // cross-function constant propagation) would ignore the `optnone` annotation we put
            // on the functions, thus necessarily involving these functions into optimisations.
            config::OptLevel::No => return config::OptLevel::No,
            // If globally optimise-speed is already specified, just use that level.
            config::OptLevel::Less => return config::OptLevel::Less,
            config::OptLevel::More => return config::OptLevel::More,
            config::OptLevel::Aggressive => return config::OptLevel::Aggressive,
            // If globally optimize-for-size has been requested, use -O2 instead (if optimize(size)
            // are present).
            config::OptLevel::Size => config::OptLevel::More,
            config::OptLevel::SizeMin => config::OptLevel::More,
        };

        let defids = tcx.collect_and_partition_mono_items(cratenum).all_mono_items;

        let any_for_speed = defids.items().any(|id| {
            let CodegenFnAttrs { optimize, .. } = tcx.codegen_fn_attrs(*id);
            matches!(optimize, OptimizeAttr::Speed)
        });

        if any_for_speed {
            return for_speed;
        }

        tcx.sess.opts.optimize
    };
}
