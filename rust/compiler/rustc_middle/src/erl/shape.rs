use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def_id::DefId;
use rustc_span::{Symbol, kw};

use crate::erl::{ErlModuleId, ErlRepr};
use crate::ty::{self, AdtDef, GenericArgsRef, Ty, TyCtxt, VariantDef};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErlShape {
    Atom(Symbol),
    AnyAtom,
    Int,
    Float,
    Binary,
    List,
    Fun,
    Tuple { arity: usize, tag: Option<Symbol> },
    AnyTuple,
    Map,
    ElixirStruct(Symbol),
    NativeRecord { module: Symbol, name: Symbol },
    Union(Vec<ErlShape>),
    Any,
}

impl ErlShape {
    /// Can there exist a term that fits both shapes?
    pub fn overlaps(&self, other: &ErlShape) -> bool {
        use ErlShape::*;
        match (self, other) {
            (Union(shapes), other) | (other, Union(shapes)) => {
                shapes.iter().any(|shape| shape.overlaps(other))
            }
            (Any, _) | (_, Any) => true,
            (Atom(a), Atom(b)) => a == b,
            (AnyAtom, Atom(_) | AnyAtom) | (Atom(_), AnyAtom) => true,
            (Tuple { arity: a, tag: x }, Tuple { arity: b, tag: y }) => {
                a == b && (x.is_none() || y.is_none() || x == y)
            }
            (AnyTuple, Tuple { .. } | AnyTuple) | (Tuple { .. }, AnyTuple) => true,
            (ElixirStruct(a), ElixirStruct(b)) => a == b,
            (Map, Map | ElixirStruct(_)) | (ElixirStruct(_), Map) => true,
            (NativeRecord { module: m, name: n }, NativeRecord { module: p, name: q }) => {
                (m, n) == (p, q)
            }
            (Int, Int) | (Float, Float) | (Binary, Binary) | (List, List) | (Fun, Fun) => true,
            _ => false,
        }
    }

    /// Simplifies redundant structure in a shape union.
    /// Call this before emitting guards.
    pub fn simplify(self) -> ErlShape {
        let ErlShape::Union(mut shapes) = self else { return self };

        if shapes.len() == 1 {
            return shapes.remove(0).simplify();
        }

        let mut alternatives: Vec<ErlShape> = Vec::with_capacity(shapes.len());
        if shapes
            .into_iter()
            .any(|shape| Self::collect_union_alternatives(&mut alternatives, shape))
        {
            return ErlShape::Any;
        }

        if alternatives.len() == 1 { alternatives.remove(0) } else { ErlShape::Union(alternatives) }
    }

    /// Adds `shape` to a simplified union, returning whether it contains `Any`.
    fn collect_union_alternatives(alternatives: &mut Vec<ErlShape>, shape: ErlShape) -> bool {
        match shape {
            ErlShape::Any => true,

            ErlShape::Union(shapes) => shapes
                .into_iter()
                .any(|shape| Self::collect_union_alternatives(alternatives, shape)),

            shape => {
                if !alternatives.iter().any(|alternative| alternative.superset_of(&shape)) {
                    alternatives.retain(|alternative| !shape.superset_of(alternative));
                    alternatives.push(shape);
                }
                false
            }
        }
    }

    /// Is `self` a superset of `other`?
    fn superset_of(&self, other: &ErlShape) -> bool {
        use ErlShape::*;
        match (self, other) {
            (Any, _) => true,
            (Union(a), Union(b)) => b.iter().all(|b| a.iter().any(|a| a.superset_of(b))),
            (Union(shapes), other) => shapes.iter().any(|shape| shape.superset_of(other)),
            (other, Union(shapes)) => shapes.iter().all(|shape| other.superset_of(shape)),
            (AnyAtom, Atom(_)) => true,
            (AnyTuple, Tuple { .. }) => true,
            (Map, ElixirStruct(_)) => true,
            (Tuple { arity: a, tag: None }, Tuple { arity: b, .. }) => a == b,
            _ => self == other,
        }
    }
}

impl<'tcx> TyCtxt<'tcx> {
    /// The runtime shape of `ty`.
    pub fn erl_shape_of_ty(self, ty: Ty<'tcx>) -> ErlShape {
        self.erl_shape_of_ty_rec(ty, &mut FxHashSet::default())
    }

    fn erl_shape_of_ty_rec(self, ty: Ty<'tcx>, visiting: &mut FxHashSet<DefId>) -> ErlShape {
        match ty.kind() {
            ty::Bool => ErlShape::Union(vec![ErlShape::Atom(kw::True), ErlShape::Atom(kw::False)]),
            ty::Int | ty::Char => ErlShape::Int,
            ty::Float => ErlShape::Float,
            ty::Str => ErlShape::Binary,
            ty::Atom => ErlShape::AnyAtom,
            ty::Slice(_) => ErlShape::List,
            ty::Map(..) => ErlShape::Map,
            ty::Tuple(tys) => ErlShape::Tuple { arity: tys.len(), tag: None },
            ty::FnDef(..) | ty::FnPtr(..) => ErlShape::Fun,
            ty::Adt(def, args) => self.erl_shape_of_adt(*def, args, visiting),
            ty::Term => ErlShape::Any,
            _ => unreachable!("not a thing in Patina"),
        }
    }

    fn erl_shape_of_adt(
        self,
        def: AdtDef<'tcx>,
        args: GenericArgsRef<'tcx>,
        visiting: &mut FxHashSet<DefId>,
    ) -> ErlShape {
        // Cyclic ADT
        if !visiting.insert(def.did()) {
            return ErlShape::Any;
        }

        let shapes = def
            .variants()
            .iter()
            .map(|variant| self.erl_shape_of_variant(def, variant, args, visiting))
            .collect();

        visiting.remove(&def.did());

        ErlShape::Union(shapes)
    }

    fn erl_shape_of_variant(
        self,
        def: AdtDef<'tcx>,
        variant: &VariantDef,
        args: GenericArgsRef<'tcx>,
        visiting: &mut FxHashSet<DefId>,
    ) -> ErlShape {
        let repr = self.erl_variant_repr(variant.def_id);
        match repr.kind {
            ErlRepr::Elixir => {
                ErlShape::ElixirStruct(self.erl_module_name(ErlModuleId::Adt(def.did())))
            }
            ErlRepr::Tagged => {
                ErlShape::Tuple { arity: variant.fields.len() + 1, tag: Some(repr.tag) }
            }
            ErlRepr::Atom => ErlShape::Atom(repr.tag),
            ErlRepr::Transparent => variant.fields.iter().next().map_or(ErlShape::Any, |field| {
                self.erl_shape_of_ty_rec(field.ty(self, args), visiting)
            }),
            ErlRepr::NativeRecord => {
                ErlShape::NativeRecord { module: repr.record_module, name: repr.tag }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rustc_span::{Symbol, create_default_session_globals_then};

    use super::ErlShape::*;

    #[test]
    fn primitive_overlap() {
        assert!(Int.overlaps(&Int));
        assert!(Any.overlaps(&Int));
        assert!(Int.overlaps(&Any));
        assert!(!Int.overlaps(&Float));
        assert!(!Binary.overlaps(&List));
        assert!(!Fun.overlaps(&Map));
    }

    #[test]
    fn atom_overlap() {
        create_default_session_globals_then(|| {
            let yes = Atom(Symbol::intern("yes"));
            let no = Atom(Symbol::intern("no"));

            assert!(yes.overlaps(&yes));
            assert!(AnyAtom.overlaps(&yes));
            assert!(yes.overlaps(&AnyAtom));
            assert!(!yes.overlaps(&no));
        });
    }

    #[test]
    fn tuple_overlap_depends_on_arity_and_tag() {
        create_default_session_globals_then(|| {
            let some = Symbol::intern("some");
            let tagged = Tuple { arity: 2, tag: Some(some) };
            let untagged = Tuple { arity: 2, tag: None };

            assert!(tagged.overlaps(&tagged));
            assert!(tagged.overlaps(&untagged));
            assert!(untagged.overlaps(&tagged));
            assert!(tagged.overlaps(&AnyTuple));
            assert!(AnyTuple.overlaps(&tagged));
            assert!(!tagged.overlaps(&Tuple { arity: 2, tag: Some(Symbol::intern("none")) }));
            assert!(!tagged.overlaps(&Tuple { arity: 3, tag: Some(some) }));
        });
    }

    #[test]
    fn map_overlap() {
        create_default_session_globals_then(|| {
            let point = ElixirStruct(Symbol::intern("Elixir.Point"));
            let line = ElixirStruct(Symbol::intern("Elixir.Line"));

            assert!(point.overlaps(&point));
            assert!(Map.overlaps(&point));
            assert!(point.overlaps(&Map));
            assert!(!point.overlaps(&line));
        });
    }

    #[test]
    fn native_record_overlap() {
        create_default_session_globals_then(|| {
            let point =
                NativeRecord { module: Symbol::intern("geometry"), name: Symbol::intern("point") };
            let line =
                NativeRecord { module: Symbol::intern("geometry"), name: Symbol::intern("line") };
            let foreign_point =
                NativeRecord { module: Symbol::intern("other"), name: Symbol::intern("point") };

            assert!(point.overlaps(&point));
            assert!(!point.overlaps(&line));
            assert!(!point.overlaps(&foreign_point));
            assert!(!point.overlaps(&AnyTuple));
        });
    }

    #[test]
    fn union_overlap() {
        let scalars = Union(vec![Int, Binary]);
        let empty = Union(Vec::new());

        assert!(scalars.overlaps(&Int));
        assert!(Binary.overlaps(&scalars));
        assert!(!scalars.overlaps(&Float));
        assert!(!empty.overlaps(&empty));
        assert!(!empty.overlaps(&Any));
        assert!(!Any.overlaps(&empty));
    }

    #[test]
    fn superset() {
        create_default_session_globals_then(|| {
            let some = Atom(Symbol::intern("some"));
            let tagged = Tuple { arity: 2, tag: Some(Symbol::intern("some")) };
            let untagged = Tuple { arity: 2, tag: None };
            let numbers = Union(vec![Int, Float]);

            assert!(Any.superset_of(&tagged));
            assert!(AnyAtom.superset_of(&some));
            assert!(!some.superset_of(&AnyAtom));
            assert!(untagged.superset_of(&tagged));
            assert!(!tagged.superset_of(&untagged));
            assert!(AnyTuple.superset_of(&tagged));
            assert!(numbers.superset_of(&Int));
            assert!(numbers.superset_of(&Union(vec![Float, Int])));
            assert!(Int.superset_of(&Union(vec![Int])));
            assert!(!Int.superset_of(&numbers));
        });
    }

    #[test]
    fn simplify_flattens_and_removes_covered_shapes() {
        create_default_session_globals_then(|| {
            let yes = Atom(Symbol::intern("yes"));
            let no = Atom(Symbol::intern("no"));
            let some = Symbol::intern("some");
            let shape = Union(vec![
                yes.clone(),
                Union(vec![no, yes]),
                AnyAtom,
                Tuple { arity: 2, tag: Some(some) },
                Tuple { arity: 2, tag: None },
                Int,
            ]);

            assert_eq!(shape.simplify(), Union(vec![AnyAtom, Tuple { arity: 2, tag: None }, Int]));
        });
    }

    #[test]
    fn simplify_is_independent_of_nested_union_order() {
        create_default_session_globals_then(|| {
            let atoms = Union(vec![Atom(Symbol::intern("yes")), Atom(Symbol::intern("no"))]);

            assert_eq!(Union(vec![AnyAtom, atoms.clone()]).simplify(), AnyAtom);
            assert_eq!(Union(vec![atoms, AnyAtom]).simplify(), AnyAtom);
        });
    }

    #[test]
    fn simplify_boundary_cases() {
        assert_eq!(Union(vec![Int, Binary]).simplify(), Union(vec![Int, Binary]));
        assert_eq!(Union(vec![Int, Union(vec![Any])]).simplify(), Any);
        assert_eq!(Union(vec![Union(vec![Int, Int])]).simplify(), Int);
        assert_eq!(Union(Vec::new()).simplify(), Union(Vec::new()));
    }
}
