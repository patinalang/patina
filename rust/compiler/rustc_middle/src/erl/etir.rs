//! The Erlang translation IR, lowered from THIR.

use rustc_data_structures::packed::Pu128;
use rustc_hir::def_id::DefId;
use rustc_hir::{RangeEnd, TermClass};
use rustc_index::{IndexVec, newtype_index};
use rustc_macros::{TyDecodable, TyEncodable};
use rustc_span::{ByteSymbol, Ident, Span, Symbol};

use crate::erl::binary_pattern::BinarySegmentLayout;
use crate::erl::{ErlCallTarget, ErlModuleResolution};
use crate::ty::Ty;

pub mod visit;

newtype_index! {
    #[encodable]
    #[debug_format = "e{}"]
    pub struct ExprId {}
}

newtype_index! {
    #[encodable]
    #[debug_format = "p{}"]
    pub struct PatId {}
}

newtype_index! {
    #[encodable]
    #[debug_format = "v{}"]
    pub struct VarId {}
}

newtype_index! {
    #[encodable]
    #[debug_format = "l{}"]
    pub struct LoopId {}
}

newtype_index! {
    #[encodable]
    #[debug_format = "j{}"]
    pub struct JoinId {}
}

newtype_index! {
    /// A runtime field of a variant, in declaration order.
    #[encodable]
    #[orderable]
    #[debug_format = "f{}"]
    pub struct FieldIdx {}
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct Body<'tcx> {
    pub params: Box<[PatId]>,
    pub value: ExprId,
    pub exprs: IndexVec<ExprId, Expr<'tcx>>,
    pub pats: IndexVec<PatId, Pat<'tcx>>,
    pub vars: IndexVec<VarId, Var>,
    pub loops: IndexVec<LoopId, Loop>,
    pub joins: IndexVec<JoinId, Join>,
}

impl<'tcx> Body<'tcx> {
    pub fn new() -> Body<'tcx> {
        Body {
            params: Box::new([]),
            value: ExprId::MAX,
            exprs: IndexVec::new(),
            pats: IndexVec::new(),
            vars: IndexVec::new(),
            loops: IndexVec::new(),
            joins: IndexVec::new(),
        }
    }
}

impl<'tcx> std::ops::Index<ExprId> for Body<'tcx> {
    type Output = Expr<'tcx>;

    fn index(&self, id: ExprId) -> &Expr<'tcx> {
        &self.exprs[id]
    }
}

impl<'tcx> std::ops::Index<PatId> for Body<'tcx> {
    type Output = Pat<'tcx>;

    fn index(&self, id: PatId) -> &Pat<'tcx> {
        &self.pats[id]
    }
}

impl<'tcx> std::ops::Index<VarId> for Body<'tcx> {
    type Output = Var;

    fn index(&self, id: VarId) -> &Var {
        &self.vars[id]
    }
}

impl<'tcx> std::ops::Index<LoopId> for Body<'tcx> {
    type Output = Loop;

    fn index(&self, id: LoopId) -> &Loop {
        &self.loops[id]
    }
}

impl<'tcx> std::ops::Index<JoinId> for Body<'tcx> {
    type Output = Join;

    fn index(&self, id: JoinId) -> &Join {
        &self.joins[id]
    }
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct Var {
    pub ident: Ident,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct Expr<'tcx> {
    pub kind: ExprKind<'tcx>,
    pub ty: Ty<'tcx>,
    pub span: Span,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub enum ExprKind<'tcx> {
    Block(Box<[ExprId]>),
    Literal(Lit, Span),
    Var(VarId),
    List { items: Box<[ExprId]>, tail: Option<ExprId> },
    Map { fields: Box<[(ExprId, ExprId)]>, base: Option<ExprId> },
    Tuple { fields: Box<[ExprId]> },
    BinaryPattern { segments: Box<[BinarySegmentExpr]> },
    Field { lhs: ExprId, repr: AdtRepr, position: FieldIdx, name: Symbol },
    TupleField { lhs: ExprId, position: FieldIdx },
    BinaryOp { op: BinaryOp, lhs: ExprId, rhs: ExprId },
    LogicalOp { op: LogicalOp, lhs: ExprId, rhs: ExprId },
    UnaryOp { op: UnaryOp, arg: ExprId },
    Cast { source: ExprId, kind: CastKind },
    Call { callee: Callee<'tcx>, args: Box<[ExprId]> },
    Adt(Box<AdtExpr>),
    Closure { def_id: DefId, params: Box<[PatId]>, body: ExprId },
    // Only a thing between lowering passes; it gets lowered to a `Closure` over its witnesses.
    FnItem { item: Box<ItemRef<'tcx>>, arity: u32, forced_extern: bool },
    Fun { fun: ErlFun, forced_extern: bool },
    Const { def_id: DefId },
    Assign { lhs: PatId, rhs: ExprId },
    Declare { vars: Box<[VarId]> },
    Match { scrutinee: ExprId, arms: Box<[Arm]> },
    Receive { arms: Box<[Arm]>, after: Option<ReceiveAfter> },
    TryCatch { body: ExprId, arms: Box<[CatchArm]>, after: Option<ExprId> },
    Return { value: Option<ExprId> },
    Loop { id: LoopId, body: ExprId },
    Break { target: LoopId, value: Option<ExprId> },
    Continue { target: LoopId },
    Join { joins: Box<[JoinId]>, body: ExprId },
    Jump { target: JoinId },
}

#[derive(Copy, Clone, Debug, TyEncodable, TyDecodable)]
pub enum Lit {
    Str(Symbol),
    Charlist(Symbol),
    Atom(Symbol),
    ByteStr(ByteSymbol),
    Char(char),
    Int { value: Pu128, neg: bool },
    Float(u64),
    Bool(bool),
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct BinarySegmentExpr {
    pub value: ExprId,
    pub layout: BinarySegmentLayout<ExprId>,
    pub span: Span,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, TyEncodable, TyDecodable)]
pub enum BinaryOp {
    Concat,
    Subtract,
    Add,
    Sub,
    Mul,
    Div,
    DivInt,
    Rem,
    Bxor,
    Band,
    Bor,
    Bsl,
    Bsr,
    EqExact,
    NeExact,
    Lt,
    Le,
    Ge,
    Gt,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, TyEncodable, TyDecodable)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, TyEncodable, TyDecodable)]
pub enum UnaryOp {
    Not,
    Bnot,
    Neg,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, TyEncodable, TyDecodable)]
pub enum CastKind {
    Identity,
    IntToFloat,
    FloatToInt,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub enum Callee<'tcx> {
    Value(ExprId),
    // Only a thing between lowering passes; it gets lowered to `Fun`.
    Item { item: Box<ItemRef<'tcx>>, forced_extern: bool },
    Fun { fun: ErlFun, forced_extern: bool },
    Direct(Symbol, Symbol),
}

/// A function, or an associated const of a trait, which an expression refers to.
#[derive(Debug, TyEncodable, TyDecodable)]
pub struct ItemRef<'tcx> {
    pub name: Symbol,
    pub target: ErlCallTarget<'tcx>,
    /// `None` if the reference dispatches through the self witness.
    pub module: Option<ErlModuleResolution>,
}

/// An Erlang function.
#[derive(Debug, TyEncodable, TyDecodable)]
pub struct ErlFun {
    pub module: ErlModuleResolution,
    pub name: Symbol,
    pub arity: u32,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct AdtExpr {
    pub repr: AdtRepr,
    pub base: Option<ExprId>,
    /// The runtime fields that the expression gives a value, in declaration order.
    pub fields: Box<[FieldExpr]>,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct FieldExpr {
    pub position: FieldIdx,
    pub name: Symbol,
    pub value: ExprId,
}

/// The runtime representation of an ADT variant.
#[derive(Copy, Clone, Debug, TyEncodable, TyDecodable)]
pub enum AdtRepr {
    Atom { tag: Symbol },
    Tagged { tag: Symbol },
    Elixir { adt: DefId, variant: DefId },
    NativeRecord { record_module: Symbol, tag: Symbol },
    Transparent { tag: Symbol, variant: DefId, has_siblings: bool },
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct Arm {
    pub pattern: PatId,
    pub guard: Option<ExprId>,
    pub body: ExprId,
    pub span: Span,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct CatchArm {
    pub class: PatId,
    pub reason: PatId,
    pub stacktrace: Option<VarId>,
    pub guard: Option<ExprId>,
    pub body: ExprId,
    pub span: Span,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct ReceiveAfter {
    pub timeout: ExprId,
    pub body: ExprId,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct Loop {
    /// Variables from the outer scope used by the loop
    pub outer_vars: Box<[LoopOuterVar]>,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct Join {
    pub body: ExprId,
    /// Outer variables that may have a different versions at each `Jump`.
    pub params: Box<[VarId]>,
}

#[derive(Copy, Clone, Debug, TyEncodable, TyDecodable)]
pub enum LoopOuterVar {
    /// Only referenced inside the loop.
    Referenced(VarId),
    /// Assigned to inside the loop.
    Reassigned(VarId),
}

impl LoopOuterVar {
    pub fn var(self) -> VarId {
        match self {
            LoopOuterVar::Referenced(var) | LoopOuterVar::Reassigned(var) => var,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, TyEncodable, TyDecodable)]
pub enum PatternInput {
    /// Static typing guarantees that the value has the pattern's type.
    Typed,
    /// The value came from `term` and each unconstrained leaf must be checked against its type.
    UntrustedRepresentation,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct Pat<'tcx> {
    pub kind: PatKind<'tcx>,
    pub ty: Ty<'tcx>,
    pub span: Span,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub enum PatKind<'tcx> {
    Wild { input: PatternInput },
    Binding { var: VarId, is_primary: bool, subpattern: Option<PatId>, input: PatternInput },
    Pin { var: VarId },
    Assignee { var: VarId },
    TermClass { class: TermClass, subpattern: PatId },
    Tuple { fields: Box<[PatId]> },
    Adt(Box<AdtPat<'tcx>>),
    Literal(Lit),
    Range(Box<PatRange<'tcx>>),
    Map { fields: Box<[(ExprId, PatId)]> },
    BinaryPattern { segments: Box<[BinarySegmentPat]> },
    List { items: Box<[PatId]>, tail: Option<PatId> },
    Or { pats: Box<[PatId]> },
}

impl<'tcx> PatKind<'tcx> {
    pub fn is_wild(&self) -> bool {
        matches!(self, PatKind::Wild { .. })
    }
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct AdtPat<'tcx> {
    pub repr: AdtRepr,
    pub fields: IndexVec<FieldIdx, FieldPat<'tcx>>,
    pub input: PatternInput,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct FieldPat<'tcx> {
    pub name: Symbol,
    pub ty: Ty<'tcx>,
    /// A wildcard if the pattern doesn't mention the field.
    pub pattern: PatId,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct PatRange<'tcx> {
    pub lo: Option<ExprId>,
    pub hi: Option<ExprId>,
    pub end: RangeEnd,
    pub ty: Ty<'tcx>,
    pub input: PatternInput,
}

#[derive(Debug, TyEncodable, TyDecodable)]
pub struct BinarySegmentPat {
    pub pattern: PatId,
    pub layout: BinarySegmentLayout<ExprId>,
    pub span: Span,
}
