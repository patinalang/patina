use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

use derive_where::derive_where;
use rustc_abi::ExternAbi;
use rustc_ast_ir::Mutability;
#[cfg(feature = "nightly")]
use rustc_data_structures::stable_hasher::{StableHash, StableHashCtxt, StableHasher};
#[cfg(feature = "nightly")]
use rustc_macros::{Decodable_NoContext, Encodable_NoContext, StableHash_NoContext};
use rustc_type_ir_macros::{
    GenericTypeVisitable, Lift_Generic, TypeFoldable_Generic, TypeVisitable_Generic,
};

use self::TyKind::*;
pub use self::closure::*;
use crate::inherent::*;
#[cfg(feature = "nightly")]
use crate::visit::TypeVisitable;
use crate::{self as ty, BoundVarIndexKind, Interner};

mod closure;

#[derive_where(Clone, Copy, Hash, PartialEq, Debug; I: Interner)]
#[derive(GenericTypeVisitable, Lift_Generic)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, StableHash_NoContext)
)]
pub enum AliasTyKind<I: Interner> {
    /// A projection `<Type as Trait>::AssocType`.
    ///
    /// Can get normalized away if monomorphic enough.
    ///
    /// The `def_id` is the `DefId` of the `TraitItem` for the associated type.
    ///
    /// Note that the `def_id` is not the `DefId` of the `TraitRef` containing this
    /// associated type, which is in `interner.associated_item(def_id).container`,
    /// aka. `interner.parent(def_id)`.
    Projection { def_id: I::DefId },

    /// An associated type in an inherent `impl`
    ///
    /// The `def_id` is the `DefId` of the `ImplItem` for the associated type.
    Inherent { def_id: I::DefId },

    /// An opaque type (usually from `impl Trait` in type aliases or function return types)
    ///
    /// `def_id` is the `DefId` of the `OpaqueType` item.
    ///
    ///
    /// Can only be normalized away in `PostAnalysis` mode or its defining scope.
    ///
    /// During codegen, `interner.type_of(def_id)` can be used to get the type of the
    /// underlying type if the type is an opaque.
    Opaque { def_id: I::DefId },

    /// A type alias that actually checks its trait bounds.
    ///
    /// Currently only used if the type alias references opaque types.
    /// Can always be normalized away.
    Free { def_id: I::DefId },
}

impl<I: Interner> AliasTyKind<I> {
    pub fn new_from_def_id(interner: I, def_id: I::DefId) -> Self {
        interner.alias_ty_kind_from_def_id(def_id)
    }

    pub fn descr(self) -> &'static str {
        match self {
            AliasTyKind::Projection { .. } => "associated type",
            AliasTyKind::Inherent { .. } => "inherent associated type",
            AliasTyKind::Opaque { .. } => "opaque type",
            AliasTyKind::Free { .. } => "type alias",
        }
    }

    pub fn def_id(self) -> I::DefId {
        let (AliasTyKind::Projection { def_id }
        | AliasTyKind::Inherent { def_id }
        | AliasTyKind::Opaque { def_id }
        | AliasTyKind::Free { def_id }) = self;

        def_id
    }
}

/// Defines the kinds of types used by the type system.
///
/// Types written by the user start out as `hir::TyKind` and get
/// converted to this representation using `<dyn HirTyLowerer>::lower_ty`.
#[cfg_attr(feature = "nightly", rustc_diagnostic_item = "IrTyKind")]
#[derive_where(Clone, Copy, Hash, PartialEq; I: Interner)]
#[derive(GenericTypeVisitable)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, StableHash_NoContext)
)]
pub enum TyKind<I: Interner> {
    /// The primitive boolean type. Written as `bool`.
    Bool,

    /// The primitive character type; holds a Unicode scalar value
    /// (a non-surrogate code point). Written as `char`.
    Char,

    /// A primitive integer type. Written as `int`.
    Int,

    /// A primitive floating-point type. Written as `float`.
    Float,

    /// Algebraic data types (ADT). For example: structures, enumerations and unions.
    ///
    /// For example, the type `List<i32>` would be represented using the `AdtDef`
    /// for `struct List<T>` and the args `[i32]`.
    ///
    /// Note that generic parameters in fields only get lazily instantiated
    /// by using something like `adt_def.all_fields().map(|field| field.ty(interner, args))`.
    Adt(I::AdtDef, I::GenericArgs),

    /// An unsized FFI type that is opaque to Rust. Written as `extern type T`.
    Foreign(I::ForeignId),

    /// The pointee of a string slice. Written as `str`.
    Str,

    /// An opaque Erlang term. Written as `term`. Has no literal form.
    Term,

    /// An Erlang atom. Written as `atom`, with literals `a"…"`.
    Atom,

    /// An Erlang binary. Written as `bin`, with literals `b"…"`. A supertype of `str`.
    Bin,

    /// An Erlang bitstring. Written as `bits`. A supertype of `bin`.
    Bits,

    /// A homogeneous map. Written as `#{K => V}`.
    Map(I::Ty, I::Ty),

    /// An array with the given length. Written as `[T; N]`.
    Array(I::Ty, I::Const),

    /// A pattern newtype.
    ///
    /// Takes any type and restricts its valid values to its pattern.
    /// This will also change the layout to take advantage of this restriction.
    /// Only `Copy` and `Clone` will automatically get implemented for pattern types.
    /// Auto-traits treat this as if it were an aggregate with a single nested type.
    /// Only supports integer range patterns for now.
    Pat(I::Ty, I::Pat),

    /// The pointee of an array slice. Written as `[T]`.
    Slice(I::Ty),

    /// A raw pointer. Written as `*mut T` or `*const T`
    RawPtr(I::Ty, Mutability),

    /// A reference; a pointer with an associated lifetime. Written as
    /// `&'a mut T` or `&'a T`.
    Ref(I::Region, I::Ty, Mutability),

    /// The anonymous type of a function declaration/definition.
    ///
    /// Each function has a unique type.
    ///
    /// For the function `fn foo() -> i32 { 3 }` this type would be
    /// shown to the user as `fn() -> i32 {foo}`.
    ///
    /// For example the type of `bar` here:
    /// ```rust
    /// fn foo() -> i32 { 1 }
    /// let bar = foo; // bar: fn() -> i32 {foo}
    /// ```
    FnDef(I::FunctionId, I::GenericArgs),

    /// A pointer to a function.
    ///
    /// Written as `fn() -> i32`.
    ///
    /// Note that both functions and closures start out as either
    /// [FnDef] or [Closure] which can be then be coerced to this variant.
    ///
    /// For example the type of `bar` here:
    ///
    /// ```rust
    /// fn foo() -> i32 { 1 }
    /// let bar: fn() -> i32 = foo;
    /// ```
    ///
    /// These two fields are equivalent to a `ty::Binder<I, FnSig<I>>`. But by
    /// splitting that into two pieces, we get a more compact data layout that
    /// reduces the size of `TyKind` by 8 bytes. It is a very hot type, so it's
    /// worth the mild inconvenience.
    FnPtr(ty::Binder<I, FnSigTys<I>>, FnHeader<I>),

    /// An unsafe binder type.
    ///
    /// A higher-ranked type used to represent a type which has had some of its
    /// lifetimes erased. This can be used to represent types in positions where
    /// a lifetime is literally inexpressible, such as self-referential types.
    UnsafeBinder(UnsafeBinderInner<I>),

    /// A trait object. Written as `dyn for<'b> Trait<'b, Assoc = u32> + Send + 'a`.
    Dynamic(I::BoundExistentialPredicates, I::Region),

    /// The anonymous type of a closure. Used to represent the type of `|a| a`.
    ///
    /// Closure args contain both the - potentially instantiated - generic parameters
    /// of its parent and some synthetic parameters. See the documentation for
    /// `ClosureArgs` for more details.
    Closure(I::ClosureId, I::GenericArgs),

    /// The anonymous type of a closure. Used to represent the type of `async |a| a`.
    ///
    /// Coroutine-closure args contain both the - potentially instantiated - generic
    /// parameters of its parent and some synthetic parameters. See the documentation
    /// for `CoroutineClosureArgs` for more details.
    CoroutineClosure(I::CoroutineClosureId, I::GenericArgs),

    /// The anonymous type of a coroutine. Used to represent the type of
    /// `|a| yield a`.
    ///
    /// For more info about coroutine args, visit the documentation for
    /// `CoroutineArgs`.
    Coroutine(I::CoroutineId, I::GenericArgs),

    /// A type representing the types stored inside a coroutine.
    ///
    /// This should only appear as part of the `CoroutineArgs`.
    ///
    /// Unlike upvars, the witness can reference lifetimes from
    /// inside of the coroutine itself. To deal with them in
    /// the type of the coroutine, we convert them to higher ranked
    /// lifetimes bound by the witness itself.
    ///
    /// This contains the `DefId` and the `GenericArgsRef` of the coroutine.
    /// The actual witness types are computed on MIR by the `mir_coroutine_witnesses` query.
    ///
    /// Looking at the following example, the witness for this coroutine
    /// may end up as something like `for<'a> [Vec<i32>, &'a Vec<i32>]`:
    ///
    /// ```
    /// #![feature(coroutines)]
    /// #[coroutine] static |a| {
    ///     let x = &vec![3];
    ///     yield a;
    ///     yield x[0];
    /// }
    /// # ;
    /// ```
    CoroutineWitness(I::CoroutineId, I::GenericArgs),

    /// The never type `!`.
    Never,

    /// A tuple type. For example, `(i32, bool)`.
    Tuple(I::Tys),

    /// A projection, opaque type, free type alias, or inherent associated type.
    ///
    /// All of these types are represented as pairs of def-id and args, and can
    /// be normalized, so they are grouped conceptually.
    Alias(AliasTy<I>),

    /// A type parameter; for example, `T` in `fn f<T>(x: T) {}`.
    Param(I::ParamTy),

    /// Bound type variable, used to represent the `'a` in `for<'a> fn(&'a ())`.
    ///
    /// For canonical queries, we replace inference variables with bound variables,
    /// so e.g. when checking whether `&'_ (): Trait<_>` holds, we canonicalize that to
    /// `for<'a, T> &'a (): Trait<T>` and then convert the introduced bound variables
    /// back to inference variables in a new inference context when inside of the query.
    ///
    /// It is conventional to render anonymous bound types like `^N` or `^D_N`,
    /// where `N` is the bound variable's anonymous index into the binder, and
    /// `D` is the debruijn index, or totally omitted if the debruijn index is zero.
    ///
    /// See the `rustc-dev-guide` for more details about
    /// [higher-ranked trait bounds][1] and [canonical queries][2].
    ///
    /// [1]: https://rustc-dev-guide.rust-lang.org/traits/hrtb.html
    /// [2]: https://rustc-dev-guide.rust-lang.org/traits/canonical-queries.html
    Bound(BoundVarIndexKind, ty::BoundTy<I>),

    /// A placeholder type, used during higher ranked subtyping to instantiate
    /// bound variables.
    ///
    /// It is conventional to render anonymous placeholder types like `!N` or `!U_N`,
    /// where `N` is the placeholder variable's anonymous index (which corresponds
    /// to the bound variable's index from the binder from which it was instantiated),
    /// and `U` is the universe index in which it is instantiated, or totally omitted
    /// if the universe index is zero.
    Placeholder(ty::PlaceholderType<I>),

    /// A type variable used during type checking.
    ///
    /// Similar to placeholders, inference variables also live in a universe to
    /// correctly deal with higher ranked types. Though unlike placeholders,
    /// that universe is stored in the `InferCtxt` instead of directly
    /// inside of the type.
    Infer(InferTy),

    /// A placeholder for a type which could not be computed.
    ///
    /// This is propagated to avoid useless error messages.
    Error(I::ErrorGuaranteed),
}

impl<I: Interner> Eq for TyKind<I> {}

impl<I: Interner> TyKind<I> {
    pub fn fn_sig(self, interner: I) -> ty::Binder<I, ty::FnSig<I>> {
        match self {
            ty::FnPtr(sig_tys, hdr) => sig_tys.with(hdr),
            ty::FnDef(def_id, args) => {
                interner.fn_sig(def_id).instantiate(interner, args).skip_norm_wip()
            }
            ty::Error(_) => {
                // ignore errors (#54954)
                ty::Binder::dummy(ty::FnSig::dummy())
            }
            ty::Closure(..) => panic!(
                "to get the signature of a closure, use `args.as_closure().sig()` not `fn_sig()`",
            ),
            _ => panic!("Ty::fn_sig() called on non-fn type: {:?}", self),
        }
    }

    /// The position of an Erlang binary type in the subtyping chain `str <: bin <: bits`.
    pub fn bitstring_rank(self) -> Option<u8> {
        match self {
            ty::Str => Some(0),
            ty::Bin => Some(1),
            ty::Bits => Some(2),
            _ => None,
        }
    }

    /// Returns `true` when the outermost type cannot be further normalized,
    /// resolved, or instantiated.
    ///
    /// This includes all primitive types, but also
    /// things like ADTs and trait objects, since even if their arguments or
    /// nested types may be further simplified, the outermost [`ty::TyKind`] or
    /// type constructor remains the same.
    pub fn is_known_rigid(self) -> bool {
        match self {
            ty::Bool
            | ty::Char
            | ty::Int
            | ty::Float
            | ty::Adt(_, _)
            | ty::Foreign(_)
            | ty::Str
            | ty::Term
            | ty::Atom
            | ty::Bin
            | ty::Bits
            | ty::Map(_, _)
            | ty::Array(_, _)
            | ty::Pat(_, _)
            | ty::Slice(_)
            | ty::RawPtr(_, _)
            | ty::Ref(_, _, _)
            | ty::FnDef(_, _)
            | ty::FnPtr(..)
            | ty::UnsafeBinder(_)
            | ty::Dynamic(_, _)
            | ty::Closure(_, _)
            | ty::CoroutineClosure(_, _)
            | ty::Coroutine(_, _)
            | ty::CoroutineWitness(..)
            | ty::Never
            | ty::Tuple(_) => true,

            ty::Error(_)
            | ty::Infer(_)
            | ty::Alias(_)
            | ty::Param(_)
            | ty::Bound(_, _)
            | ty::Placeholder(_) => false,
        }
    }
}

// This is manually implemented because a derive would require `I: Debug`
impl<I: Interner> fmt::Debug for TyKind<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bool => write!(f, "bool"),
            Char => write!(f, "char"),
            Int => write!(f, "int"),
            Float => write!(f, "float"),
            Adt(d, s) => {
                write!(f, "{d:?}")?;
                let mut s = s.iter();
                let first = s.next();
                match first {
                    Some(first) => write!(f, "<{:?}", first)?,
                    None => return Ok(()),
                };

                for arg in s {
                    write!(f, ", {:?}", arg)?;
                }

                write!(f, ">")
            }
            Foreign(d) => f.debug_tuple("Foreign").field(d).finish(),
            Str => write!(f, "str"),
            Term => write!(f, "term"),
            Atom => write!(f, "atom"),
            Bin => write!(f, "bin"),
            Bits => write!(f, "bits"),
            Map(k, v) => write!(f, "#{{{k:?} => {v:?}}}"),
            Array(t, c) => write!(f, "[{t:?}; {c:?}]"),
            Pat(t, p) => write!(f, "pattern_type!({t:?} is {p:?})"),
            Slice(t) => write!(f, "[{:?}]", &t),
            RawPtr(ty, mutbl) => write!(f, "*{} {:?}", mutbl.ptr_str(), ty),
            Ref(r, t, m) => write!(f, "&{:?} {}{:?}", r, m.prefix_str(), t),
            FnDef(d, s) => f.debug_tuple("FnDef").field(d).field(&s).finish(),
            FnPtr(sig_tys, hdr) => write!(f, "{:?}", sig_tys.with(*hdr)),
            // FIXME(unsafe_binder): print this like `unsafe<'a> T<'a>`.
            UnsafeBinder(binder) => write!(f, "{:?}", binder),
            Dynamic(p, r) => write!(f, "dyn {p:?} + {r:?}"),
            Closure(d, s) => f.debug_tuple("Closure").field(d).field(&s).finish(),
            CoroutineClosure(d, s) => f.debug_tuple("CoroutineClosure").field(d).field(&s).finish(),
            Coroutine(d, s) => f.debug_tuple("Coroutine").field(d).field(&s).finish(),
            CoroutineWitness(d, s) => f.debug_tuple("CoroutineWitness").field(d).field(&s).finish(),
            Never => write!(f, "!"),
            Tuple(t) => {
                write!(f, "(")?;
                let mut count = 0;
                for ty in t.iter() {
                    if count > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{ty:?}")?;
                    count += 1;
                }
                // unary tuples need a trailing comma
                if count == 1 {
                    write!(f, ",")?;
                }
                write!(f, ")")
            }
            Alias(a) => f.debug_tuple("Alias").field(&a).finish(),
            Param(p) => write!(f, "{p:?}"),
            Bound(d, b) => crate::debug_bound_var(f, *d, b),
            Placeholder(p) => write!(f, "{p:?}"),
            Infer(t) => write!(f, "{:?}", t),
            TyKind::Error(_) => write!(f, "{{type error}}"),
        }
    }
}

/// Represents the projection of an associated, opaque, or lazy-type-alias type.
///
/// * For a projection, this would be `<Ty as Trait<...>>::N<...>`.
/// * For an inherent projection, this would be `Ty::N<...>`.
/// * For an opaque type, there is no explicit syntax.
#[derive_where(Clone, Copy, Hash, PartialEq, Debug; I: Interner)]
#[derive(TypeVisitable_Generic, GenericTypeVisitable, TypeFoldable_Generic, Lift_Generic)]
#[cfg_attr(
    feature = "nightly",
    derive(Decodable_NoContext, Encodable_NoContext, StableHash_NoContext)
)]
pub struct AliasTy<I: Interner> {
    /// The parameters of the associated or opaque type.
    ///
    /// For a projection, these are the generic parameters for the trait and the
    /// GAT parameters, if there are any.
    ///
    /// For an inherent projection, they consist of the self type and the GAT parameters,
    /// if there are any.
    ///
    /// For RPIT the generic parameters are for the generics of the function,
    /// while for TAIT it is used for the generic parameters of the alias.
    pub args: I::GenericArgs,

    #[type_foldable(identity)]
    #[type_visitable(ignore)]
    pub kind: AliasTyKind<I>,

    /// This field exists to prevent the creation of `AliasTy` without using [`AliasTy::new_from_args`].
    #[derive_where(skip(Debug))]
    pub(crate) _use_alias_ty_new_instead: (),
}

impl<I: Interner> Eq for AliasTy<I> {}

impl<I: Interner> AliasTy<I> {
    pub fn new_from_args(interner: I, kind: AliasTyKind<I>, args: I::GenericArgs) -> AliasTy<I> {
        interner.debug_assert_args_compatible(kind.def_id(), args);
        AliasTy { kind, args, _use_alias_ty_new_instead: () }
    }

    pub fn new(
        interner: I,
        kind: AliasTyKind<I>,
        args: impl IntoIterator<Item: Into<I::GenericArg>>,
    ) -> AliasTy<I> {
        let args = interner.mk_args_from_iter(args.into_iter().map(Into::into));
        Self::new_from_args(interner, kind, args)
    }

    /// Whether this alias type is an opaque.
    pub fn is_opaque(self) -> bool {
        matches!(self.kind, AliasTyKind::Opaque { .. })
    }

    pub fn to_ty(self, interner: I) -> I::Ty {
        Ty::new_alias(interner, self)
    }
}

/// The following methods work only with (trait) associated type projections.
impl<I: Interner> AliasTy<I> {
    #[track_caller]
    pub fn self_ty(self) -> I::Ty {
        self.args.type_at(0)
    }

    pub fn with_replaced_self_ty(self, interner: I, self_ty: I::Ty) -> Self {
        AliasTy::new(
            interner,
            self.kind,
            [self_ty.into()].into_iter().chain(self.args.iter().skip(1)),
        )
    }

    pub fn trait_def_id(self, interner: I) -> I::DefId {
        let AliasTyKind::Projection { def_id } = self.kind else { panic!("expected a projection") };

        interner.parent(def_id)
    }

    /// Extracts the underlying trait reference and own args from this projection.
    ///
    /// For example, if this is a projection of `<T as StreamingIterator>::Item<'a>`,
    /// then this function would return a `T: StreamingIterator` trait reference and
    /// `['a]` as the own args.
    pub fn trait_ref_and_own_args(self, interner: I) -> (ty::TraitRef<I>, I::GenericArgsSlice) {
        let AliasTyKind::Projection { def_id } = self.kind else { panic!("expected a projection") };

        interner.trait_ref_and_own_args_for_alias(def_id, self.args)
    }

    /// Extracts the underlying trait reference from this projection.
    ///
    /// For example, if this is a projection of `<T as Iterator>::Item`,
    /// then this function would return a `T: Iterator` trait reference.
    ///
    /// WARNING: This will drop the args for generic associated types
    /// consider calling [Self::trait_ref_and_own_args] to get those
    /// as well.
    pub fn trait_ref(self, interner: I) -> ty::TraitRef<I> {
        self.trait_ref_and_own_args(interner).0
    }
}

rustc_index::newtype_index! {
    /// A **ty**pe **v**ariable **ID**.
    #[encodable]
    #[orderable]
    #[debug_format = "?{}t"]
    #[gate_rustc_only]
    pub struct TyVid {}
}

/// A placeholder for a type that hasn't been inferred yet.
///
/// E.g., if we have an empty array (`[]`), then we create a fresh
/// type variable for the element type since we won't know until it's
/// used what the element type is supposed to be.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "nightly", derive(Encodable_NoContext, Decodable_NoContext))]
pub enum InferTy {
    /// A type variable.
    TyVar(TyVid),

    /// A [`FreshTy`][Self::FreshTy] is one that is generated as a replacement
    /// for an unbound type variable.
    ///
    /// This is convenient for caching etc. See `TypeFreshener` for more details.
    ///
    /// Compare with [`TyVar`][Self::TyVar].
    FreshTy(u32),
}

#[cfg(feature = "nightly")]
impl StableHash for InferTy {
    fn stable_hash<Hcx: StableHashCtxt>(&self, hcx: &mut Hcx, hasher: &mut StableHasher) {
        use InferTy::*;
        std::mem::discriminant(self).stable_hash(hcx, hasher);
        match self {
            TyVar(_) => {
                panic!("type variables should not be hashed: {self:?}")
            }
            FreshTy(v) => v.stable_hash(hcx, hasher),
        }
    }
}

impl fmt::Display for InferTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InferTy::*;
        match *self {
            TyVar(_) => write!(f, "_"),
            FreshTy(v) => write!(f, "FreshTy({v})"),
        }
    }
}

impl fmt::Debug for InferTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InferTy::*;
        match *self {
            TyVar(ref v) => v.fmt(f),
            FreshTy(v) => write!(f, "FreshTy({v:?})"),
        }
    }
}

#[derive_where(Clone, Copy, PartialEq, Hash, Debug; I: Interner)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, StableHash_NoContext)
)]
#[derive(TypeVisitable_Generic, GenericTypeVisitable, TypeFoldable_Generic)]
pub struct TypeAndMut<I: Interner> {
    pub ty: I::Ty,
    pub mutbl: Mutability,
}

impl<I: Interner> Eq for TypeAndMut<I> {}

/// Contains the packed non-type fields of a function signature.
// FIXME(splat): add the splatted argument index as a u16
#[derive_where(Copy, Clone, PartialEq, Eq, Hash; I: Interner)]
#[derive(TypeVisitable_Generic, TypeFoldable_Generic)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, StableHash_NoContext)
)]
pub struct FnSigKind<I: Interner> {
    /// Holds the `is_fnref` and safety bitflags, and 6 bits for the `ExternAbi` variant and unwind
    /// flag.
    #[type_visitable(ignore)]
    #[type_foldable(identity)]
    flags: u8,
    #[type_visitable(ignore)]
    #[type_foldable(identity)]
    _marker: PhantomData<fn() -> I>,
}

impl<I: Interner, J: Interner> crate::lift::Lift<J> for FnSigKind<I> {
    type Lifted = FnSigKind<J>;
    fn lift_to_interner(self, _cx: J) -> Self::Lifted {
        FnSigKind { flags: self.flags, _marker: PhantomData }
    }
}

impl<I: Interner> fmt::Debug for FnSigKind<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("FnSigKind");

        if self.is_safe() {
            f.field(&"Safe");
        } else {
            f.field(&"Unsafe");
        }

        f.field(&self.abi());

        if self.is_fnref() {
            f.field(&"FnRef");
        };

        f.finish()
    }
}

impl<I: Interner> FnSigKind<I> {
    /// Mask for the `ExternAbi` variant, including the unwind flag.
    const EXTERN_ABI_MASK: u8 = 0b111111;

    /// Bitflag for `Safety::Safe`. The default is `Unsafe`.
    const SAFE_FLAG: u8 = 1 << 6;

    /// Bitflag for `is_fnref`
    const IS_FNREF_FLAG: u8 = 1 << 7;

    /// Create a new FnSigKind with the "Rust" ABI, "Unsafe" safety, and `is_fnref = false`.
    /// To modify these flags, use the `set_*` methods, for readability.
    // FIXME: use Default instead when that trait is const stable.
    pub fn default() -> Self {
        Self { flags: 0, _marker: PhantomData }
            .set_abi(ExternAbi::Rust)
            .set_safety(I::Safety::unsafe_mode())
            .set_is_fnref(false)
    }

    /// Create a new FnSigKind with the given ABI, safety, and `is_fnref` flag.
    pub fn new(abi: ExternAbi, safety: I::Safety, is_fnref: bool) -> Self {
        Self::default().set_abi(abi).set_safety(safety).set_is_fnref(is_fnref)
    }

    /// Set the ABI, including the unwind flag.
    #[must_use = "this method does not modify the receiver"]
    pub fn set_abi(mut self, abi: ExternAbi) -> Self {
        let abi_index = abi.as_packed();
        assert!(abi_index <= Self::EXTERN_ABI_MASK);

        self.flags &= !Self::EXTERN_ABI_MASK;
        self.flags |= abi_index;

        self
    }

    /// Set the safety flag, `true` is `Safe`.
    #[must_use = "this method does not modify the receiver"]
    pub fn set_safety(mut self, safety: I::Safety) -> Self {
        if safety.is_safe() {
            self.flags |= Self::SAFE_FLAG;
        } else {
            self.flags &= !Self::SAFE_FLAG;
        }

        self
    }

    /// Set the `is_fnref` flag.
    #[must_use = "this method does not modify the receiver"]
    pub fn set_is_fnref(mut self, is_fnref: bool) -> Self {
        if is_fnref {
            self.flags |= Self::IS_FNREF_FLAG;
        } else {
            self.flags &= !Self::IS_FNREF_FLAG;
        }

        self
    }

    /// Get the ABI, including the unwind flag.
    pub fn abi(self) -> ExternAbi {
        let abi_index = self.flags & Self::EXTERN_ABI_MASK;
        ExternAbi::from_packed(abi_index)
    }

    /// Get the safety flag.
    pub fn is_safe(self) -> bool {
        self.flags & Self::SAFE_FLAG != 0
    }

    /// Returns the safety mode.
    pub fn safety(self) -> I::Safety {
        if self.is_safe() { I::Safety::safe() } else { I::Safety::unsafe_mode() }
    }

    /// Returns `true` if this is a Patina `fnref(A) -> R` type.
    pub fn is_fnref(self) -> bool {
        self.flags & Self::IS_FNREF_FLAG != 0
    }
}

#[derive_where(Clone, Copy, PartialEq, Hash; I: Interner)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, StableHash_NoContext)
)]
#[derive(TypeVisitable_Generic, GenericTypeVisitable, TypeFoldable_Generic, Lift_Generic)]
pub struct FnSig<I: Interner> {
    pub inputs_and_output: I::Tys,
    #[type_visitable(ignore)]
    #[type_foldable(identity)]
    pub fn_sig_kind: FnSigKind<I>,
}

impl<I: Interner> Eq for FnSig<I> {}

impl<I: Interner> FnSig<I> {
    pub fn inputs(self) -> I::FnInputTys {
        self.inputs_and_output.inputs()
    }

    pub fn output(self) -> I::Ty {
        self.inputs_and_output.output()
    }

    pub fn is_fn_trait_compatible(self) -> bool {
        !self.is_fnref() && self.safety().is_safe() && self.abi() == ExternAbi::Rust
    }

    pub fn set_safety(self, safety: I::Safety) -> Self {
        Self { fn_sig_kind: FnSigKind::new(self.abi(), safety, self.is_fnref()), ..self }
    }

    pub fn safety(self) -> I::Safety {
        self.fn_sig_kind.safety()
    }

    pub fn abi(self) -> ExternAbi {
        self.fn_sig_kind.abi()
    }

    pub fn is_fnref(self) -> bool {
        self.fn_sig_kind.is_fnref()
    }

    pub fn dummy() -> Self {
        Self {
            inputs_and_output: Default::default(),
            fn_sig_kind: FnSigKind::new(ExternAbi::Rust, I::Safety::safe(), false),
        }
    }
}

impl<I: Interner> ty::Binder<I, FnSig<I>> {
    #[inline]
    pub fn inputs(self) -> ty::Binder<I, I::FnInputTys> {
        self.map_bound(|fn_sig| fn_sig.inputs())
    }

    #[inline]
    #[track_caller]
    pub fn input(self, index: usize) -> ty::Binder<I, I::Ty> {
        self.map_bound(|fn_sig| fn_sig.inputs().get(index).unwrap())
    }

    pub fn inputs_and_output(self) -> ty::Binder<I, I::Tys> {
        self.map_bound(|fn_sig| fn_sig.inputs_and_output)
    }

    #[inline]
    pub fn output(self) -> ty::Binder<I, I::Ty> {
        self.map_bound(|fn_sig| fn_sig.output())
    }

    pub fn fn_sig_kind(self) -> FnSigKind<I> {
        self.skip_binder().fn_sig_kind
    }

    pub fn is_fnref(self) -> bool {
        self.skip_binder().is_fnref()
    }

    pub fn safety(self) -> I::Safety {
        self.skip_binder().safety()
    }

    pub fn abi(self) -> ExternAbi {
        self.skip_binder().abi()
    }

    pub fn is_fn_trait_compatible(&self) -> bool {
        self.skip_binder().is_fn_trait_compatible()
    }

    // Used to split a single value into the two fields in `TyKind::FnPtr`.
    pub fn split(self) -> (ty::Binder<I, FnSigTys<I>>, FnHeader<I>) {
        let hdr = FnHeader { fn_sig_kind: self.fn_sig_kind() };
        (self.map_bound(|sig| FnSigTys { inputs_and_output: sig.inputs_and_output }), hdr)
    }
}

impl<I: Interner> fmt::Debug for FnSig<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig = self;
        let FnSig { inputs_and_output: _, fn_sig_kind } = sig;

        write!(f, "{}", fn_sig_kind.safety().prefix_str())?;
        if fn_sig_kind.abi() != ExternAbi::Rust {
            write!(f, "extern \"{:?}\" ", fn_sig_kind.abi())?;
        }

        if fn_sig_kind.is_fnref() {
            write!(f, "fnref(")?;
        } else {
            write!(f, "fn(")?;
        }
        let inputs = sig.inputs();
        for (i, ty) in inputs.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{ty:?}")?;
        }
        write!(f, ")")?;

        let output = sig.output();
        match output.kind() {
            Tuple(list) if list.is_empty() => Ok(()),
            _ => write!(f, " -> {:?}", sig.output()),
        }
    }
}

// FIXME: this is a distinct type because we need to define `Encode`/`Decode`
// impls in this crate for `Binder<I, I::Ty>`.
#[derive_where(Clone, Copy, PartialEq, Hash; I: Interner)]
#[cfg_attr(feature = "nightly", derive(StableHash_NoContext))]
#[derive(TypeVisitable_Generic, GenericTypeVisitable, TypeFoldable_Generic, Lift_Generic)]
pub struct UnsafeBinderInner<I: Interner>(ty::Binder<I, I::Ty>);

impl<I: Interner> Eq for UnsafeBinderInner<I> {}

impl<I: Interner> From<ty::Binder<I, I::Ty>> for UnsafeBinderInner<I> {
    fn from(value: ty::Binder<I, I::Ty>) -> Self {
        UnsafeBinderInner(value)
    }
}

impl<I: Interner> From<UnsafeBinderInner<I>> for ty::Binder<I, I::Ty> {
    fn from(value: UnsafeBinderInner<I>) -> Self {
        value.0
    }
}

impl<I: Interner> fmt::Debug for UnsafeBinderInner<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<I: Interner> Deref for UnsafeBinderInner<I> {
    type Target = ty::Binder<I, I::Ty>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "nightly")]
impl<I: Interner, E: rustc_serialize::Encoder> rustc_serialize::Encodable<E>
    for UnsafeBinderInner<I>
where
    I::Ty: rustc_serialize::Encodable<E>,
    I::BoundVarKinds: rustc_serialize::Encodable<E>,
{
    fn encode(&self, e: &mut E) {
        self.bound_vars().encode(e);
        self.as_ref().skip_binder().encode(e);
    }
}

#[cfg(feature = "nightly")]
impl<I: Interner, D: rustc_serialize::Decoder> rustc_serialize::Decodable<D>
    for UnsafeBinderInner<I>
where
    I::Ty: TypeVisitable<I> + rustc_serialize::Decodable<D>,
    I::BoundVarKinds: rustc_serialize::Decodable<D>,
{
    fn decode(decoder: &mut D) -> Self {
        let bound_vars = rustc_serialize::Decodable::decode(decoder);
        UnsafeBinderInner(ty::Binder::bind_with_vars(
            rustc_serialize::Decodable::decode(decoder),
            bound_vars,
        ))
    }
}

// This is just a `FnSig` without the `FnHeader` fields.
#[derive_where(Clone, Copy, Debug, PartialEq, Hash; I: Interner)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, StableHash_NoContext)
)]
#[derive(TypeVisitable_Generic, GenericTypeVisitable, TypeFoldable_Generic, Lift_Generic)]
pub struct FnSigTys<I: Interner> {
    pub inputs_and_output: I::Tys,
}

impl<I: Interner> Eq for FnSigTys<I> {}

impl<I: Interner> FnSigTys<I> {
    pub fn inputs(self) -> I::FnInputTys {
        self.inputs_and_output.inputs()
    }

    pub fn output(self) -> I::Ty {
        self.inputs_and_output.output()
    }
}

impl<I: Interner> ty::Binder<I, FnSigTys<I>> {
    // Used to combine the two fields in `TyKind::FnPtr` into a single value.
    pub fn with(self, hdr: FnHeader<I>) -> ty::Binder<I, FnSig<I>> {
        self.map_bound(|sig_tys| FnSig {
            inputs_and_output: sig_tys.inputs_and_output,
            fn_sig_kind: hdr.fn_sig_kind,
        })
    }

    #[inline]
    pub fn inputs(self) -> ty::Binder<I, I::FnInputTys> {
        self.map_bound(|sig_tys| sig_tys.inputs())
    }

    #[inline]
    #[track_caller]
    pub fn input(self, index: usize) -> ty::Binder<I, I::Ty> {
        self.map_bound(|sig_tys| sig_tys.inputs().get(index).unwrap())
    }

    pub fn inputs_and_output(self) -> ty::Binder<I, I::Tys> {
        self.map_bound(|sig_tys| sig_tys.inputs_and_output)
    }

    #[inline]
    pub fn output(self) -> ty::Binder<I, I::Ty> {
        self.map_bound(|sig_tys| sig_tys.output())
    }
}

#[derive_where(Clone, Copy, Debug, PartialEq, Hash; I: Interner)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, StableHash_NoContext)
)]
#[derive(TypeVisitable_Generic, GenericTypeVisitable, TypeFoldable_Generic, Lift_Generic)]
pub struct FnHeader<I: Interner> {
    #[type_visitable(ignore)]
    #[type_foldable(identity)]
    pub fn_sig_kind: FnSigKind<I>,
}

impl<I: Interner> FnHeader<I> {
    pub fn is_fnref(self) -> bool {
        self.fn_sig_kind.is_fnref()
    }

    pub fn safety(self) -> I::Safety {
        self.fn_sig_kind.safety()
    }

    pub fn abi(self) -> ExternAbi {
        self.fn_sig_kind.abi()
    }

    pub fn dummy() -> Self {
        Self { fn_sig_kind: FnSigKind::new(ExternAbi::Rust, I::Safety::safe(), false) }
    }
}

impl<I: Interner> Eq for FnHeader<I> {}

#[derive_where(Clone, Copy, Debug, PartialEq, Hash; I: Interner)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, StableHash_NoContext)
)]
#[derive(TypeVisitable_Generic, GenericTypeVisitable, TypeFoldable_Generic, Lift_Generic)]
pub struct CoroutineWitnessTypes<I: Interner> {
    pub types: I::Tys,
    pub assumptions: I::RegionAssumptions,
}

impl<I: Interner> Eq for CoroutineWitnessTypes<I> {}
