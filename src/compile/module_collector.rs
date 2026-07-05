use rustc_data_structures::sharded::ShardedHashMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::PrimTy;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::definitions::DefPathData;
use rustc_middle::erl::ErlModuleId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;

pub(super) struct ModuleNameCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    names: ShardedHashMap<ErlModuleId, Symbol>,
}

impl<'tcx> ModuleNameCollector<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> ModuleNameCollector<'tcx> {
        ModuleNameCollector { tcx, names: Default::default() }
    }

    pub fn module_name(&self, module: ErlModuleId) -> Symbol {
        self.names.get_or_insert_with(module, || match module {
            ErlModuleId::Mod(def_id) | ErlModuleId::Adt(def_id) => self.def_module_name(def_id),
            ErlModuleId::Extern(def_id) => self.extern_module_name(def_id),
            ErlModuleId::Prim(prim_ty) => self.prim_module_name(prim_ty),
        })
    }

    pub fn constructor_name(&self, def_id: DefId) -> Symbol {
        assert!(matches!(
            self.tcx.def_kind(def_id),
            DefKind::Struct | DefKind::Variant | DefKind::Ctor(..)
        ));
        // This is more of a temporary workaround; a variant/ctor doesn't fit `ErlModuleId`,
        // but at this second it's translated the same way. It likely won't stay that way
        // as we start to properly translate Enum variants, which is why I'm not building
        // a more general abstraction for this.
        self.names.get_or_insert_with(ErlModuleId::Adt(def_id), || self.def_module_name(def_id))
    }

    fn def_module_name(&self, def_id: DefId) -> Symbol {
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
    }

    fn extern_module_name(&self, mut def_id: DefId) -> Symbol {
        let mut str_segments = SmallVec::<[String; 6]>::new();

        while self.tcx.def_kind(def_id) == DefKind::ExternMod {
            str_segments.push(self.tcx.item_name(def_id).as_str().to_owned());
            def_id = self.tcx.parent(def_id);
        }

        str_segments.reverse();
        Symbol::intern(&str_segments.join("."))
    }

    fn prim_module_name(&self, prim_ty: PrimTy) -> Symbol {
        Symbol::intern(&format!("Elixir.Patina.Core.{}", prim_ty.name()))
    }
}
