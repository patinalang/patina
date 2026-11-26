use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_macros::StableHash;
use rustc_span::{Symbol, kw};

use crate::erl::{ErlModuleId, ErlReprKind, ErlVariantRepr};
use crate::ty::{self, AdtDef, GenericArgsRef, Ty, TyCtxt, VariantDef};

#[derive(Clone, Debug, PartialEq, Eq, StableHash)]
pub enum ErlShape {
    Atom(Symbol),
    AnyAtom,
    Int,
    Float,
    Binary,
    Bitstring,
    List,
    Fun,
    Pid,
    Port,
    Ref,
    Tuple { arity: usize, first: Box<ErlShape> },
    AnyTuple,
    Map,
    ElixirStruct(Symbol),
    NativeRecord { module: Symbol, name: Symbol },
    Union(Box<[ErlShape]>),
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
            (Tuple { arity: a, first: x }, Tuple { arity: b, first: y }) => a == b && x.overlaps(y),
            (AnyTuple, Tuple { .. } | AnyTuple) | (Tuple { .. }, AnyTuple) => true,
            (ElixirStruct(a), ElixirStruct(b)) => a == b,
            (Map, Map | ElixirStruct(_)) | (ElixirStruct(_), Map) => true,
            (NativeRecord { module: m, name: n }, NativeRecord { module: p, name: q }) => {
                (m, n) == (p, q)
            }
            (Binary | Bitstring, Binary | Bitstring) => true,
            (Int, Int)
            | (Float, Float)
            | (List, List)
            | (Fun, Fun)
            | (Pid, Pid)
            | (Port, Port)
            | (Ref, Ref) => true,
            _ => false,
        }
    }

    /// Simplifies redundant structure in a shape union.
    fn simplify(self) -> ErlShape {
        let ErlShape::Union(shapes) = self else { return self };

        if let [shape] = shapes.as_ref() {
            return shape.clone().simplify();
        }

        let mut alternatives: Vec<ErlShape> = Vec::with_capacity(shapes.len());
        if shapes
            .into_iter()
            .any(|shape| Self::collect_union_alternatives(&mut alternatives, shape))
        {
            return ErlShape::Any;
        }

        if alternatives.len() == 1 {
            alternatives.remove(0)
        } else {
            ErlShape::Union(alternatives.into())
        }
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
            (Bitstring, Binary) => true,
            (Map, ElixirStruct(_)) => true,
            (Tuple { arity: a, first: x }, Tuple { arity: b, first: y }) => {
                a == b && x.superset_of(y)
            }
            _ => self == other,
        }
    }

    fn tagged_tuple(arity: usize, tag: Symbol) -> ErlShape {
        ErlShape::Tuple { arity, first: Box::new(ErlShape::Atom(tag)) }
    }

    /// Widen any non-atom first element of a tuple to `Any`.
    /// This helps build smaller guard sets.
    fn shallow(&self) -> ErlShape {
        match self {
            ErlShape::Tuple { arity: _, first: box ErlShape::Atom(_) } => self.clone(),
            ErlShape::Tuple { arity, first: _ } => {
                ErlShape::Tuple { arity: *arity, first: Box::new(ErlShape::Any) }
            }
            ErlShape::Union(shapes) => {
                ErlShape::Union(shapes.iter().map(ErlShape::shallow).collect())
            }
            shape => shape.clone(),
        }
    }
}

impl<'tcx> TyCtxt<'tcx> {
    fn erl_shape_of_ty_rec(self, ty: Ty<'tcx>, visiting: &mut FxHashSet<DefId>) -> ErlShape {
        match ty.kind() {
            ty::Bool => {
                ErlShape::Union(Box::new([ErlShape::Atom(kw::True), ErlShape::Atom(kw::False)]))
            }
            ty::Int | ty::Char => ErlShape::Int,
            ty::Float => ErlShape::Float,
            ty::Str | ty::Bin => ErlShape::Binary,
            ty::Bits => ErlShape::Bitstring,
            ty::Atom => ErlShape::AnyAtom,
            ty::Slice(_) => ErlShape::List,
            ty::Map(..) => ErlShape::Map,
            ty::Tuple(tys) => {
                let first = match tys.first() {
                    Some(&first) => self.erl_shape_of_ty_rec(first, visiting).simplify(),
                    None => ErlShape::Any,
                };
                ErlShape::Tuple { arity: tys.len(), first: Box::new(first) }
            }
            ty::FnDef(..) | ty::FnPtr(..) => ErlShape::Fun,
            ty::Pid => ErlShape::Pid,
            ty::Port => ErlShape::Port,
            ty::ErlRef => ErlShape::Ref,
            ty::Adt(def, args) => self.erl_shape_of_adt(*def, args, visiting),
            ty::Term | ty::Param(_) | ty::Alias(..) | ty::Error(_) => ErlShape::Any,
            ty::Never => ErlShape::Union(Box::new([])),
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
            .map(|variant| self.erl_variant_term_shape(def, variant, args, visiting))
            .collect::<Box<[_]>>();

        visiting.remove(&def.did());

        ErlShape::Union(shapes)
    }

    fn erl_variant_term_shape(
        self,
        def: AdtDef<'tcx>,
        variant: &VariantDef,
        args: GenericArgsRef<'tcx>,
        visiting: &mut FxHashSet<DefId>,
    ) -> ErlShape {
        let repr = self.erl_variant_repr(variant.def_id);
        match repr.kind {
            ErlReprKind::Transparent { open_payload } => {
                let Some((_, field)) = self.erl_single_runtime_field(variant) else {
                    // Any other variant is invalid, and will be reported by the type checker.
                    // Returning an empty union prevents error cascades from treating this variant
                    // as open-shape.
                    return ErlShape::Union(Box::new([]));
                };

                let payload = self.erl_shape_of_ty_rec(field.ty(self, args), visiting);

                if !open_payload || self.erl_variant_wrap_guards((variant.def_id, args)).is_empty()
                {
                    // This variant's shape doesn't overlap with any other variant.
                    payload
                } else {
                    // This variant's shape can overlap with other variants, so we also need
                    // to account for its wrapped shape.
                    ErlShape::Union(Box::new([payload, ErlShape::tagged_tuple(2, repr.tag)]))
                }
            }

            _ => self.erl_variant_simple_shape(def, variant, repr),
        }
    }

    /// A shape that tells the values of the closed transparent variant `variant_def_id` apart
    /// from its siblings' values.
    pub fn erl_variant_discriminating_shape(
        self,
        variant_def_id: DefId,
        args: GenericArgsRef<'tcx>,
        payload_ty: Ty<'tcx>,
    ) -> ErlShape {
        let shape = self.erl_shape_of_ty(payload_ty);
        let shallow = shape.shallow();

        let def = self.adt_def(self.parent(variant_def_id));
        let mut siblings =
            def.variants().iter().filter(|sibling| sibling.def_id != variant_def_id).map(
                |sibling| {
                    self.erl_variant_term_shape(def, sibling, args, &mut FxHashSet::default())
                        .simplify()
                },
            );

        if siblings.all(|sibling| !shallow.overlaps(&sibling)) { shallow } else { shape.clone() }
    }

    /// Returns the runtime shape which must remain unique for this variant's representation.
    ///
    /// For an open transparent variant this is its wrapper shape; overlap of its unwrapped
    /// payload is handled dynamically.
    pub fn erl_variant_reserved_shape(
        self,
        adt: AdtDef<'tcx>,
        variant: &VariantDef,
        repr: ErlVariantRepr,
    ) -> ErlShape {
        match repr.kind {
            ErlReprKind::Transparent { open_payload: true } => ErlShape::tagged_tuple(2, repr.tag),

            ErlReprKind::Transparent { open_payload: false } => {
                let Some((_, field)) = self.erl_single_runtime_field(variant) else {
                    // Invalid variant, returned just so later diagnostics can report an error.
                    return ErlShape::Union(Box::new([]));
                };
                let args = ty::GenericArgs::identity_for_item(self, adt.did());
                self.erl_shape_of_ty(field.ty(self, args)).clone()
            }

            _ => self.erl_variant_simple_shape(adt, variant, repr),
        }
    }

    /// Common between reserved and term shapes.
    /// Do not call this for transparent variants - they have different shapes depending
    /// on the context we're checking in.
    fn erl_variant_simple_shape(
        self,
        adt: AdtDef<'tcx>,
        variant: &VariantDef,
        repr: ErlVariantRepr,
    ) -> ErlShape {
        match repr.kind {
            ErlReprKind::Elixir => {
                ErlShape::ElixirStruct(self.erl_module_name(ErlModuleId::Adt(adt.did())))
            }
            ErlReprKind::Tagged => {
                ErlShape::tagged_tuple(self.erl_runtime_field_count(variant) + 1, repr.tag)
            }
            ErlReprKind::Atom => ErlShape::Atom(repr.tag),
            ErlReprKind::Int => ErlShape::Int,
            ErlReprKind::NativeRecord => {
                ErlShape::NativeRecord { module: repr.record_module, name: repr.tag }
            }
            ErlReprKind::Transparent { open_payload: _ } => unreachable!(),
        }
    }
}

pub(super) fn erl_shape_of_ty<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> ErlShape {
    tcx.erl_shape_of_ty_rec(ty, &mut FxHashSet::default()).simplify()
}

pub(super) fn erl_variant_wrap_guards<'tcx>(
    tcx: TyCtxt<'tcx>,
    (variant_def_id, args): (DefId, GenericArgsRef<'tcx>),
) -> &'tcx [ErlShape] {
    if tcx.def_kind(variant_def_id) != DefKind::Variant
        || !matches!(
            tcx.erl_variant_repr(variant_def_id).kind,
            ErlReprKind::Transparent { open_payload: true }
        )
    {
        return &[];
    }

    let def = tcx.adt_def(tcx.parent(variant_def_id));
    if def.variants().len() == 1 {
        // Nothing to guard against.
        return &[];
    }

    let variant = def.variant_with_id(variant_def_id);
    let Some((_, payload_field)) = tcx.erl_single_runtime_field(variant) else { return &[] };
    let payload = tcx.erl_shape_of_ty(payload_field.ty(tcx, args));

    tcx.arena.alloc_from_iter(def.variants().iter().filter_map(|sibling| {
        let shape = if sibling.def_id == variant_def_id {
            ErlShape::tagged_tuple(2, tcx.erl_variant_repr(variant_def_id).tag)
        } else {
            tcx.erl_variant_term_shape(def, sibling, args, &mut FxHashSet::default()).simplify()
        };

        payload.overlaps(&shape).then_some(shape)
    }))
}

#[cfg(test)]
mod tests {
    use rustc_span::{Symbol, create_default_session_globals_then};

    use super::ErlShape::*;

    fn union<const N: usize>(shapes: [super::ErlShape; N]) -> super::ErlShape {
        Union(Box::new(shapes))
    }

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

    fn tuple(arity: usize, first: super::ErlShape) -> super::ErlShape {
        Tuple { arity, first: Box::new(first) }
    }

    #[test]
    fn tuple_overlap_depends_on_arity_and_tag() {
        create_default_session_globals_then(|| {
            let some = Symbol::intern("some");
            let tagged = super::ErlShape::tagged_tuple(2, some);
            let untagged = tuple(2, Any);

            assert!(tagged.overlaps(&tagged));
            assert!(tagged.overlaps(&untagged));
            assert!(untagged.overlaps(&tagged));
            assert!(tagged.overlaps(&AnyTuple));
            assert!(AnyTuple.overlaps(&tagged));
            assert!(!tagged.overlaps(&super::ErlShape::tagged_tuple(2, Symbol::intern("none"))));
            assert!(!tagged.overlaps(&super::ErlShape::tagged_tuple(3, some)));
        });
    }

    #[test]
    fn tuple_overlap_depends_on_first_element() {
        create_default_session_globals_then(|| {
            let tagged = super::ErlShape::tagged_tuple(2, Symbol::intern("some"));

            assert!(!tuple(2, Int).overlaps(&tagged));
            assert!(!tagged.overlaps(&tuple(2, Binary)));
            assert!(tuple(2, AnyAtom).overlaps(&tagged));
            assert!(!tuple(2, Atom(Symbol::intern("ok"))).overlaps(&tagged));
            assert!(tuple(2, union([Int, AnyAtom])).overlaps(&tagged));
            assert!(tuple(0, Any).overlaps(&tuple(0, Any)));
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
        let scalars = union([Int, Binary]);
        let empty = union([]);

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
            let tagged = super::ErlShape::tagged_tuple(2, Symbol::intern("some"));
            let untagged = tuple(2, Any);
            let numbers = union([Int, Float]);

            assert!(Any.superset_of(&tagged));
            assert!(AnyAtom.superset_of(&some));
            assert!(!some.superset_of(&AnyAtom));
            assert!(untagged.superset_of(&tagged));
            assert!(!tagged.superset_of(&untagged));
            assert!(tuple(2, AnyAtom).superset_of(&tagged));
            assert!(!tuple(2, Int).superset_of(&tagged));
            assert!(AnyTuple.superset_of(&tagged));
            assert!(numbers.superset_of(&Int));
            assert!(numbers.superset_of(&union([Float, Int])));
            assert!(Int.superset_of(&union([Int])));
            assert!(!Int.superset_of(&numbers));
        });
    }

    #[test]
    fn simplify_flattens_and_removes_covered_shapes() {
        create_default_session_globals_then(|| {
            let yes = Atom(Symbol::intern("yes"));
            let no = Atom(Symbol::intern("no"));
            let some = Symbol::intern("some");
            let shape = union([
                yes.clone(),
                union([no, yes]),
                AnyAtom,
                super::ErlShape::tagged_tuple(2, some),
                tuple(2, Any),
                Int,
            ]);

            assert_eq!(shape.simplify(), union([AnyAtom, tuple(2, Any), Int]));
        });
    }

    #[test]
    fn simplify_is_independent_of_nested_union_order() {
        create_default_session_globals_then(|| {
            let atoms = union([Atom(Symbol::intern("yes")), Atom(Symbol::intern("no"))]);

            assert_eq!(union([AnyAtom, atoms.clone()]).simplify(), AnyAtom);
            assert_eq!(union([atoms, AnyAtom]).simplify(), AnyAtom);
        });
    }

    #[test]
    fn simplify_boundary_cases() {
        assert_eq!(union([Int, Binary]).simplify(), union([Int, Binary]));
        assert_eq!(union([Int, union([Any])]).simplify(), Any);
        assert_eq!(union([union([Int, Int])]).simplify(), Int);
        assert_eq!(union([]).simplify(), union([]));
    }
}
