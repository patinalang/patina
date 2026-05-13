// Comments in this file come from the Erlang documentation
// https://www.erlang.org/doc/apps/erts/absform.html
// available under Apache 2.0 license

#![allow(unused)]

use num_bigint::BigUint;
use serde::{Serialize, Serializer};

/// The word `ANNO` in this section represents an annotation, and denotes among
/// other things the number of the line in the source file where the construction
/// occurred. See `m:erl_anno` for details. Several instances of `ANNO` in the same
/// construction can denote different annotations.
#[derive(serde::Serialize, Debug, Clone, Copy)]
#[serde(rename = "__elide__")]
pub(crate) struct Anno {
    pub line: usize,
    pub column: usize,
}

/* Module Declarations and Forms

A module declaration consists of a sequence of forms, which are either function
declarations or attributes.
*/

/// If D is a module declaration consisting of the forms `F_1`, ..., `F_k`, then
/// Rep(D) = `[Rep(F_1), ..., Rep(F_k)]`.
#[derive(serde::Serialize, Debug)]
pub(crate) struct Module<'a>(pub Vec<ModuleForm<'a>>);

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ModuleForm<'a> {
    /// If F is an attribute `-export([Fun_1/A_1, ..., Fun_k/A_k])`, then Rep(F) =
    /// `{attribute,ANNO,export,[{Fun_1,A_1}, ..., {Fun_k,A_k}]}`.
    #[serde(rename = "__attribute__export")]
    ExportAttr(Anno, Vec<NameArity<'a>>),
    /// If F is an attribute `-import(Mod,[Fun_1/A_1, ..., Fun_k/A_k])`, then Rep(F) =
    /// `{attribute,ANNO,import,{Mod,[{Fun_1,A_1}, ..., {Fun_k,A_k}]}}`.
    #[serde(rename = "__attribute__import")]
    ImportAttr(Anno, ImportModFuns<'a>),
    /// If F is an attribute `-module(Mod)`, then Rep(F) =
    /// `{attribute,ANNO,module,Mod}`.
    #[serde(rename = "__attribute__module")]
    ModuleAttr(Anno, &'a str),
    /// If F is an attribute `-file(File,Line)`, then Rep(F) =
    /// `{attribute,ANNO,file,{File,Line}}`.
    #[serde(rename = "__attribute__file")]
    FileAttr(Anno, FileLine),
    /// If F is a function declaration `Name Fc_1 ; ... ; Name Fc_k`, where each
    /// `Fc_i` is a function clause with a pattern sequence of the same length
    /// `Arity`, then Rep(F) =
    /// `{function,ANNO,Name,Arity,[Rep(Fc_1), ...,Rep(Fc_k)]}`.
    Function(Anno, &'a str, usize, Vec<Clause<'a>>),
    /// If F is a function specification `-Spec Name Ft_1; ...; Ft_k`, where `Spec` is
    /// either the atom `spec` or the atom `callback`, and each `Ft_i` is a possibly
    /// constrained function type with an argument sequence of the same length
    /// `Arity`, then Rep(F) =
    /// `{attribute,ANNO,Spec,{{Name,Arity},[Rep(Ft_1), ..., Rep(Ft_k)]}}`.
    #[serde(rename = "attribute")]
    SpecAttr(Anno, SpecType, Spec<'a>),
    /// If F is a function specification `-spec Mod:Name Ft_1; ...; Ft_k`, where each
    /// `Ft_i` is a possibly constrained function type with an argument sequence of
    /// the same length `Arity`, then Rep(F) =
    /// `{attribute,ANNO,spec,{{Mod,Name,Arity},[Rep(Ft_1), ..., Rep(Ft_k)]}}`.
    #[serde(rename = "__attribute__spec")]
    RemoteSpecAttr(Anno, RemoteSpec<'a>),
    /// If F is a record declaration `-record(Name,{V_1, ..., V_k})`, where each `V_i`
    /// is a record field, then Rep(F) =
    /// `{attribute,ANNO,record,{Name,[Rep(V_1), ..., Rep(V_k)]}}`. For Rep(V), see
    /// below.
    #[serde(rename = "__attribute__record")]
    RecordAttr(Anno, RecordDecl<'a>),
    /// If F is a type declaration `-Type Name(V_1, ..., V_k) :: T`, where `Type` is
    /// either the atom `type` or the atom `opaque`, each `V_i` is a type variable,
    /// and `T` is a type, then Rep(F) =
    /// `{attribute,ANNO,Type,{Name,Rep(T),[Rep(V_1), ..., Rep(V_k)]}}`.
    #[serde(rename = "attribute")]
    TypeAttr(Anno, TypeDeclType, Box<TypeDecl<'a>>),
    /// If F is a wild attribute `-A(T)`, then Rep(F) = `{attribute,ANNO,A,T}`.
    #[serde(rename = "attribute")]
    WildAttr(Anno, &'a str, Vec<&'a str>),
}

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__elide__")]
pub(crate) struct NameArity<'a>(pub &'a str, pub usize);

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__elide__")]
pub(crate) struct FileLine(pub Vec<char>, pub usize);

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__elide__")]
pub(crate) struct ModNameArity<'a>(pub &'a str, pub &'a str, pub usize);

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__elide__")]
pub(crate) struct ImportModFuns<'a>(pub &'a str, pub Vec<NameArity<'a>>);

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__elide__")]
pub(crate) struct Spec<'a>(pub NameArity<'a>, pub Vec<Type<'a>>);

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__elide__")]
pub(crate) struct RemoteSpec<'a>(pub ModNameArity<'a>, pub Vec<Type<'a>>);

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SpecType {
    Spec,
    Callback,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__elide__")]
pub(crate) struct RecordDecl<'a>(pub &'a str, pub Vec<RecordFieldDecl<'a>>);

#[derive(serde::Serialize, Debug)]
pub(crate) enum RecordFieldDecl<'a> {
    Untyped(RecordField<Atom<'a>, Expr<'a>>),
    Typed(Box<TypedRecordField<Atom<'a>, Expr<'a>, Type<'a>>>),
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TypeDeclType {
    Type,
    Opaque,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__elide__")]
pub(crate) struct TypeDecl<'a>(pub &'a str, pub Type<'a>, pub Vec<Type<'a>>);

/* Record Fields

Each field in a record declaration can have an optional, explicit, default
initializer expression, and an optional type.
*/

/// If V is `A`, then Rep(V) = `{record_field,ANNO,Rep(A)}`.
///
/// If V is `A = E`, where `E` is an expression, then Rep(V) =
/// `{record_field,ANNO,Rep(A),Rep(E)}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "record_field")]
pub(crate) struct RecordField<A: Serialize, E: Serialize>(
    pub Anno,
    pub A,
    #[serde(skip_serializing_if = "Option::is_none")] pub Option<E>,
);

/// If V is `A :: T`, where `T` is a type, then Rep(V) =
/// `{typed_record_field,{record_field,ANNO,Rep(A)},Rep(T)}`.
///
/// If V is `A = E :: T`, where `E` is an expression and `T` is a type, then
/// Rep(V) = `{typed_record_field,{record_field,ANNO,Rep(A),Rep(E)},Rep(T)}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "typed_record_field")]
pub(crate) struct TypedRecordField<A: Serialize, E: Serialize, T: Serialize>(
    pub RecordField<A, E>,
    pub T,
);

/* Atomic Literals */

/// There are five kinds of atomic literals, which are represented in the same way
/// in patterns, expressions, and guards.
/// Notice that negative integer and float literals do not occur as such; they are
/// parsed as an application of the unary negation operator.
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AtomicLiteral<'a> {
    /// If L is an atom literal, then Rep(L) = `{atom,ANNO,L}`.
    Atom(Anno, &'a str),
    /// If L is a character literal, then Rep(L) = `{char,ANNO,L}`.
    Char(Anno, char),
    /// If L is a float literal, then Rep(L) = `{float,ANNO,L}`.
    Float(Anno, f64),
    /// If L is an integer literal, then Rep(L) = `{integer,ANNO,L}`.
    Integer(Anno, #[serde(serialize_with = "serialize_biguint")] BigUint),
    /// If L is a string literal consisting of the characters `C_1`, ..., `C_k`, then
    /// Rep(L) = `{string,ANNO,[C_1, ..., C_k]}`.
    String(Anno, Vec<char>),
}

/// If L is an atom literal, then Rep(L) = `{atom,ANNO,L}`.
/// Moved outside of AtomicLiteral because it's used individually in many places.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "atom")]
pub(crate) struct Atom<'a>(pub Anno, pub &'a str);

/// If L is an integer literal, then Rep(L) = `{integer,ANNO,L}`.
/// Moved outside of AtomicLiteral because it's used individually in many places.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "integer")]
pub(crate) struct Integer(pub Anno, #[serde(serialize_with = "serialize_biguint")] pub BigUint);

/* Patterns */

/// If Ps is a sequence of patterns `P_1, ..., P_k`, then Rep(Ps) =
/// `[Rep(P_1), ..., Rep(P_k)]`. Such sequences occur as the list of arguments to a
/// function or fun.
/// Individual patterns are represented as follows.
/// Notice that every pattern has the same source form as some expression, and is
/// represented in the same way as the corresponding expression.
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Pattern<'a> {
    /// If P is an atomic literal `L`, then Rep(P) = Rep(L).
    AtomicLiteral(AtomicLiteral<'a>),
    /// If P is a bitstring pattern `<<P_1:Size_1/TSL_1, ..., P_k:Size_k/TSL_k>>`,
    /// where each `Size_i` is an expression that can be evaluated to an integer, and
    /// each `TSL_i` is a type specificer list, then Rep(P) =
    /// `{bin,ANNO,[{bin_element,ANNO,Rep(P_1),Rep(Size_1),Rep(TSL_1)}, ..., {bin_element,ANNO,Rep(P_k),Rep(Size_k),Rep(TSL_k)}]}`.
    /// For Rep(TSL), see below. An omitted `Size_i` is represented by `default`. An
    /// omitted `TSL_i` is represented by `default`.
    Bin(Anno, Vec<BinElement<'a, Pattern<'a>>>),
    /// If P is a compound pattern `P_1 = P_2`, then Rep(P) =
    /// `{match,ANNO,Rep(P_1),Rep(P_2)}`.
    Match(Anno, Box<Pattern<'a>>, Box<Pattern<'a>>),
    /// If P is a cons pattern `[P_h | P_t]`, then Rep(P) =
    /// `{cons,ANNO,Rep(P_h),Rep(P_t)}`.
    Cons(Anno, Box<Pattern<'a>>, Box<Pattern<'a>>),
    /// If P is a map pattern `#{A_1, ..., A_k}`, where each `A_i` is an association
    /// `P_i_1 := P_i_2`, then Rep(P) = `{map,ANNO,[Rep(A_1), ..., Rep(A_k)]}`. For
    /// Rep(A), see below.
    Map(Anno, Vec<MapFieldExact<Pattern<'a>, Pattern<'a>>>),
    /// If P is a nil pattern `[]`, then Rep(P) = `{nil,ANNO}`.
    #[rustfmt::skip]
    Nil { anno: Anno },
    /// If P is an operator pattern `P_1 Op P_2`, where `Op` is a binary operator
    /// (this is either an occurrence of `++` applied to a literal string or character
    /// list, or an occurrence of an expression that can be evaluated to a number at
    /// compile time), then Rep(P) = `{op,ANNO,Op,Rep(P_1),Rep(P_2)}`.
    #[serde(rename = "op")]
    BinaryOp(Anno, &'a str, Box<Pattern<'a>>, Box<Pattern<'a>>),
    /// If P is an operator pattern `Op P_0`, where `Op` is a unary operator (this is
    /// an occurrence of an expression that can be evaluated to a number at compile
    /// time), then Rep(P) = `{op,ANNO,Op,Rep(P_0)}`.
    #[serde(rename = "op")]
    UnaryOp(Anno, &'a str, Box<Pattern<'a>>),
    /// If P is a parenthesized pattern `( P_0 )`, then Rep(P) = `Rep(P_0)`, that is,
    /// parenthesized patterns cannot be distinguished from their bodies.
    Parenthesized(Box<Pattern<'a>>),
    /// If P is a record field index pattern `#Name.Field`, where `Field` is an atom,
    /// then Rep(P) = `{record_index,ANNO,Name,Rep(Field)}`.
    RecordIndex(Anno, &'a str, Atom<'a>),
    /// If P is a record pattern `#Name{Field_1=P_1, ..., Field_k=P_k}`, where each
    /// `Field_i` is an atom or `_`, then Rep(P) =
    /// `{record,ANNO,Name,[{record_field,ANNO,Rep(Field_1),Rep(P_1)}, ..., {record_field,ANNO,Rep(Field_k),Rep(P_k)}]}`.
    Record(Anno, &'a str, Vec<RecordField<AtomOrVar<'a>, Pattern<'a>>>),
    /// If P is a tuple pattern `{P_1, ..., P_k}`, then Rep(P) =
    /// `{tuple,ANNO,[Rep(P_1), ..., Rep(P_k)]}`.
    Tuple(Anno, Vec<Pattern<'a>>),
    /// If P is a universal pattern `_`, then Rep(P) = `{var,ANNO,'_'}`.
    ///
    /// If P is a variable pattern `V`, then Rep(P) = `{var,ANNO,A}`, where A is an
    /// atom with a printname consisting of the same characters as `V`.
    Var(Anno, &'a str),
}

/* Expressions */

/// A body B is a non-empty sequence of expressions `E_1, ..., E_k`, and Rep(B) =
/// `[Rep(E_1), ..., Rep(E_k)]`.
#[derive(serde::Serialize, Debug)]
pub(crate) struct Body<'a>(pub Vec<Expr<'a>>);

/// An expression E is one of the following:
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Expr<'a> {
    /// If E is an atomic literal `L`, then Rep(E) = Rep(L).
    AtomicLiteral(AtomicLiteral<'a>),
    /// If E is a bitstring comprehension `<<E_0 || Q_1, ..., Q_k>>`, where each `Q_i`
    /// is a qualifier, then Rep(E) = `{bc,ANNO,Rep(E_0),[Rep(Q_1), ..., Rep(Q_k)]}`.
    /// For Rep(Q), see below.
    Bc(Anno, Box<Expr<'a>>, Vec<Qualifier<'a>>),
    /// If E is a bitstring constructor `<<E_1:Size_1/TSL_1, ..., E_k:Size_k/TSL_k>>`,
    /// where each `Size_i` is an expression and each `TSL_i` is a type specificer
    /// list, then Rep(E) =
    /// `{bin,ANNO,[{bin_element,ANNO,Rep(E_1),Rep(Size_1),Rep(TSL_1)}, ..., {bin_element,ANNO,Rep(E_k),Rep(Size_k),Rep(TSL_k)}]}`.
    /// For Rep(TSL), see below. An omitted `Size_i` is represented by `default`. An
    /// omitted `TSL_i` is represented by `default`.
    Bin(Anno, Vec<BinElement<'a, Expr<'a>>>),
    /// If E is a block expression `begin B end`, where `B` is a body, then Rep(E) =
    /// `{block,ANNO,Rep(B)}`.
    Block(Anno, Body<'a>),
    /// If E is a case expression `case E_0 of Cc_1 ; ... ; Cc_k end`, where `E_0` is
    /// an expression and each `Cc_i` is a case clause, then Rep(E) =
    /// `{'case',ANNO,Rep(E_0),[Rep(Cc_1), ..., Rep(Cc_k)]}`.
    Case(Anno, Box<Expr<'a>>, Vec<Clause<'a>>),
    /// If E is a catch expression `catch E_0`, then Rep(E) =
    /// `{'catch',ANNO,Rep(E_0)}`.
    Catch(Anno, Box<Expr<'a>>),
    /// If E is a cons skeleton `[E_h | E_t]`, then Rep(E) =
    /// `{cons,ANNO,Rep(E_h),Rep(E_t)}`.
    Cons(Anno, Box<Expr<'a>>, Box<Expr<'a>>),
    /// If E is a fun expression `fun Name/Arity`, then Rep(E) =
    /// `{'fun',ANNO,{function,Name,Arity}}`.
    #[serde(rename = "fun")]
    LocalFun(Anno, Function<'a>),
    /// If E is a fun expression `fun Module:Name/Arity`, then Rep(E) =
    /// `{'fun',ANNO,{function,Rep(Module),Rep(Name),Rep(Arity)}}`.
    #[serde(rename = "fun")]
    RemoteFun(Anno, RemoteFunction<'a>),
    /// If E is a fun expression `fun Fc_1 ; ... ; Fc_k end`, where each `Fc_i` is a
    /// function clause, then Rep(E) =
    /// `{'fun',ANNO,{clauses,[Rep(Fc_1), ..., Rep(Fc_k)]}}`.
    #[serde(rename = "fun")]
    ClausedFun(Anno, Clauses<'a>),
    /// If E is a fun expression `fun Name Fc_1 ; ... ; Name Fc_k end`, where `Name`
    /// is a variable and each `Fc_i` is a function clause, then Rep(E) =
    /// `{named_fun,ANNO,Name,[Rep(Fc_1), ..., Rep(Fc_k)]}`.
    NamedFun(Anno, &'a str, Vec<Clause<'a>>),
    /// If E is a function call `E_0(E_1, ..., E_k)`, then Rep(E) =
    /// `{call,ANNO,Rep(E_0),[Rep(E_1), ..., Rep(E_k)]}`.
    #[serde(rename = "call")]
    LocalCall(Anno, Box<Expr<'a>>, Vec<Expr<'a>>),
    /// If E is a function call `E_m:E_0(E_1, ..., E_k)`, then Rep(E) =
    /// `{call,ANNO,{remote,ANNO,Rep(E_m),Rep(E_0)},[Rep(E_1), ..., Rep(E_k)]}`.
    #[serde(rename = "call")]
    RemoteCall(Anno, Remote<Expr<'a>>, Vec<Expr<'a>>),
    /// If E is an if expression `if Ic_1 ; ... ; Ic_k end`, where each `Ic_i` is an
    /// if clause, then Rep(E) = `{'if',ANNO,[Rep(Ic_1), ..., Rep(Ic_k)]}`.
    If(Anno, Vec<Clause<'a>>),
    /// If E is a list comprehension `[E_0 || Q_1, ..., Q_k]`, where each `Q_i` is a
    /// qualifier, then Rep(E) = `{lc,ANNO,Rep(E_0),[Rep(Q_1), ..., Rep(Q_k)]}`. For
    /// Rep(Q), see below.
    Lc(Anno, Box<Expr<'a>>, Vec<Qualifier<'a>>),
    /// If E is a map comprehension `#{E_0 || Q_1, ..., Q_k}`, where `E_0` is an
    /// association `K => V` and each `Q_i` is a qualifier, then Rep(E) =
    /// `{mc,ANNO,Rep(E_0),[Rep(Q_1), ..., Rep(Q_k)]}`. For Rep(E_0) and Rep(Q), see
    /// below.
    Mc(Anno, Box<Expr<'a>>, Vec<Qualifier<'a>>),
    /// If E is a map creation `#{A_1, ..., A_k}`, where each `A_i` is an association
    /// `E_i_1 => E_i_2`, then Rep(E) = `{map,ANNO,[Rep(A_1), ..., Rep(A_k)]}`. For
    /// Rep(A), see below.
    #[serde(rename = "map")]
    MapCreate(Anno, Vec<MapFieldAssoc<Expr<'a>, Expr<'a>>>),
    /// If E is a map update `E_0#{A_1, ..., A_k}`, where each `A_i` is an association
    /// `E_i_1 => E_i_2` or `E_i_1 := E_i_2`, then Rep(E) =
    /// `{map,ANNO,Rep(E_0),[Rep(A_1), ..., Rep(A_k)]}`. For Rep(A), see below.
    #[serde(rename = "map")]
    MapUpdate(Anno, Box<Expr<'a>>, Vec<MapField<Expr<'a>, Expr<'a>>>),
    /// If E is a match operator expression `P = E_0`, where `P` is a pattern, then
    /// Rep(E) = `{match,ANNO,Rep(P),Rep(E_0)}`.
    Match(Anno, Pattern<'a>, Box<Expr<'a>>),
    /// If E is a conditional match operator expression `P ?= E_0`, where `P` is a
    /// pattern, then Rep(E) = `{maybe_match,ANNO,Rep(P),Rep(E_0)}`.
    MaybeMatch(Anno, Pattern<'a>, Box<Expr<'a>>),
    /// If E is a maybe expression `maybe B end`, where `B` is a body then Rep(E) =
    /// `{'maybe',ANNO,Rep(B)}`.
    Maybe(Anno, Body<'a>),
    /// If E is a maybe expression `maybe B else Ec_1 ; ... ; Ec_k end`, where `B` is
    /// a body and each `Ec_i` is an else clause then Rep(E) =
    /// `{'maybe',ANNO,Rep(B),{'else',ANNO,[Rep(Ec_1), ..., Rep(Ec_k)]}}`.
    #[serde(rename = "maybe")]
    MaybeElse(Anno, Body<'a>, Else<'a>),
    /// If E is nil, `[]`, then Rep(E) = `{nil,ANNO}`.
    #[rustfmt::skip]
    Nil { anno: Anno },
    /// If E is an operator expression `E_1 Op E_2`, where `Op` is a binary operator
    /// other than match operator `=`, then Rep(E) = `{op,ANNO,Op,Rep(E_1),Rep(E_2)}`.
    #[serde(rename = "op")]
    BinaryOp(Anno, &'a str, Box<Expr<'a>>, Box<Expr<'a>>),
    /// If E is an operator expression `Op E_0`, where `Op` is a unary operator, then
    /// Rep(E) = `{op,ANNO,Op,Rep(E_0)}`.
    #[serde(rename = "op")]
    UnaryOp(Anno, &'a str, Box<Expr<'a>>),
    /// If E is a parenthesized expression `( E_0 )`, then Rep(E) = `Rep(E_0)`, that
    /// is, parenthesized expressions cannot be distinguished from their bodies.
    Parenthesized(Box<Expr<'a>>),
    /// If E is a receive expression `receive Cc_1 ; ... ; Cc_k end`, where each
    /// `Cc_i` is a case clause, then Rep(E) =
    /// `{'receive',ANNO,[Rep(Cc_1), ..., Rep(Cc_k)]}`.
    Receive(Anno, Vec<Clause<'a>>),
    /// If E is a receive expression `receive Cc_1 ; ... ; Cc_k after E_0 -> B_t end`,
    /// where each `Cc_i` is a case clause, `E_0` is an expression, and `B_t` is a
    /// body, then Rep(E) =
    /// `{'receive',ANNO,[Rep(Cc_1), ..., Rep(Cc_k)],Rep(E_0),Rep(B_t)}`.
    #[serde(rename = "receive")]
    ReceiveAfter(Anno, Vec<Clause<'a>>, Box<Expr<'a>>, Body<'a>),
    /// If E is a record creation `#Name{Field_1=E_1, ..., Field_k=E_k}`, where each
    /// `Field_i` is an atom or `_`, then Rep(E) =
    /// `{record,ANNO,Name,[{record_field,ANNO,Rep(Field_1),Rep(E_1)}, ..., {record_field,ANNO,Rep(Field_k),Rep(E_k)}]}`.
    #[serde(rename = "record")]
    RecordCreate(Anno, &'a str, Vec<RecordField<AtomOrVar<'a>, Expr<'a>>>),
    /// If E is a record field access `E_0#Name.Field`, where `Field` is an atom, then
    /// Rep(E) = `{record_field,ANNO,Rep(E_0),Name,Rep(Field)}`.
    RecordField(Anno, Box<Expr<'a>>, &'a str, Atom<'a>),
    /// If E is a record field index `#Name.Field`, where `Field` is an atom, then
    /// Rep(E) = `{record_index,ANNO,Name,Rep(Field)}`.
    RecordIndex(Anno, &'a str, Atom<'a>),
    /// If E is a record update `E_0#Name{Field_1=E_1, ..., Field_k=E_k}`, where each
    /// `Field_i` is an atom, then Rep(E) =
    /// `{record,ANNO,Rep(E_0),Name,[{record_field,ANNO,Rep(Field_1),Rep(E_1)}, ..., {record_field,ANNO,Rep(Field_k),Rep(E_k)}]}`.
    #[serde(rename = "record")]
    RecordUpdate(Anno, Box<Expr<'a>>, &'a str, Vec<RecordField<Atom<'a>, Expr<'a>>>),
    /// If E is a tuple skeleton `{E_1, ..., E_k}`, then Rep(E) =
    /// `{tuple,ANNO,[Rep(E_1), ..., Rep(E_k)]}`.
    Tuple(Anno, Vec<Expr<'a>>),
    /// If E is a try expression `try B catch Tc_1 ; ... ; Tc_k end`, where `B` is a
    /// body and each `Tc_i` is a catch clause, then Rep(E) =
    /// `{'try',ANNO,Rep(B),[],[Rep(Tc_1), ..., Rep(Tc_k)],[]}`.
    ///
    /// If E is a try expression
    /// `try B of Cc_1 ; ... ; Cc_k catch Tc_1 ; ... ; Tc_n end`, where `B` is a body,
    /// each `Cc_i` is a case clause, and each `Tc_j` is a catch clause, then Rep(E) =
    /// `{'try',ANNO,Rep(B),[Rep(Cc_1), ..., Rep(Cc_k)],[Rep(Tc_1), ..., Rep(Tc_n)],[]}`.
    ///
    /// If E is a try expression `try B after A end`, where `B` and `A` are bodies,
    /// then Rep(E) = `{'try',ANNO,Rep(B),[],[],Rep(A)}`.
    ///
    /// If E is a try expression `try B of Cc_1 ; ... ; Cc_k after A end`, where `B`
    /// and `A` are a bodies, and each `Cc_i` is a case clause, then Rep(E) =
    /// `{'try',ANNO,Rep(B),[Rep(Cc_1), ..., Rep(Cc_k)],[],Rep(A)}`.
    ///
    /// If E is a try expression
    /// `try B of Cc_1 ; ... ; Cc_k catch Tc_1 ; ... ; Tc_n after A end`, where `B`
    /// and `A` are a bodies, each `Cc_i` is a case clause, and each `Tc_j` is a catch
    /// clause, then Rep(E) =
    /// `{'try',ANNO,Rep(B),[Rep(Cc_1), ..., Rep(Cc_k)],[Rep(Tc_1), ..., Rep(Tc_n)],Rep(A)}`.
    Try {
        anno: Anno,
        body: Body<'a>,
        of: Vec<Clause<'a>>,
        catch: Vec<CatchClause<'a>>,
        after: Body<'a>,
    },
    /// If E is a variable `V`, then Rep(E) = `{var,ANNO,A}`, where `A` is an atom
    /// with a printname consisting of the same characters as `V`.
    Var(Anno, &'a str),
}

/// If E is a fun expression `fun Name/Arity`, then Rep(E) =
/// `{'fun',ANNO,{function,Name,Arity}}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "function")]
pub(crate) struct Function<'a> {
    pub name: &'a str,
    pub arity: usize,
}

/// If E is a fun expression `fun Module:Name/Arity`, then Rep(E) =
/// `{'fun',ANNO,{function,Rep(Module),Rep(Name),Rep(Arity)}}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "function")]
pub(crate) struct RemoteFunction<'a>(pub Box<Expr<'a>>, pub Box<Expr<'a>>, pub Box<Expr<'a>>);

/// If E is a fun expression `fun Fc_1 ; ... ; Fc_k end`, where each `Fc_i` is a
/// function clause, then Rep(E) =
/// `{'fun',ANNO,{clauses,[Rep(Fc_1), ..., Rep(Fc_k)]}}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "clauses")]
pub(crate) struct Clauses<'a> {
    pub clauses: Vec<Clause<'a>>,
}

/// If E is a function call `E_m:E_0(E_1, ..., E_k)`, then Rep(E) =
/// `{call,ANNO,{remote,ANNO,Rep(E_m),Rep(E_0)},[Rep(E_1), ..., Rep(E_k)]}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "remote")]
pub(crate) struct Remote<T: Serialize>(pub Anno, pub Box<T>, pub Box<T>);

/// If E is a maybe expression `maybe B else Ec_1 ; ... ; Ec_k end`, where `B` is
/// a body and each `Ec_i` is an else clause then Rep(E) =
/// `{'maybe',ANNO,Rep(B),{'else',ANNO,[Rep(Ec_1), ..., Rep(Ec_k)]}}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "else")]
pub(crate) struct Else<'a> {
    pub anno: Anno,
    pub clauses: Vec<Clause<'a>>,
}

/* Qualifiers */

/// A qualifier Q is one of the following.
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Qualifier<'a> {
    /// If Q is a filter `E`, where `E` is an expression, then Rep(Q) = `Rep(E)`.
    Filter(Box<Expr<'a>>),
    /// If Q is a zip generator `Q_1 && ...&& Q_k]`, where each `Q_i` is
    /// a non-zip generator, then Rep(E) = `{zip,ANNO,[Rep(Q_1), ..., Rep(Q_k)]}`.
    /// For Rep(Q), see below.
    Zip(Anno, Vec<Generator<'a>>),
    Generator(Box<Generator<'a>>),
}

/// A qualifier Q is one of the following (cont)
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Generator<'a> {
    /// If Q is a list generator `P <- E`, where `P` is a pattern and `E` is an
    /// expression, then Rep(Q) = `{generate,ANNO,Rep(P),Rep(E)}`.
    Generate(Anno, Pattern<'a>, Expr<'a>),
    /// If Q is a list generator `P <:- E`, where `P` is a pattern and `E` is an
    /// expression, then Rep(Q) = `{generate_strict,ANNO,Rep(P),Rep(E)}`.
    GenerateStrict(Anno, Pattern<'a>, Expr<'a>),
    /// If Q is a bitstring generator `P <= E`, where `P` is a pattern and `E` is an
    /// expression, then Rep(Q) = `{b_generate,ANNO,Rep(P),Rep(E)}`.
    BGenerate(Anno, Pattern<'a>, Expr<'a>),
    /// If Q is a bitstring generator `P <:= E`, where `P` is a pattern and `E` is an
    /// expression, then Rep(Q) = `{b_generate_strict,ANNO,Rep(P),Rep(E)}`.
    BGenerateStrict(Anno, Pattern<'a>, Expr<'a>),
    /// If Q is a map generator `P <- E`, where `P` is an association pattern
    /// `P_1 := P_2` and `E` is an expression, then Rep(Q) =
    /// `{m_generate,ANNO,Rep(P),Rep(E)}`. For Rep(P), see below.
    MGenerate(Anno, Pattern<'a>, Expr<'a>),
    /// If Q is a map generator `P <:- E`, where `P` is an association pattern
    /// `P_1 := P_2` and `E` is an expression, then Rep(Q) =
    /// `{m_generate_strict,ANNO,Rep(P),Rep(E)}`.
    MGenerateStrict(Anno, Pattern<'a>, Expr<'a>),
}

/* Bitstring Element Type Specifiers */

#[derive(serde::Serialize, Debug)]
#[serde(rename = "bin_element")]
pub(crate) struct BinElement<'a, V: Serialize> {
    pub anno: Anno,
    pub value: V,
    pub size: BinElemSize<'a>,
    pub tsl: BinElemTsl,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BinElemSize<'a> {
    Default,
    Size(Box<Expr<'a>>),
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BinElemTsl {
    Default,
    Tsl(Vec<Tsl>),
}

/// A type specifier list TSL for a bitstring element is a sequence of type
/// specifiers `TS_1 - ... - TS_k`, and Rep(TSL) = `[Rep(TS_1), ..., Rep(TS_k)]`.
/// If TS is a type specifier `A`, where `A` is an atom, then Rep(TS) = `A`.
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Tsl {
    Integer,
    Float,
    Binary,
    Bitstring,
    Bits,
    Utf8,
    Utf16,
    Utf32,
    Signed,
    Unsigned,
    Big,
    Little,
    Native,
    /// If TS is a type specifier `A:Value`, where `A` is an atom and `Value` is an
    /// integer, then Rep(TS) = `{A,Value}`.
    #[rustfmt::skip]
    Unit { size: usize },
}

/* Associations */

/// An association A is one of the following
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MapField<K: Serialize, V: Serialize> {
    /// If A is an association `K => V`, then Rep(A) =
    /// `{map_field_assoc,ANNO,Rep(K),Rep(V)}`.
    MapFieldAssoc(Anno, K, V),
    /// If A is an association `K := V`, then Rep(A) =
    /// `{map_field_exact,ANNO,Rep(K),Rep(V)}`.
    MapFieldExact(Anno, K, V),
}

/// If A is an association `K => V`, then Rep(A) =
/// `{map_field_assoc,ANNO,Rep(K),Rep(V)}`.
/// Moved outside of MapField because it's used individually in many places.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "map_field_assoc")]
pub(crate) struct MapFieldAssoc<K: Serialize, V: Serialize>(pub Anno, pub K, pub V);

/// If A is an association `K := V`, then Rep(A) =
/// `{map_field_exact,ANNO,Rep(K),Rep(V)}`.
/// Moved outside of MapField because it's used individually in many places.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "map_field_exact")]
pub(crate) struct MapFieldExact<K: Serialize, V: Serialize>(pub Anno, pub K, pub V);

/* Clauses

There are function clauses, if clauses, case clauses, and catch clauses.
A clause C is one of the following: */

/// If C is a case clause `P -> B`, where `P` is a pattern and `B` is a body, then
/// Rep(C) = `{clause,ANNO,[Rep(P)],[],Rep(B)}`.
///
/// If C is a case clause `P when Gs -> B`, where `P` is a pattern, `Gs` is a
/// guard sequence, and `B` is a body, then Rep(C) =
/// `{clause,ANNO,[Rep(P)],Rep(Gs),Rep(B)}`.
///
/// If C is a function clause `( Ps ) -> B`, where `Ps` is a pattern sequence and
/// `B` is a body, then Rep(C) = `{clause,ANNO,Rep(Ps),[],Rep(B)}`.
///
/// If C is a function clause `( Ps ) when Gs -> B`, where `Ps` is a pattern
/// sequence, `Gs` is a guard sequence and `B` is a body, then Rep(C) =
/// `{clause,ANNO,Rep(Ps),Rep(Gs),Rep(B)}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "clause")]
pub(crate) struct Clause<'a> {
    pub anno: Anno,
    pub patterns: Vec<Pattern<'a>>,
    pub guards: GuardSeq<'a>,
    pub body: Body<'a>,
}

/// If C is a catch clause `P -> B`, where `P` is a pattern and `B` is a body,
/// then Rep(C) = `{clause,ANNO,[Rep({throw,P,_})],[],Rep(B)}`, that is, a catch
/// clause with an explicit exception class `throw` and with or without an
/// explicit stacktrace variable `_` cannot be distinguished from a catch clause
/// without an explicit exception class and without an explicit stacktrace
/// variable.
///
/// If C is a catch clause `X : P -> B`, where `X` is an atomic literal or a
/// variable pattern, `P` is a pattern, and `B` is a body, then Rep(C) =
/// `{clause,ANNO,[Rep({X,P,_})],[],Rep(B)}`, that is, a catch clause with an
/// explicit exception class and with an explicit stacktrace variable `_` cannot
/// be distinguished from a catch clause with an explicit exception class and
/// without an explicit stacktrace variable.
///
/// If C is a catch clause `X : P : S -> B`, where `X` is an atomic literal or a
/// variable pattern, `P` is a pattern, `S` is a variable, and `B` is a body, then
/// Rep(C) = `{clause,ANNO,[Rep({X,P,S})],[],Rep(B)}`.
///
/// If C is a catch clause `P when Gs -> B`, where `P` is a pattern, `Gs` is a
/// guard sequence, and `B` is a body, then Rep(C) =
/// `{clause,ANNO,[Rep({throw,P,_})],Rep(Gs),Rep(B)}`, that is, a catch clause
/// with an explicit exception class `throw` and with or without an explicit
/// stacktrace variable `_` cannot be distinguished from a catch clause without an
/// explicit exception class and without an explicit stacktrace variable.
///
/// If C is a catch clause `X : P when Gs -> B`, where `X` is an atomic literal or
/// a variable pattern, `P` is a pattern, `Gs` is a guard sequence, and `B` is a
/// body, then Rep(C) = `{clause,ANNO,[Rep({X,P,_})],Rep(Gs),Rep(B)}`, that is, a
/// catch clause with an explicit exception class and with an explicit stacktrace
/// variable `_` cannot be distinguished from a catch clause with an explicit
/// exception class and without an explicit stacktrace variable.
///
/// If C is a catch clause `X : P : S when Gs -> B`, where `X` is an atomic
/// literal or a variable pattern, `P` is a pattern, `Gs` is a guard sequence, `S`
/// is a variable, and `B` is a body, then Rep(C) =
/// `{clause,ANNO,[Rep({X,P,S})],Rep(Gs),Rep(B)}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "clause")]
pub(crate) struct CatchClause<'a> {
    pub anno: Anno,
    pub what: (CatchWhat<'a>,),
    pub guards: GuardSeq<'a>,
    pub body: Body<'a>,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename = "tuple")]
pub(crate) struct CatchWhat<'a>(pub Anno, pub (ExceptionKind<'a>, Pattern<'a>, Var<'a>));

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExceptionKind<'a> {
    AtomicLiteral(AtomicLiteral<'a>),
    Var(Var<'a>),
}

/* Guards */

/// A guard sequence Gs is a sequence of guards `G_1; ...; G_k`, and Rep(Gs) =
/// `[Rep(G_1), ..., Rep(G_k)]`. If the guard sequence is empty, then Rep(Gs) =
/// `[]`.
#[derive(serde::Serialize, Debug)]
pub(crate) struct GuardSeq<'a>(pub Vec<Guard<'a>>);

/// A guard G is a non-empty sequence of guard tests `Gt_1, ..., Gt_k`, and Rep(G) =
/// `[Rep(Gt_1), ..., Rep(Gt_k)]`.
#[derive(serde::Serialize, Debug)]
pub(crate) struct Guard<'a>(pub Vec<GuardTest<'a>>);

/// A guard test Gt is one of the following.
/// Notice that every guard test has the same source form as some expression, and is
/// represented in the same way as the corresponding expression.
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GuardTest<'a> {
    /// If Gt is an atomic literal `L`, then Rep(Gt) = Rep(L).
    AtomicLiteral(AtomicLiteral<'a>),
    /// If Gt is a bitstring constructor
    /// `<<Gt_1:Size_1/TSL_1, ..., Gt_k:Size_k/TSL_k>>`, where each `Size_i` is a
    /// guard test and each `TSL_i` is a type specificer list, then Rep(Gt) =
    /// `{bin,ANNO,[{bin_element,ANNO,Rep(Gt_1),Rep(Size_1),Rep(TSL_1)}, ..., {bin_element,ANNO,Rep(Gt_k),Rep(Size_k),Rep(TSL_k)}]}`.
    /// For Rep(TSL), see above. An omitted `Size_i` is represented by `default`. An
    /// omitted `TSL_i` is represented by `default`.
    Bin(Anno, Vec<BinElement<'a, GuardTest<'a>>>),
    /// If Gt is a cons skeleton `[Gt_h | Gt_t]`, then Rep(Gt) =
    /// `{cons,ANNO,Rep(Gt_h),Rep(Gt_t)}`.
    Cons(Anno, Box<GuardTest<'a>>, Box<GuardTest<'a>>),
    /// If Gt is a function call `A(Gt_1, ..., Gt_k)`, where `A` is an atom, then
    /// Rep(Gt) = `{call,ANNO,Rep(A),[Rep(Gt_1), ..., Rep(Gt_k)]}`.
    #[serde(rename = "call")]
    LocalCall(Anno, Atom<'a>, Vec<GuardTest<'a>>),
    /// If Gt is a function call `A_m:A(Gt_1, ..., Gt_k)`, where `A_m` is the atom
    /// `erlang` and `A` is an atom or an operator, then Rep(Gt) =
    /// `{call,ANNO,{remote,ANNO,Rep(A_m),Rep(A)},[Rep(Gt_1), ..., Rep(Gt_k)]}`.
    #[serde(rename = "call")]
    RemoteCall(Anno, Remote<Atom<'a>>, Vec<GuardTest<'a>>),
    /// If Gt is a map creation `#{A_1, ..., A_k}`, where each `A_i` is an association
    /// `Gt_i_1 => Gt_i_2`, then Rep(Gt) = `{map,ANNO,[Rep(A_1), ..., Rep(A_k)]}`. For
    /// Rep(A), see above.
    #[serde(rename = "map")]
    MapCreate(Anno, Vec<MapFieldAssoc<GuardTest<'a>, GuardTest<'a>>>),
    /// If Gt is a map update `Gt_0#{A_1, ..., A_k}`, where each `A_i` is an
    /// association `Gt_i_1 => Gt_i_2` or `Gt_i_1 := Gt_i_2`, then Rep(Gt) =
    /// `{map,ANNO,Rep(Gt_0),[Rep(A_1), ..., Rep(A_k)]}`. For Rep(A), see above.
    #[serde(rename = "map")]
    MapUpdate(Anno, Box<GuardTest<'a>>, Vec<MapField<GuardTest<'a>, GuardTest<'a>>>),
    /// If Gt is nil, `[]`, then Rep(Gt) = `{nil,ANNO}`.
    #[rustfmt::skip]
    Nil{ anno: Anno },
    /// If Gt is an operator guard test `Gt_1 Op Gt_2`, where `Op` is a binary
    /// operator other than match operator `=`, then Rep(Gt) =
    /// `{op,ANNO,Op,Rep(Gt_1),Rep(Gt_2)}`.
    #[serde(rename = "op")]
    BinaryOp(Anno, &'a str, Box<GuardTest<'a>>, Box<GuardTest<'a>>),
    /// If Gt is an operator guard test `Op Gt_0`, where `Op` is a unary operator,
    /// then Rep(Gt) = `{op,ANNO,Op,Rep(Gt_0)}`.
    #[serde(rename = "op")]
    UnaryOp(Anno, &'a str, Box<GuardTest<'a>>),
    /// If Gt is a parenthesized guard test `( Gt_0 )`, then Rep(Gt) = `Rep(Gt_0)`,
    /// that is, parenthesized guard tests cannot be distinguished from their bodies.
    Parenthesized(Box<GuardTest<'a>>),
    /// If Gt is a record creation `#Name{Field_1=Gt_1, ..., Field_k=Gt_k}`, where
    /// each `Field_i` is an atom or `_`, then Rep(Gt) =
    /// `{record,ANNO,Name,[{record_field,ANNO,Rep(Field_1),Rep(Gt_1)}, ..., {record_field,ANNO,Rep(Field_k),Rep(Gt_k)}]}`.
    Record(Anno, &'a str, Vec<RecordField<AtomOrVar<'a>, GuardTest<'a>>>),
    /// If Gt is a record field access `Gt_0#Name.Field`, where `Field` is an atom,
    /// then Rep(Gt) = `{record_field,ANNO,Rep(Gt_0),Name,Rep(Field)}`.
    RecordField(Anno, Box<GuardTest<'a>>, &'a str, Atom<'a>),
    /// If Gt is a record field index `#Name.Field`, where `Field` is an atom, then
    /// Rep(Gt) = `{record_index,ANNO,Name,Rep(Field)}`.
    RecordIndex(Anno, &'a str, Atom<'a>),
    /// If Gt is a tuple skeleton `{Gt_1, ..., Gt_k}`, then Rep(Gt) =
    /// `{tuple,ANNO,[Rep(Gt_1), ..., Rep(Gt_k)]}`.
    Tuple(Anno, Vec<GuardTest<'a>>),
    /// If Gt is a variable pattern `V`, then Rep(Gt) = `{var,ANNO,A}`, where A is an
    /// atom with a printname consisting of the same characters as `V`.
    Var(Anno, &'a str),
}

/* Types */

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Type<'a> {
    /// If T is an annotated type `A :: T_0`, where `A` is a variable, then Rep(T) =
    /// `{ann_type,ANNO,[Rep(A),Rep(T_0)]}`.
    AnnType(Anno, (Var<'a>, Box<Type<'a>>)),
    /// If T is an atom, a character, or an integer literal L, then Rep(T) = Rep(L).
    AtomicLiteral(AtomicLiteral<'a>),
    /// If T is a bitstring type `<<_:M,_:_*N>>`, where `M` and `N` are singleton
    /// integer types, then Rep(T) = `{type,ANNO,binary,[Rep(M),Rep(N)]}`.
    #[serde(rename = "__type__binary")]
    Binary(Anno, (Integer, Integer)),
    /// If T is the empty list type `[]`, then Rep(T) = `{type,ANNO,nil,[]}`, that is,
    /// the empty list type `[]` cannot be distinguished from the predefined type
    /// `t:nil/0`.
    #[serde(rename = "__type__nil")]
    Nil(Anno, ()),
    /// If T is a fun type `fun()`, then Rep(T) = `{type,ANNO,'fun',[]}`.
    #[serde(rename = "__type__fun")]
    Fun(Anno, ()),
    /// If T is a fun type `fun((...) -> T_0)`, then Rep(T) =
    /// `{type,ANNO,'fun',[{type,ANNO,any},Rep(T_0)]}`.
    #[serde(rename = "__type__fun")]
    AnyFun(Anno, (AnyType, Box<Type<'a>>)),
    /// If T is a fun type `fun(Ft)`, where `Ft` is a function type, then Rep(T) =
    /// `Rep(Ft)`. For Rep(Ft), see below.
    FunctionType(FunctionType<'a>),
    /// If T is an integer range type `L .. H`, where `L` and `H` are singleton
    /// integer types, then Rep(T) = `{type,ANNO,range,[Rep(L),Rep(H)]}`.
    #[serde(rename = "__type__range")]
    Range(Anno, (Box<Type<'a>>, Box<Type<'a>>)),
    /// If T is a map type `t:map/0`, then Rep(T) = `{type,ANNO,map,any}`.
    ///
    /// If T is a map type `#{A_1, ..., A_k}`, where each `A_i` is an association
    /// type, then Rep(T) = `{type,ANNO,map,[Rep(A_1), ..., Rep(A_k)]}`. For Rep(A),
    /// see below.
    #[serde(rename = "__type__map")]
    Map(Anno, MapTypeAssocs<'a>),
    /// If T is an operator type `T_1 Op T_2`, where `Op` is a binary operator (this
    /// is an occurrence of an expression that can be evaluated to an integer at
    /// compile time), then Rep(T) = `{op,ANNO,Op,Rep(T_1),Rep(T_2)}`.
    #[serde(rename = "op")]
    BinaryOp(Anno, &'a str, Expr<'a>, Expr<'a>),
    /// If T is an operator type `Op T_0`, where `Op` is a unary operator (this is an
    /// occurrence of an expression that can be evaluated to an integer at compile
    /// time), then Rep(T) = `{op,ANNO,Op,Rep(T_0)}`.
    #[serde(rename = "op")]
    UnaryOp(Anno, &'a str, Expr<'a>),
    /// If T is `( T_0 )`, then Rep(T) = `Rep(T_0)`, that is, parenthesized types
    /// cannot be distinguished from their bodies.
    Parenthesized(Box<Type<'a>>),
    /// If T is a predefined (or built-in) type `N(T_1, ..., T_k)`, then Rep(T) =
    /// `{type,ANNO,N,[Rep(T_1), ..., Rep(T_k)]}`.
    #[serde(rename = "type")]
    Predefined(Anno, &'a str, Vec<Type<'a>>),
    /// If T is a record type `#Name{F_1, ..., F_k}`, where each `F_i` is a record
    /// field type, then Rep(T) =
    /// `{type,ANNO,record,[Rep(Name),Rep(F_1), ..., Rep(F_k)]}`. For Rep(F), see
    /// below.
    #[serde(rename = "__type__record")]
    Record(Anno, RecordTypeFields<'a>),
    /// If T is a remote type `M:N(T_1, ..., T_k)`, then Rep(T) =
    /// `{remote_type,ANNO,[Rep(M),Rep(N),[Rep(T_1), ..., Rep(T_k)]]}`.
    RemoteType(Anno, (Atom<'a>, Atom<'a>, Vec<Type<'a>>)),
    /// If T is a tuple type `t:tuple/0`, then Rep(T) = `{type,ANNO,tuple,any}`.
    ///
    /// If T is a tuple type `{T_1, ..., T_k}`, then Rep(T) =
    /// `{type,ANNO,tuple,[Rep(T_1), ..., Rep(T_k)]}`.
    #[serde(rename = "__type__tuple")]
    Tuple(Anno, TupleTypeElem<'a>),
    /// If T is a type union `T_1 | ... | T_k`, then Rep(T) =
    /// `{type,ANNO,union,[Rep(T_1), ..., Rep(T_k)]}`.
    #[serde(rename = "__type__union")]
    Union(Anno, Vec<Type<'a>>),
    /// If T is a type variable `V`, then Rep(T) = `{var,ANNO,A}`, where `A` is an
    /// atom with a printname consisting of the same characters as `V`. A type
    /// variable is any variable except underscore (`_`).
    Var(Anno, &'a str),
    /// If T is a user-defined type `N(T_1, ..., T_k)`, then Rep(T) =
    /// `{user_type,ANNO,N,[Rep(T_1), ..., Rep(T_k)]}`.
    UserType(Anno, &'a str, Vec<Type<'a>>),
}

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__type__any")]
pub(crate) struct AnyType {
    pub anno: Anno,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MapTypeAssocs<'a> {
    Any,
    Assocs(Vec<AssociationType<'a>>),
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TupleTypeElem<'a> {
    Any,
    Assocs(Vec<Type<'a>>),
}

#[derive(Debug)]
pub(crate) struct RecordTypeFields<'a>(pub Atom<'a>, pub Vec<RecordFieldType<'a>>);

impl<'a> Serialize for RecordTypeFields<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut s = serializer.serialize_seq(Some(1 + self.1.len()))?;
        s.serialize_element(&self.0)?;
        for field in &self.1 {
            s.serialize_element(field)?;
        }
        s.end()
    }
}

/* Function Types */

/// A function type Ft is one of the following:
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FunctionType<'a> {
    /// If Ft is a constrained function type `Ft_1 when Fc`, where `Ft_1` is a
    /// function type and `Fc` is a function constraint, then Rep(T) =
    /// `{type,ANNO,bounded_fun,[Rep(Ft_1),Rep(Fc)]}`. For Rep(Fc), see below.
    #[serde(rename = "__type__bounded_fun")]
    BoundedFun(Anno, (Box<FunctionType<'a>>, FunctionConstraint<'a>)),
    /// If Ft is a function type `(T_1, ..., T_n) -> T_0`, where each `T_i` is a type,
    /// then Rep(Ft) =
    /// `{type,ANNO,'fun',[{type,ANNO,product,[Rep(T_1), ..., Rep(T_n)]},Rep(T_0)]}`.
    #[serde(rename = "__type__fun")]
    Fun(Anno, (ProductType<'a>, Box<Type<'a>>)),
}

#[derive(serde::Serialize, Debug)]
#[serde(rename = "__type__product")]
pub(crate) struct ProductType<'a>(pub Anno, pub Vec<Type<'a>>);

/* Function Constraints */

/// A function constraint Fc is a non-empty sequence of constraints `C_1, ..., C_k`,
/// and Rep(Fc) = `[Rep(C_1), ..., Rep(C_k)]`.
#[derive(serde::Serialize, Debug)]
pub(crate) struct FunctionConstraint<'a>(pub Vec<Constraint<'a>>);

/// If C is a constraint `V :: T`, where `V` is a type variable and `T` is a type,
/// then Rep(C) =
/// `{type,ANNO,constraint,[{atom,ANNO,is_subtype},[Rep(V),Rep(T)]]}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "__type__constraint")]
pub(crate) struct Constraint<'a>(pub Anno, pub (IsSubtype, (Var<'a>, Type<'a>)));

#[derive(serde::Serialize, Debug)]
#[serde(rename = "atom")]
pub(crate) struct IsSubtype(pub Anno, pub IsSubtypeMark);

#[derive(serde::Serialize, Debug)]
#[serde(rename = "is_subtype")]
pub(crate) struct IsSubtypeMark;

/* Association Types */

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AssociationType<'a> {
    /// If A is an association type `K => V`, where `K` and `V` are types, then Rep(A)
    /// = `{type,ANNO,map_field_assoc,[Rep(K),Rep(V)]}`.
    #[serde(rename = "__type__map_field_assoc")]
    MapFieldAssoc(Anno, (Type<'a>, Type<'a>)),
    /// If A is an association type `K := V`, where `K` and `V` are types, then Rep(A)
    /// = `{type,ANNO,map_field_exact,[Rep(K),Rep(V)]}`.
    #[serde(rename = "__type__map_field_exact")]
    MapFieldExact(Anno, (Type<'a>, Type<'a>)),
}

/* Record Field Types */

/// If F is a record field type `Name :: Type`, where `Type` is a type, then
/// Rep(F) = `{type,ANNO,field_type,[Rep(Name),Rep(Type)]}`.
#[derive(serde::Serialize, Debug)]
#[serde(rename = "__type__field_type")]
pub(crate) struct RecordFieldType<'a>(pub Anno, pub (Atom<'a>, Type<'a>));

/* Helpers */

#[derive(serde::Serialize, Debug)]
#[serde(rename = "var")]
pub(crate) struct Var<'a>(pub Anno, pub &'a str);

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AtomOrVar<'a> {
    Atom(Anno, &'a str),
    Var(Anno, &'a str),
}

pub(crate) fn serialize_biguint<S: Serializer>(
    v: &BigUint,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use num_traits::ToPrimitive;

    if let Some(v) = v.to_u128() {
        serializer.serialize_u128(v)
    } else {
        const PREFIX: &[u8] = b"BigInt";

        let sign = 0u8;
        let mag = v.to_bytes_le();
        let mut payload = Vec::with_capacity(PREFIX.len() + 1 + mag.len());

        payload.extend_from_slice(PREFIX);
        payload.push(sign);
        payload.extend_from_slice(&mag);

        serializer.serialize_bytes(&payload)
    }
}

#[cfg(test)]
mod tests;
