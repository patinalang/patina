use rustc_data_structures::sharded::ShardedHashMap;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::definitions::DefPathData;
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;

pub(super) struct ModuleNameCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    names: ShardedHashMap<DefId, Symbol>,
}

impl<'tcx> ModuleNameCollector<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> ModuleNameCollector<'tcx> {
        ModuleNameCollector { tcx, names: Default::default() }
    }

    pub fn module_name(&self, def_id: DefId) -> Symbol {
        self.names.get_or_insert_with(def_id, || {
            let def_kind = self.tcx.def_kind(def_id);
            let path = self.tcx.def_path(def_id);

            let mut str_segments = Vec::new();

            if def_kind != DefKind::ExternMod {
                str_segments.push("Elixir.Patina".to_owned());
                str_segments.push(self.tcx.crate_name(path.krate).as_str().to_owned());
            };

            for data in &path.data {
                if data.data != DefPathData::Ctor {
                    str_segments.push(data.as_sym(true).as_str().to_owned());
                }
            }

            Symbol::intern(&str_segments.join("."))
        })
    }
}
