use rustc_data_structures::sharded::ShardedHashMap;
use rustc_data_structures::smallvec::SmallVec;
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
        assert!(self.tcx.def_kind(def_id) != DefKind::ExternMod);

        self.names.get_or_insert_with(def_id, || {
            let path = self.tcx.def_path(def_id);

            let mut str_segments = SmallVec::<[String; 6]>::from_iter([
                "Elixir.Patina".to_owned(),
                self.tcx.crate_name(path.krate).as_str().to_owned(),
            ]);

            for data in &path.data {
                if data.data != DefPathData::Ctor {
                    str_segments.push(data.as_sym(true).as_str().to_owned());
                }
            }

            Symbol::intern(&str_segments.join("."))
        })
    }

    pub fn home_module_name(&self, def_id: DefId) -> Symbol {
        self.module_name(self.tcx.erl_home_module(def_id))
    }

    pub fn extern_module_name(&self, mut def_id: DefId) -> Symbol {
        assert!(self.tcx.def_kind(def_id) == DefKind::ExternMod);

        self.names.get_or_insert_with(def_id, || {
            let mut str_segments = SmallVec::<[String; 6]>::new();

            while self.tcx.def_kind(def_id) == DefKind::ExternMod {
                str_segments.push(self.tcx.item_name(def_id).as_str().to_owned());
                def_id = self.tcx.parent(def_id);
            }

            str_segments.reverse();
            Symbol::intern(&str_segments.join("."))
        })
    }
}
