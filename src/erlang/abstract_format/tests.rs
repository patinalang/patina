use super::*;
use crate::erlang::etf_serde::to_bytes;

const TERM_HEADER: u8 = 131;
const SMALL_ATOM_UTF8_EXT: u8 = 119;
const SMALL_INTEGER_EXT: u8 = 97;
const SMALL_BIG_EXT: u8 = 110;
const NEW_FLOAT_EXT: u8 = 70;
const SMALL_TUPLE_EXT: u8 = 104;
const LIST_EXT: u8 = 108;
const NIL_EXT: u8 = 106;

fn anno() -> Anno {
    Anno { line: 1, column: 2 }
}

/// ANNO is an elided 2-tuple `{Line, Column}` in the on-wire form.
fn anno_bytes() -> Vec<u8> {
    vec![SMALL_TUPLE_EXT, 2, SMALL_INTEGER_EXT, 1, SMALL_INTEGER_EXT, 2]
}

fn atom(s: &str) -> Vec<u8> {
    let mut v = vec![SMALL_ATOM_UTF8_EXT, s.len() as u8];
    v.extend_from_slice(s.as_bytes());
    v
}

fn small_int(n: u8) -> Vec<u8> {
    vec![SMALL_INTEGER_EXT, n]
}

fn file_line(s: &str, line: usize) -> FileLine {
    FileLine(s.chars().collect(), line)
}

fn char_list(s: &str) -> Vec<u8> {
    list(&s.chars().map(|c| small_int(c as u8)).collect::<Vec<_>>())
}

fn tuple(items: &[Vec<u8>]) -> Vec<u8> {
    let mut v = vec![SMALL_TUPLE_EXT, items.len() as u8];
    for item in items {
        v.extend_from_slice(item);
    }
    v
}

fn list(items: &[Vec<u8>]) -> Vec<u8> {
    if items.is_empty() {
        return vec![NIL_EXT];
    }
    let mut v = vec![LIST_EXT];
    v.extend_from_slice(&(items.len() as u32).to_be_bytes());
    for item in items {
        v.extend_from_slice(item);
    }
    v.push(NIL_EXT);
    v
}

fn nil() -> Vec<u8> {
    vec![NIL_EXT]
}

fn term(payload: Vec<u8>) -> Vec<u8> {
    let mut v = vec![TERM_HEADER];
    v.extend(payload);
    v
}

/// `{integer, ANNO, n}` shortcut.
fn integer_lit(n: u8) -> Vec<u8> {
    tuple(&[atom("integer"), anno_bytes(), small_int(n)])
}

/// `{atom, ANNO, s}` shortcut.
fn atom_lit(s: &str) -> Vec<u8> {
    tuple(&[atom("atom"), anno_bytes(), atom(s)])
}

/// `{var, ANNO, name}` shortcut.
fn var_lit(name: &str) -> Vec<u8> {
    tuple(&[atom("var"), anno_bytes(), atom(name)])
}

/// `{type, ANNO, name, [args]}` shortcut for predefined builtin types.
fn predefined_type(name: &str, args: &[Vec<u8>]) -> Vec<u8> {
    tuple(&[atom("type"), anno_bytes(), atom(name), list(args)])
}

fn integer_struct(n: u8) -> Integer {
    Integer(anno(), BigUint::from(n))
}

fn integer_struct_bytes(n: u8) -> Vec<u8> {
    // Same wire form as integer_lit — the standalone Integer struct is named "integer".
    integer_lit(n)
}

/* Anno */

#[test]
fn anno_elides_struct_name() {
    assert_eq!(to_bytes(&anno()).unwrap(), term(anno_bytes()));
}

/* Atomic Literals */

#[test]
fn atomic_literal_atom() {
    // Rep(L) = {atom, ANNO, L}
    let v = AtomicLiteral::Atom(anno(), "foo");
    let expected = term(tuple(&[atom("atom"), anno_bytes(), atom("foo")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn atomic_literal_char() {
    // Rep(L) = {char, ANNO, L}
    let v = AtomicLiteral::Char(anno(), 'a');
    let expected = term(tuple(&[atom("char"), anno_bytes(), small_int(b'a')]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn atomic_literal_float() {
    // Rep(L) = {float, ANNO, L}
    let v = AtomicLiteral::Float(anno(), 1.5);
    let mut float_b = vec![NEW_FLOAT_EXT];
    float_b.extend_from_slice(&1.5f64.to_be_bytes());
    let expected = term(tuple(&[atom("float"), anno_bytes(), float_b]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn atomic_literal_integer_small() {
    // Rep(L) = {integer, ANNO, L}
    let v = AtomicLiteral::Integer(anno(), BigUint::from(42u32));
    let expected = term(tuple(&[atom("integer"), anno_bytes(), small_int(42)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn atomic_literal_integer_above_u128() {
    // 2^128 forces the SMALL_BIG_EXT path inside the integer literal.
    let v = AtomicLiteral::Integer(anno(), BigUint::from(1u32) << 128);
    let mut bigint_b = vec![SMALL_BIG_EXT, 17, 0];
    bigint_b.extend(std::iter::repeat(0u8).take(16));
    bigint_b.push(1);
    let expected = term(tuple(&[atom("integer"), anno_bytes(), bigint_b]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Patterns */

#[test]
fn pattern_nil() {
    // Rep(P) = {nil, ANNO}
    let v = Pattern::Nil { anno: anno() };
    let expected = term(tuple(&[atom("nil"), anno_bytes()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_var() {
    // Rep(P) = {var, ANNO, A}
    let v = Pattern::Var(anno(), "X");
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}

#[test]
fn pattern_atomic_literal_is_transparent() {
    // "Rep(P) = Rep(L)" — the AtomicLiteral wrapper must not add a tag.
    let v = Pattern::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(7u32)));
    assert_eq!(to_bytes(&v).unwrap(), term(integer_lit(7)));
}

#[test]
fn pattern_cons() {
    // Rep(P) = {cons, ANNO, Rep(P_h), Rep(P_t)}
    let v = Pattern::Cons(
        anno(),
        Box::new(Pattern::Var(anno(), "H")),
        Box::new(Pattern::Nil { anno: anno() }),
    );
    let expected = term(tuple(&[
        atom("cons"),
        anno_bytes(),
        var_lit("H"),
        tuple(&[atom("nil"), anno_bytes()]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_tuple() {
    // Rep(P) = {tuple, ANNO, [Rep(P_1), ..., Rep(P_k)]}
    let v = Pattern::Tuple(anno(), vec![Pattern::Var(anno(), "X"), Pattern::Var(anno(), "Y")]);
    let expected = term(tuple(&[atom("tuple"), anno_bytes(), list(&[var_lit("X"), var_lit("Y")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_binary_op() {
    // Rep(P) = {op, ANNO, Op, Rep(P_1), Rep(P_2)}
    let v = Pattern::BinaryOp(
        anno(),
        "++",
        Box::new(Pattern::Var(anno(), "X")),
        Box::new(Pattern::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
    );
    let expected =
        term(tuple(&[atom("op"), anno_bytes(), atom("++"), var_lit("X"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_unary_op() {
    // Rep(P) = {op, ANNO, Op, Rep(P_0)}
    let v = Pattern::UnaryOp(
        anno(),
        "-",
        Box::new(Pattern::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
    );
    let expected = term(tuple(&[atom("op"), anno_bytes(), atom("-"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_match() {
    // Rep(P) = {match, ANNO, Rep(P_1), Rep(P_2)}
    let v = Pattern::Match(
        anno(),
        Box::new(Pattern::Var(anno(), "X")),
        Box::new(Pattern::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
    );
    let expected = term(tuple(&[atom("match"), anno_bytes(), var_lit("X"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_parenthesized_is_transparent() {
    // "parenthesized patterns cannot be distinguished from their bodies"
    let v = Pattern::Parenthesized(Box::new(Pattern::Var(anno(), "X")));
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}

/* Expressions */

#[test]
fn expr_nil() {
    // Rep(E) = {nil, ANNO}
    let v = Expr::Nil { anno: anno() };
    let expected = term(tuple(&[atom("nil"), anno_bytes()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_atomic_literal_is_transparent() {
    let v = Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "ok"));
    assert_eq!(to_bytes(&v).unwrap(), term(atom_lit("ok")));
}

#[test]
fn expr_block() {
    // Rep(E) = {block, ANNO, Rep(B)} where Rep(B) is a list of exprs
    let body = Body(vec![
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(2u32))),
    ]);
    let v = Expr::Block(anno(), body);
    let expected =
        term(tuple(&[atom("block"), anno_bytes(), list(&[integer_lit(1), integer_lit(2)])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_catch() {
    // Rep(E) = {'catch', ANNO, Rep(E_0)}
    let v = Expr::Catch(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
    );
    let expected = term(tuple(&[atom("catch"), anno_bytes(), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_local_call_with_args() {
    // Rep(E) = {call, ANNO, Rep(E_0), [Rep(E_1), ..., Rep(E_k)]}
    let v = Expr::LocalCall(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "f"))),
        vec![Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))],
    );
    let expected =
        term(tuple(&[atom("call"), anno_bytes(), atom_lit("f"), list(&[integer_lit(1)])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_local_call_no_args_becomes_nil_list() {
    // An empty argument list serializes as a bare NIL_EXT (the Erlang `[]`).
    let v = Expr::LocalCall(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "now"))),
        vec![],
    );
    let expected = term(tuple(&[atom("call"), anno_bytes(), atom_lit("now"), vec![NIL_EXT]]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_match() {
    // Rep(E) = {match, ANNO, Rep(P), Rep(E_0)}
    let v = Expr::Match(
        anno(),
        Pattern::Var(anno(), "X"),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(42u32)))),
    );
    let expected = term(tuple(&[atom("match"), anno_bytes(), var_lit("X"), integer_lit(42)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_maybe_match() {
    // Rep(E) = {maybe_match, ANNO, Rep(P), Rep(E_0)}
    let v = Expr::MaybeMatch(
        anno(),
        Pattern::Var(anno(), "X"),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(42u32)))),
    );
    let expected = term(tuple(&[atom("maybe_match"), anno_bytes(), var_lit("X"), integer_lit(42)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_tuple() {
    // Rep(E) = {tuple, ANNO, [Rep(E_1), ..., Rep(E_k)]}
    let v = Expr::Tuple(
        anno(),
        vec![
            Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
            Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(2u32))),
        ],
    );
    let expected =
        term(tuple(&[atom("tuple"), anno_bytes(), list(&[integer_lit(1), integer_lit(2)])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_cons() {
    // Rep(E) = {cons, ANNO, Rep(E_h), Rep(E_t)}
    let v = Expr::Cons(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
        Box::new(Expr::Nil { anno: anno() }),
    );
    let expected = term(tuple(&[
        atom("cons"),
        anno_bytes(),
        integer_lit(1),
        tuple(&[atom("nil"), anno_bytes()]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_binary_op() {
    // Rep(E) = {op, ANNO, Op, Rep(E_1), Rep(E_2)}
    let v = Expr::BinaryOp(
        anno(),
        "+",
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(2u32)))),
    );
    let expected =
        term(tuple(&[atom("op"), anno_bytes(), atom("+"), integer_lit(1), integer_lit(2)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_unary_op() {
    // Rep(E) = {op, ANNO, Op, Rep(E_0)}
    let v = Expr::UnaryOp(
        anno(),
        "-",
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
    );
    let expected = term(tuple(&[atom("op"), anno_bytes(), atom("-"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_if_no_clauses() {
    // Rep(E) = {'if', ANNO, [Rep(Ic_1), ..., Rep(Ic_k)]}
    let v = Expr::If(anno(), vec![]);
    let expected = term(tuple(&[atom("if"), anno_bytes(), vec![NIL_EXT]]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_receive_no_clauses() {
    // Rep(E) = {'receive', ANNO, [Rep(Cc_1), ..., Rep(Cc_k)]}
    let v = Expr::Receive(anno(), vec![]);
    let expected = term(tuple(&[atom("receive"), anno_bytes(), vec![NIL_EXT]]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_parenthesized_is_transparent() {
    let v = Expr::Parenthesized(Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "foo"))));
    assert_eq!(to_bytes(&v).unwrap(), term(atom_lit("foo")));
}

#[test]
fn expr_maybe() {
    // Rep(E) = {'maybe', ANNO, Rep(B)}
    let body = Body(vec![Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))]);
    let v = Expr::Maybe(anno(), body);
    let expected = term(tuple(&[atom("maybe"), anno_bytes(), list(&[integer_lit(1)])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_case_no_clauses() {
    // Rep(E) = {'case', ANNO, Rep(E_0), [Rep(Cc_1), ..., Rep(Cc_k)]}
    let v =
        Expr::Case(anno(), Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "x"))), vec![]);
    let expected = term(tuple(&[atom("case"), anno_bytes(), atom_lit("x"), vec![NIL_EXT]]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_named_fun_no_clauses() {
    // Rep(E) = {named_fun, ANNO, Name, [Rep(Fc_1), ..., Rep(Fc_k)]}
    let v = Expr::NamedFun(anno(), "Loop", vec![]);
    let expected = term(tuple(&[atom("named_fun"), anno_bytes(), atom("Loop"), vec![NIL_EXT]]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Bitstring Type Specifiers */

#[test]
fn tsl_unit_atoms() {
    // Atom-only specifiers serialize as bare atoms.
    assert_eq!(to_bytes(&Tsl::Integer).unwrap(), term(atom("integer")));
    assert_eq!(to_bytes(&Tsl::Float).unwrap(), term(atom("float")));
    assert_eq!(to_bytes(&Tsl::Binary).unwrap(), term(atom("binary")));
    assert_eq!(to_bytes(&Tsl::Signed).unwrap(), term(atom("signed")));
    assert_eq!(to_bytes(&Tsl::Unsigned).unwrap(), term(atom("unsigned")));
    assert_eq!(to_bytes(&Tsl::Big).unwrap(), term(atom("big")));
    assert_eq!(to_bytes(&Tsl::Little).unwrap(), term(atom("little")));
    assert_eq!(to_bytes(&Tsl::Native).unwrap(), term(atom("native")));
}

#[test]
fn tsl_unit_with_size() {
    // {A, Value} — here A = unit
    let v = Tsl::Unit { size: 8 };
    let expected = term(tuple(&[atom("unit"), small_int(8)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn bin_elem_size_default_is_atom() {
    assert_eq!(to_bytes(&BinElemSize::Default).unwrap(), term(atom("default")));
}

#[test]
fn bin_elem_tsl_default_is_atom() {
    assert_eq!(to_bytes(&BinElemTsl::Default).unwrap(), term(atom("default")));
}

/* Qualifiers / Generators */

#[test]
fn generator_generate() {
    // Rep(Q) = {generate, ANNO, Rep(P), Rep(E)}
    let v = Generator::Generate(
        anno(),
        Pattern::Var(anno(), "X"),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "list")),
    );
    let expected = term(tuple(&[atom("generate"), anno_bytes(), var_lit("X"), atom_lit("list")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn generator_generate_strict() {
    // Rep(Q) = {generate_strict, ANNO, Rep(P), Rep(E)}
    let v = Generator::GenerateStrict(
        anno(),
        Pattern::Var(anno(), "X"),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "list")),
    );
    let expected =
        term(tuple(&[atom("generate_strict"), anno_bytes(), var_lit("X"), atom_lit("list")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn generator_b_generate() {
    // Rep(Q) = {b_generate, ANNO, Rep(P), Rep(E)}
    let v = Generator::BGenerate(
        anno(),
        Pattern::Var(anno(), "X"),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "bin")),
    );
    let expected = term(tuple(&[atom("b_generate"), anno_bytes(), var_lit("X"), atom_lit("bin")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn generator_m_generate() {
    // Rep(Q) = {m_generate, ANNO, Rep(P), Rep(E)}
    let v = Generator::MGenerate(
        anno(),
        Pattern::Var(anno(), "X"),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "map")),
    );
    let expected = term(tuple(&[atom("m_generate"), anno_bytes(), var_lit("X"), atom_lit("map")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* MapField (enum form, used by MapUpdate / guard MapUpdate) */

#[test]
fn map_field_assoc_variant() {
    // {map_field_assoc, ANNO, Rep(K), Rep(V)}
    let v: MapField<Expr<'_>, Expr<'_>> = MapField::MapFieldAssoc(
        anno(),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
    );
    let expected =
        term(tuple(&[atom("map_field_assoc"), anno_bytes(), atom_lit("k"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn map_field_exact_variant() {
    // {map_field_exact, ANNO, Rep(K), Rep(V)}
    let v: MapField<Expr<'_>, Expr<'_>> = MapField::MapFieldExact(
        anno(),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
    );
    let expected =
        term(tuple(&[atom("map_field_exact"), anno_bytes(), atom_lit("k"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* ExceptionKind and AtomOrVar */

#[test]
fn exception_kind_atom() {
    // {atom, ANNO, throw}
    let v = ExceptionKind::AtomicLiteral(AtomicLiteral::Atom(anno(), "throw"));
    let expected = term(tuple(&[atom("atom"), anno_bytes(), atom("throw")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn exception_kind_var() {
    // {var, ANNO, X}
    let v = ExceptionKind::Var(Var(anno(), "X"));
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}

#[test]
fn atom_or_var_atom() {
    let v = AtomOrVar::Atom(anno(), "name");
    let expected = term(tuple(&[atom("atom"), anno_bytes(), atom("name")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn atom_or_var_var() {
    let v = AtomOrVar::Var(anno(), "X");
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}

/* Guard tests */

#[test]
fn guard_test_atomic_literal_is_transparent() {
    let v = GuardTest::AtomicLiteral(AtomicLiteral::Atom(anno(), "true"));
    assert_eq!(to_bytes(&v).unwrap(), term(atom_lit("true")));
}

#[test]
fn guard_test_var() {
    let v = GuardTest::Var(anno(), "X");
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}

#[test]
fn guard_test_nil() {
    let v = GuardTest::Nil { anno: anno() };
    let expected = term(tuple(&[atom("nil"), anno_bytes()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_binary_op() {
    // Rep(Gt) = {op, ANNO, Op, Rep(Gt_1), Rep(Gt_2)}
    let v = GuardTest::BinaryOp(
        anno(),
        ">=",
        Box::new(GuardTest::Var(anno(), "X")),
        Box::new(GuardTest::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(0u32)))),
    );
    let expected =
        term(tuple(&[atom("op"), anno_bytes(), atom(">="), var_lit("X"), integer_lit(0)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_unary_op() {
    // Rep(Gt) = {op, ANNO, Op, Rep(Gt_0)}
    let v = GuardTest::UnaryOp(anno(), "not", Box::new(GuardTest::Var(anno(), "X")));
    let expected = term(tuple(&[atom("op"), anno_bytes(), atom("not"), var_lit("X")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_parenthesized_is_transparent() {
    let v = GuardTest::Parenthesized(Box::new(GuardTest::Var(anno(), "X")));
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}

/* AtomicLiteral::String — list of character codes */

#[test]
fn atomic_literal_string() {
    // Rep(L) = {string, ANNO, [C_1, ..., C_k]}
    let v = AtomicLiteral::String(anno(), vec!['a', 'b', 'c']);
    let expected = term(tuple(&[
        atom("string"),
        anno_bytes(),
        list(&[small_int(b'a'), small_int(b'b'), small_int(b'c')]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn atomic_literal_empty_string_is_nil() {
    // An empty string encodes as the bare `[]`.
    let v = AtomicLiteral::String(anno(), vec![]);
    let expected = term(tuple(&[atom("string"), anno_bytes(), nil()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Standalone Atom / Integer / Var structs */

#[test]
fn atom_struct_wire_form() {
    // Doc: {atom, ANNO, L}
    let v = Atom(anno(), "foo");
    let expected = term(tuple(&[atom("atom"), anno_bytes(), atom("foo")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn integer_struct_wire_form() {
    // Doc: {integer, ANNO, L}
    let v = Integer(anno(), BigUint::from(7u32));
    let expected = term(tuple(&[atom("integer"), anno_bytes(), small_int(7)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn var_struct_wire_form() {
    // Doc: {var, ANNO, A}
    let v = Var(anno(), "X");
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}

/* Body / GuardSeq / Guard are transparent newtype Vec wrappers */

#[test]
fn body_serializes_as_list_of_exprs() {
    // Rep(B) = [Rep(E_1), ..., Rep(E_k)]
    let v = Body(vec![
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "a")),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "b")),
    ]);
    let expected = term(list(&[atom_lit("a"), atom_lit("b")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_seq_empty_is_nil_list() {
    // "If the guard sequence is empty, then Rep(Gs) = []."
    let v = GuardSeq(vec![]);
    assert_eq!(to_bytes(&v).unwrap(), term(nil()));
}

#[test]
fn guard_seq_serializes_as_list_of_guards() {
    // Rep(Gs) = [Rep(G_1), ..., Rep(G_k)]
    let v = GuardSeq(vec![Guard(vec![GuardTest::Var(anno(), "X")])]);
    let expected = term(list(&[list(&[var_lit("X")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_serializes_as_list_of_guard_tests() {
    // Rep(G) = [Rep(Gt_1), ..., Rep(Gt_k)]
    let v = Guard(vec![GuardTest::Var(anno(), "X"), GuardTest::Var(anno(), "Y")]);
    let expected = term(list(&[var_lit("X"), var_lit("Y")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Pattern variants that were previously skipped */

#[test]
fn pattern_bin() {
    // Rep(P) = {bin, ANNO, [{bin_element, ANNO, Rep(P_i), Rep(Size_i), Rep(TSL_i)}, ...]}
    let elem = BinElement::<Pattern> {
        anno: anno(),
        value: Pattern::Var(anno(), "X"),
        size: BinElemSize::Default,
        tsl: BinElemTsl::Default,
    };
    let v = Pattern::Bin(anno(), vec![elem]);
    let expected = term(tuple(&[
        atom("bin"),
        anno_bytes(),
        list(&[tuple(&[
            atom("bin_element"),
            anno_bytes(),
            var_lit("X"),
            atom("default"),
            atom("default"),
        ])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_map() {
    // Rep(P) = {map, ANNO, [Rep(A_1), ..., Rep(A_k)]}
    // Each A_i is `P_i_1 := P_i_2`, which is a map_field_exact association.
    let v = Pattern::Map(
        anno(),
        vec![MapFieldExact(
            anno(),
            Pattern::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
            Pattern::Var(anno(), "V"),
        )],
    );
    let expected = term(tuple(&[
        atom("map"),
        anno_bytes(),
        list(&[tuple(&[atom("map_field_exact"), anno_bytes(), atom_lit("k"), var_lit("V")])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_record_index() {
    // Rep(P) = {record_index, ANNO, Name, Rep(Field)}
    let v = Pattern::RecordIndex(anno(), "rec", Atom(anno(), "f"));
    let expected = term(tuple(&[
        atom("record_index"),
        anno_bytes(),
        atom("rec"),
        tuple(&[atom("atom"), anno_bytes(), atom("f")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn pattern_record() {
    // Rep(P) = {record, ANNO, Name, [{record_field, ANNO, Rep(Field), Rep(P_i)}, ...]}
    let v = Pattern::Record(
        anno(),
        "rec",
        vec![RecordField(anno(), AtomOrVar::Atom(anno(), "f"), Some(Pattern::Var(anno(), "V")))],
    );
    let expected = term(tuple(&[
        atom("record"),
        anno_bytes(),
        atom("rec"),
        list(&[tuple(&[atom("record_field"), anno_bytes(), atom_lit("f"), var_lit("V")])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Expr variants that were previously skipped */

#[test]
fn expr_bin() {
    // Rep(E) = {bin, ANNO, [{bin_element, ANNO, Rep(E_i), Rep(Size_i), Rep(TSL_i)}, ...]}
    let elem = BinElement::<Expr> {
        anno: anno(),
        value: Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(7u32))),
        size: BinElemSize::Size(Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(
            anno(),
            BigUint::from(8u32),
        )))),
        tsl: BinElemTsl::Tsl(vec![Tsl::Integer]),
    };
    let v = Expr::Bin(anno(), vec![elem]);
    let expected = term(tuple(&[
        atom("bin"),
        anno_bytes(),
        list(&[tuple(&[
            atom("bin_element"),
            anno_bytes(),
            integer_lit(7),
            integer_lit(8),
            list(&[atom("integer")]),
        ])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_bc() {
    // Rep(E) = {bc, ANNO, Rep(E_0), [Rep(Q_1), ..., Rep(Q_k)]}
    let v = Expr::Bc(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "x"))),
        vec![Qualifier::Filter(Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "true"))))],
    );
    let expected =
        term(tuple(&[atom("bc"), anno_bytes(), atom_lit("x"), list(&[atom_lit("true")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_lc() {
    // Rep(E) = {lc, ANNO, Rep(E_0), [Rep(Q_1), ..., Rep(Q_k)]}
    let v = Expr::Lc(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "x"))),
        vec![Qualifier::Filter(Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "true"))))],
    );
    let expected =
        term(tuple(&[atom("lc"), anno_bytes(), atom_lit("x"), list(&[atom_lit("true")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_mc() {
    // Rep(E) = {mc, ANNO, Rep(E_0), [Rep(Q_1), ..., Rep(Q_k)]}
    let v = Expr::Mc(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "x"))),
        vec![Qualifier::Filter(Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "true"))))],
    );
    let expected =
        term(tuple(&[atom("mc"), anno_bytes(), atom_lit("x"), list(&[atom_lit("true")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_map_create() {
    // Rep(E) = {map, ANNO, [Rep(A_1), ..., Rep(A_k)]}
    // Each A_i is `E_i_1 => E_i_2` — a map_field_assoc.
    let v = Expr::MapCreate(
        anno(),
        vec![MapFieldAssoc(
            anno(),
            Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
            Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
        )],
    );
    let expected = term(tuple(&[
        atom("map"),
        anno_bytes(),
        list(&[tuple(&[atom("map_field_assoc"), anno_bytes(), atom_lit("k"), integer_lit(1)])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_map_update() {
    // Rep(E) = {map, ANNO, Rep(E_0), [Rep(A_1), ..., Rep(A_k)]}
    let v = Expr::MapUpdate(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "m"))),
        vec![MapField::MapFieldExact(
            anno(),
            Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
            Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
        )],
    );
    let expected = term(tuple(&[
        atom("map"),
        anno_bytes(),
        atom_lit("m"),
        list(&[tuple(&[atom("map_field_exact"), anno_bytes(), atom_lit("k"), integer_lit(1)])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_record_create() {
    // Rep(E) = {record, ANNO, Name, [{record_field, ANNO, Rep(Field_i), Rep(E_i)}, ...]}
    let v = Expr::RecordCreate(
        anno(),
        "rec",
        vec![RecordField(
            anno(),
            AtomOrVar::Atom(anno(), "f"),
            Some(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
        )],
    );
    let expected = term(tuple(&[
        atom("record"),
        anno_bytes(),
        atom("rec"),
        list(&[tuple(&[atom("record_field"), anno_bytes(), atom_lit("f"), integer_lit(1)])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_record_field_access() {
    // Rep(E) = {record_field, ANNO, Rep(E_0), Name, Rep(Field)}
    let v = Expr::RecordField(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "r"))),
        "rec",
        Atom(anno(), "f"),
    );
    let expected = term(tuple(&[
        atom("record_field"),
        anno_bytes(),
        atom_lit("r"),
        atom("rec"),
        tuple(&[atom("atom"), anno_bytes(), atom("f")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_record_index() {
    // Rep(E) = {record_index, ANNO, Name, Rep(Field)}
    let v = Expr::RecordIndex(anno(), "rec", Atom(anno(), "f"));
    let expected = term(tuple(&[
        atom("record_index"),
        anno_bytes(),
        atom("rec"),
        tuple(&[atom("atom"), anno_bytes(), atom("f")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_record_update() {
    // Rep(E) = {record, ANNO, Rep(E_0), Name, [record_field..., ...]}
    let v = Expr::RecordUpdate(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "r"))),
        "rec",
        vec![RecordField(
            anno(),
            Atom(anno(), "f"),
            Some(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
        )],
    );
    let expected = term(tuple(&[
        atom("record"),
        anno_bytes(),
        atom_lit("r"),
        atom("rec"),
        list(&[tuple(&[
            atom("record_field"),
            anno_bytes(),
            tuple(&[atom("atom"), anno_bytes(), atom("f")]),
            integer_lit(1),
        ])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_remote_call() {
    // Rep(E) = {call, ANNO, {remote, ANNO, Rep(E_m), Rep(E_0)}, [args]}
    let v = Expr::RemoteCall(
        anno(),
        Remote(
            anno(),
            Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "m"))),
            Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "f"))),
        ),
        vec![Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))],
    );
    let expected = term(tuple(&[
        atom("call"),
        anno_bytes(),
        tuple(&[atom("remote"), anno_bytes(), atom_lit("m"), atom_lit("f")]),
        list(&[integer_lit(1)]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_local_fun() {
    // Rep(E) = {'fun', ANNO, {function, Name, Arity}}
    let v = Expr::LocalFun(anno(), Function { name: "foo", arity: 2 });
    let expected = term(tuple(&[
        atom("fun"),
        anno_bytes(),
        tuple(&[atom("function"), atom("foo"), small_int(2)]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_remote_fun() {
    // Rep(E) = {'fun', ANNO, {function, Rep(Module), Rep(Name), Rep(Arity)}}
    let v = Expr::RemoteFun(
        anno(),
        RemoteFunction(
            Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "m"))),
            Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "f"))),
            Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(2u32)))),
        ),
    );
    let expected = term(tuple(&[
        atom("fun"),
        anno_bytes(),
        tuple(&[atom("function"), atom_lit("m"), atom_lit("f"), integer_lit(2)]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_claused_fun() {
    // Rep(E) = {'fun', ANNO, {clauses, [Rep(Fc_1), ..., Rep(Fc_k)]}}
    let v = Expr::ClausedFun(anno(), Clauses { clauses: vec![] });
    let expected = term(tuple(&[atom("fun"), anno_bytes(), tuple(&[atom("clauses"), nil()])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_maybe_else() {
    // Rep(E) = {'maybe', ANNO, Rep(B), {'else', ANNO, [Rep(Ec_1), ..., Rep(Ec_k)]}}
    let v = Expr::MaybeElse(
        anno(),
        Body(vec![Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))]),
        Else { anno: anno(), clauses: vec![] },
    );
    let expected = term(tuple(&[
        atom("maybe"),
        anno_bytes(),
        list(&[integer_lit(1)]),
        tuple(&[atom("else"), anno_bytes(), nil()]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn expr_receive_after() {
    // Rep(E) = {'receive', ANNO, [Rep(Cc_1), ..., Rep(Cc_k)], Rep(E_0), Rep(B_t)}
    let v = Expr::ReceiveAfter(
        anno(),
        vec![],
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1000u32)))),
        Body(vec![Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "timeout"))]),
    );
    let mut int1000 = vec![SMALL_INTEGER_EXT, 0]; // placeholder, real bytes constructed below
    // 1000 is too large for SMALL_INTEGER_EXT (which only goes to 255), so it uses INTEGER_EXT.
    int1000.clear();
    let integer_1000 = tuple(&[atom("integer"), anno_bytes(), {
        let mut b = vec![98u8]; // INTEGER_EXT
        b.extend_from_slice(&1000i32.to_be_bytes());
        b
    }]);
    let expected = term(tuple(&[
        atom("receive"),
        anno_bytes(),
        nil(),
        integer_1000,
        list(&[atom_lit("timeout")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
    let _ = int1000;
}

#[test]
fn expr_try_minimal() {
    // try B catch ... end → {'try', ANNO, Rep(B), [], [Rep(Tc_1), ..., Rep(Tc_k)], []}
    let v = Expr::Try {
        anno: anno(),
        body: Body(vec![Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "ok"))]),
        of: vec![],
        catch: vec![],
        after: Body(vec![]),
    };
    let expected =
        term(tuple(&[atom("try"), anno_bytes(), list(&[atom_lit("ok")]), nil(), nil(), nil()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Function / RemoteFunction / Remote / Else / Clauses standalone structs */

#[test]
fn function_struct() {
    // {function, Name, Arity}
    let v = Function { name: "foo", arity: 3 };
    let expected = term(tuple(&[atom("function"), atom("foo"), small_int(3)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn remote_function_struct() {
    // {function, Rep(Module), Rep(Name), Rep(Arity)}
    let v = RemoteFunction(
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "m"))),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "f"))),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(2u32)))),
    );
    let expected = term(tuple(&[atom("function"), atom_lit("m"), atom_lit("f"), integer_lit(2)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn remote_struct() {
    // {remote, ANNO, Rep(E_m), Rep(E_0)}
    let v = Remote(
        anno(),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "m"))),
        Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "f"))),
    );
    let expected = term(tuple(&[atom("remote"), anno_bytes(), atom_lit("m"), atom_lit("f")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn else_struct() {
    // {'else', ANNO, [Rep(Ec_1), ..., Rep(Ec_k)]}
    let v = Else { anno: anno(), clauses: vec![] };
    let expected = term(tuple(&[atom("else"), anno_bytes(), nil()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn clauses_struct() {
    // {clauses, [Rep(Fc_1), ..., Rep(Fc_k)]}
    let v = Clauses { clauses: vec![] };
    let expected = term(tuple(&[atom("clauses"), nil()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Clause / CatchClause / CatchWhat */

#[test]
fn clause_struct() {
    // Doc (case clause): {clause, ANNO, [Rep(P)], Rep(Gs), Rep(B)}
    let v = Clause {
        anno: anno(),
        patterns: vec![Pattern::Var(anno(), "X")],
        guards: GuardSeq(vec![]),
        body: Body(vec![Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "ok"))]),
    };
    let expected = term(tuple(&[
        atom("clause"),
        anno_bytes(),
        list(&[var_lit("X")]),
        nil(),
        list(&[atom_lit("ok")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn catch_what_struct() {
    // Rep({X, P, S}) = {tuple, ANNO, [Rep(X), Rep(P), Rep(S)]}
    let v = CatchWhat(
        anno(),
        (
            ExceptionKind::AtomicLiteral(AtomicLiteral::Atom(anno(), "throw")),
            Pattern::Var(anno(), "P"),
            Var(anno(), "S"),
        ),
    );
    let expected = term(tuple(&[
        atom("tuple"),
        anno_bytes(),
        list(&[tuple(&[atom("atom"), anno_bytes(), atom("throw")]), var_lit("P"), var_lit("S")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn catch_clause_struct() {
    // Doc: {clause, ANNO, [Rep({X, P, S})], Rep(Gs), Rep(B)}
    // Rep(B) is a list of expressions.
    let v = CatchClause {
        anno: anno(),
        what: (CatchWhat(
            anno(),
            (
                ExceptionKind::AtomicLiteral(AtomicLiteral::Atom(anno(), "throw")),
                Pattern::Var(anno(), "P"),
                Var(anno(), "S"),
            ),
        ),),
        guards: GuardSeq(vec![]),
        body: Body(vec![Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "ok"))]),
    };
    let inner_tuple = tuple(&[
        atom("tuple"),
        anno_bytes(),
        list(&[tuple(&[atom("atom"), anno_bytes(), atom("throw")]), var_lit("P"), var_lit("S")]),
    ]);
    let expected = term(tuple(&[
        atom("clause"),
        anno_bytes(),
        list(&[inner_tuple]),
        nil(),
        list(&[atom_lit("ok")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* BinElement standalone */

#[test]
fn bin_element_struct_full() {
    // {bin_element, ANNO, Rep(V), Rep(Size), Rep(TSL)}
    let v = BinElement::<Expr> {
        anno: anno(),
        value: Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(42u32))),
        size: BinElemSize::Size(Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(
            anno(),
            BigUint::from(8u32),
        )))),
        tsl: BinElemTsl::Tsl(vec![Tsl::Integer, Tsl::Unit { size: 1 }]),
    };
    let expected = term(tuple(&[
        atom("bin_element"),
        anno_bytes(),
        integer_lit(42),
        integer_lit(8),
        list(&[atom("integer"), tuple(&[atom("unit"), small_int(1)])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn bin_elem_size_with_expr_is_transparent() {
    // BinElemSize::Size wraps an Expr without adding a tag (newtype variant).
    let v = BinElemSize::Size(Box::new(Expr::AtomicLiteral(AtomicLiteral::Integer(
        anno(),
        BigUint::from(8u32),
    ))));
    assert_eq!(to_bytes(&v).unwrap(), term(integer_lit(8)));
}

#[test]
fn bin_elem_tsl_with_list_is_transparent() {
    let v = BinElemTsl::Tsl(vec![Tsl::Integer, Tsl::Signed]);
    let expected = term(list(&[atom("integer"), atom("signed")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* MapFieldAssoc / MapFieldExact standalone structs */

#[test]
fn map_field_assoc_struct() {
    // {map_field_assoc, ANNO, Rep(K), Rep(V)}
    let v = MapFieldAssoc::<Expr, Expr>(
        anno(),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
    );
    let expected =
        term(tuple(&[atom("map_field_assoc"), anno_bytes(), atom_lit("k"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn map_field_exact_struct() {
    // {map_field_exact, ANNO, Rep(K), Rep(V)}
    let v = MapFieldExact::<Expr, Expr>(
        anno(),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
    );
    let expected =
        term(tuple(&[atom("map_field_exact"), anno_bytes(), atom_lit("k"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* RecordField standalone struct */

#[test]
fn record_field_struct() {
    // {record_field, ANNO, Rep(Field), Rep(V)}
    let v = RecordField::<Atom, Expr>(
        anno(),
        Atom(anno(), "f"),
        Some(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
    );
    let expected = term(tuple(&[
        atom("record_field"),
        anno_bytes(),
        tuple(&[atom("atom"), anno_bytes(), atom("f")]),
        integer_lit(1),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn record_field_struct_without_value() {
    // V = A (no default): Rep(V) = {record_field, ANNO, Rep(A)} — 3-tuple, no value.
    let v = RecordField::<Atom, Expr>(anno(), Atom(anno(), "f"), None);
    let expected = term(tuple(&[
        atom("record_field"),
        anno_bytes(),
        tuple(&[atom("atom"), anno_bytes(), atom("f")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Qualifier variants */

#[test]
fn qualifier_filter_is_transparent() {
    // Rep(Q) = Rep(E)
    let v = Qualifier::Filter(Box::new(Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "true"))));
    assert_eq!(to_bytes(&v).unwrap(), term(atom_lit("true")));
}

#[test]
fn qualifier_generator_is_transparent() {
    // Rep(Q) = Rep(Generator) — newtype variant
    let v = Qualifier::Generator(Box::new(Generator::Generate(
        anno(),
        Pattern::Var(anno(), "X"),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "list")),
    )));
    let expected = term(tuple(&[atom("generate"), anno_bytes(), var_lit("X"), atom_lit("list")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn qualifier_zip() {
    // Rep(Q) = {zip, ANNO, [Rep(Q_1), ..., Rep(Q_k)]}
    let v = Qualifier::Zip(
        anno(),
        vec![Generator::Generate(
            anno(),
            Pattern::Var(anno(), "X"),
            Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "list")),
        )],
    );
    let expected = term(tuple(&[
        atom("zip"),
        anno_bytes(),
        list(&[tuple(&[atom("generate"), anno_bytes(), var_lit("X"), atom_lit("list")])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Generator variants — remaining */

#[test]
fn generator_b_generate_strict() {
    let v = Generator::BGenerateStrict(
        anno(),
        Pattern::Var(anno(), "X"),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "bin")),
    );
    let expected =
        term(tuple(&[atom("b_generate_strict"), anno_bytes(), var_lit("X"), atom_lit("bin")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn generator_m_generate_strict() {
    let v = Generator::MGenerateStrict(
        anno(),
        Pattern::Var(anno(), "X"),
        Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "map")),
    );
    let expected =
        term(tuple(&[atom("m_generate_strict"), anno_bytes(), var_lit("X"), atom_lit("map")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* GuardTest variants — remaining */

#[test]
fn guard_test_cons() {
    // Rep(Gt) = {cons, ANNO, Rep(Gt_h), Rep(Gt_t)}
    let v = GuardTest::Cons(
        anno(),
        Box::new(GuardTest::Var(anno(), "H")),
        Box::new(GuardTest::Nil { anno: anno() }),
    );
    let expected = term(tuple(&[
        atom("cons"),
        anno_bytes(),
        var_lit("H"),
        tuple(&[atom("nil"), anno_bytes()]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_tuple() {
    // Rep(Gt) = {tuple, ANNO, [Rep(Gt_1), ..., Rep(Gt_k)]}
    let v =
        GuardTest::Tuple(anno(), vec![GuardTest::Var(anno(), "X"), GuardTest::Var(anno(), "Y")]);
    let expected = term(tuple(&[atom("tuple"), anno_bytes(), list(&[var_lit("X"), var_lit("Y")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_local_call() {
    // Rep(Gt) = {call, ANNO, Rep(A), [Rep(Gt_1), ..., Rep(Gt_k)]}
    let v =
        GuardTest::LocalCall(anno(), Atom(anno(), "is_atom"), vec![GuardTest::Var(anno(), "X")]);
    let expected = term(tuple(&[
        atom("call"),
        anno_bytes(),
        tuple(&[atom("atom"), anno_bytes(), atom("is_atom")]),
        list(&[var_lit("X")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_remote_call() {
    // Rep(Gt) = {call, ANNO, {remote, ANNO, Rep(A_m), Rep(A)}, [args]}
    let v = GuardTest::RemoteCall(
        anno(),
        Remote(anno(), Box::new(Atom(anno(), "erlang")), Box::new(Atom(anno(), "is_atom"))),
        vec![GuardTest::Var(anno(), "X")],
    );
    let expected = term(tuple(&[
        atom("call"),
        anno_bytes(),
        tuple(&[
            atom("remote"),
            anno_bytes(),
            tuple(&[atom("atom"), anno_bytes(), atom("erlang")]),
            tuple(&[atom("atom"), anno_bytes(), atom("is_atom")]),
        ]),
        list(&[var_lit("X")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_map_create() {
    // Rep(Gt) = {map, ANNO, [Rep(A_1), ..., Rep(A_k)]}
    let v = GuardTest::MapCreate(
        anno(),
        vec![MapFieldAssoc(
            anno(),
            GuardTest::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
            GuardTest::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
        )],
    );
    let expected = term(tuple(&[
        atom("map"),
        anno_bytes(),
        list(&[tuple(&[atom("map_field_assoc"), anno_bytes(), atom_lit("k"), integer_lit(1)])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_map_update() {
    // Rep(Gt) = {map, ANNO, Rep(Gt_0), [...]}
    let v = GuardTest::MapUpdate(
        anno(),
        Box::new(GuardTest::Var(anno(), "M")),
        vec![MapField::MapFieldAssoc(
            anno(),
            GuardTest::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
            GuardTest::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
        )],
    );
    let expected = term(tuple(&[
        atom("map"),
        anno_bytes(),
        var_lit("M"),
        list(&[tuple(&[atom("map_field_assoc"), anno_bytes(), atom_lit("k"), integer_lit(1)])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_record() {
    // Rep(Gt) = {record, ANNO, Name, [record_field..., ...]}
    // NOTE: impl tag is "record" but the snake_case-default of the `Record` variant
    // would also be "record" — but it's NOT renamed and the variant name `Record`
    // snake_cases to "record". OK.
    let v = GuardTest::Record(
        anno(),
        "rec",
        vec![RecordField(anno(), AtomOrVar::Atom(anno(), "f"), Some(GuardTest::Var(anno(), "X")))],
    );
    let expected = term(tuple(&[
        atom("record"),
        anno_bytes(),
        atom("rec"),
        list(&[tuple(&[atom("record_field"), anno_bytes(), atom_lit("f"), var_lit("X")])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_record_field() {
    // Rep(Gt) = {record_field, ANNO, Rep(Gt_0), Name, Rep(Field)}
    let v = GuardTest::RecordField(
        anno(),
        Box::new(GuardTest::Var(anno(), "R")),
        "rec",
        Atom(anno(), "f"),
    );
    let expected = term(tuple(&[
        atom("record_field"),
        anno_bytes(),
        var_lit("R"),
        atom("rec"),
        tuple(&[atom("atom"), anno_bytes(), atom("f")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_record_index() {
    let v = GuardTest::RecordIndex(anno(), "rec", Atom(anno(), "f"));
    let expected = term(tuple(&[
        atom("record_index"),
        anno_bytes(),
        atom("rec"),
        tuple(&[atom("atom"), anno_bytes(), atom("f")]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn guard_test_bin() {
    let elem = BinElement::<GuardTest> {
        anno: anno(),
        value: GuardTest::Var(anno(), "X"),
        size: BinElemSize::Default,
        tsl: BinElemTsl::Default,
    };
    let v = GuardTest::Bin(anno(), vec![elem]);
    let expected = term(tuple(&[
        atom("bin"),
        anno_bytes(),
        list(&[tuple(&[
            atom("bin_element"),
            anno_bytes(),
            var_lit("X"),
            atom("default"),
            atom("default"),
        ])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Types */

#[test]
fn type_atomic_literal_is_transparent() {
    let v = Type::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(5u32)));
    assert_eq!(to_bytes(&v).unwrap(), term(integer_lit(5)));
}

#[test]
fn type_ann_type() {
    // Rep(T) = {ann_type, ANNO, [Rep(A), Rep(T_0)]}
    let v = Type::AnnType(
        anno(),
        (Var(anno(), "A"), Box::new(Type::Predefined(anno(), "atom", vec![]))),
    );
    let expected = term(tuple(&[
        atom("ann_type"),
        anno_bytes(),
        list(&[var_lit("A"), predefined_type("atom", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_binary() {
    // Rep(T) = {type, ANNO, binary, [Rep(M), Rep(N)]}
    let v = Type::Binary(anno(), (integer_struct(8), integer_struct(0)));
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("binary"),
        list(&[integer_struct_bytes(8), integer_struct_bytes(0)]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_nil() {
    // Rep(T) = {type, ANNO, nil, []}
    let v = Type::Nil(anno(), ());
    let expected = term(predefined_type("nil", &[]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_fun_no_args() {
    // Rep(T) = {type, ANNO, 'fun', []}
    let v = Type::Fun(anno(), ());
    let expected = term(tuple(&[atom("type"), anno_bytes(), atom("fun"), nil()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_any_fun() {
    // Rep(T) = {type, ANNO, 'fun', [{type, ANNO, any}, Rep(T_0)]}
    let v = Type::AnyFun(
        anno(),
        (AnyType { anno: anno() }, Box::new(Type::Predefined(anno(), "atom", vec![]))),
    );
    let any_bytes = tuple(&[atom("type"), anno_bytes(), atom("any")]);
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("fun"),
        list(&[any_bytes, predefined_type("atom", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_range() {
    // Rep(T) = {type, ANNO, range, [Rep(L), Rep(H)]}
    // L and H are singleton integer types — using integer literals here.
    let v = Type::Range(
        anno(),
        (
            Box::new(Type::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32)))),
            Box::new(Type::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(10u32)))),
        ),
    );
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("range"),
        list(&[integer_lit(1), integer_lit(10)]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_map_any() {
    // Rep(T) = {type, ANNO, map, any}  (the `t:map/0` case)
    let v = Type::Map(anno(), MapTypeAssocs::Any);
    let expected = term(tuple(&[atom("type"), anno_bytes(), atom("map"), atom("any")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_map_with_assocs() {
    // Rep(T) = {type, ANNO, map, [Rep(A_1), ..., Rep(A_k)]}
    let assoc = AssociationType::MapFieldAssoc(
        anno(),
        (Type::Predefined(anno(), "atom", vec![]), Type::Predefined(anno(), "integer", vec![])),
    );
    let v = Type::Map(anno(), MapTypeAssocs::Assocs(vec![assoc]));
    let assoc_bytes = tuple(&[
        atom("type"),
        anno_bytes(),
        atom("map_field_assoc"),
        list(&[predefined_type("atom", &[]), predefined_type("integer", &[])]),
    ]);
    let expected = term(tuple(&[atom("type"), anno_bytes(), atom("map"), list(&[assoc_bytes])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_binary_op() {
    // Rep(T) = {op, ANNO, Op, Rep(T_1), Rep(T_2)}
    let v = Type::BinaryOp(
        anno(),
        "*",
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(2u32))),
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(3u32))),
    );
    let expected =
        term(tuple(&[atom("op"), anno_bytes(), atom("*"), integer_lit(2), integer_lit(3)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_unary_op() {
    // Rep(T) = {op, ANNO, Op, Rep(T_0)}
    let v = Type::UnaryOp(
        anno(),
        "-",
        Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
    );
    let expected = term(tuple(&[atom("op"), anno_bytes(), atom("-"), integer_lit(1)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_parenthesized_is_transparent() {
    let v = Type::Parenthesized(Box::new(Type::Predefined(anno(), "atom", vec![])));
    assert_eq!(to_bytes(&v).unwrap(), term(predefined_type("atom", &[])));
}

#[test]
fn type_predefined_no_args() {
    // Rep(T) = {type, ANNO, N, [Rep(T_1), ..., Rep(T_k)]}
    let v = Type::Predefined(anno(), "atom", vec![]);
    assert_eq!(to_bytes(&v).unwrap(), term(predefined_type("atom", &[])));
}

#[test]
fn type_predefined_with_args() {
    let v = Type::Predefined(anno(), "list", vec![Type::Predefined(anno(), "atom", vec![])]);
    let expected = term(predefined_type("list", &[predefined_type("atom", &[])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_record() {
    // Rep(T) = {type, ANNO, record, [Rep(Name), Rep(F_1), ..., Rep(F_k)]}
    let field =
        RecordFieldType(anno(), (Atom(anno(), "f"), Type::Predefined(anno(), "atom", vec![])));
    let v = Type::Record(anno(), RecordTypeFields(Atom(anno(), "rec"), vec![field]));
    let field_bytes = tuple(&[
        atom("type"),
        anno_bytes(),
        atom("field_type"),
        list(&[tuple(&[atom("atom"), anno_bytes(), atom("f")]), predefined_type("atom", &[])]),
    ]);
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("record"),
        list(&[tuple(&[atom("atom"), anno_bytes(), atom("rec")]), field_bytes]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_remote_type() {
    // Rep(T) = {remote_type, ANNO, [Rep(M), Rep(N), [Rep(T_1), ..., Rep(T_k)]]}
    let v = Type::RemoteType(
        anno(),
        (Atom(anno(), "m"), Atom(anno(), "t"), vec![Type::Predefined(anno(), "atom", vec![])]),
    );
    let expected = term(tuple(&[
        atom("remote_type"),
        anno_bytes(),
        list(&[
            tuple(&[atom("atom"), anno_bytes(), atom("m")]),
            tuple(&[atom("atom"), anno_bytes(), atom("t")]),
            list(&[predefined_type("atom", &[])]),
        ]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_tuple_any() {
    // Rep(T) = {type, ANNO, tuple, any}
    let v = Type::Tuple(anno(), TupleTypeElem::Any);
    let expected = term(tuple(&[atom("type"), anno_bytes(), atom("tuple"), atom("any")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_tuple_with_elements() {
    // Rep(T) = {type, ANNO, tuple, [Rep(T_1), ..., Rep(T_k)]}
    let v = Type::Tuple(
        anno(),
        TupleTypeElem::Assocs(vec![
            Type::Predefined(anno(), "atom", vec![]),
            Type::Predefined(anno(), "integer", vec![]),
        ]),
    );
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("tuple"),
        list(&[predefined_type("atom", &[]), predefined_type("integer", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_union() {
    // Rep(T) = {type, ANNO, union, [Rep(T_1), ..., Rep(T_k)]}
    let v = Type::Union(
        anno(),
        vec![Type::Predefined(anno(), "atom", vec![]), Type::Predefined(anno(), "integer", vec![])],
    );
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("union"),
        list(&[predefined_type("atom", &[]), predefined_type("integer", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_var() {
    // Rep(T) = {var, ANNO, A}
    let v = Type::Var(anno(), "X");
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}

#[test]
fn type_user_type() {
    // Rep(T) = {user_type, ANNO, N, [Rep(T_1), ..., Rep(T_k)]}
    let v = Type::UserType(anno(), "my_t", vec![Type::Var(anno(), "X")]);
    let expected =
        term(tuple(&[atom("user_type"), anno_bytes(), atom("my_t"), list(&[var_lit("X")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* AnyType / ProductType / FunctionType / FunctionConstraint / Constraint / IsSubtype */

#[test]
fn any_type_struct() {
    // {type, ANNO, any}
    let v = AnyType { anno: anno() };
    let expected = term(tuple(&[atom("type"), anno_bytes(), atom("any")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn product_type_struct() {
    // {type, ANNO, product, [Rep(T_1), ..., Rep(T_n)]}
    let v = ProductType(anno(), vec![Type::Predefined(anno(), "atom", vec![])]);
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("product"),
        list(&[predefined_type("atom", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn function_type_fun() {
    // Rep(Ft) = {type, ANNO, 'fun', [{type, ANNO, product, [...]}, Rep(T_0)]}
    let v = FunctionType::Fun(
        anno(),
        (
            ProductType(anno(), vec![Type::Var(anno(), "X")]),
            Box::new(Type::Predefined(anno(), "atom", vec![])),
        ),
    );
    let product_bytes =
        tuple(&[atom("type"), anno_bytes(), atom("product"), list(&[var_lit("X")])]);
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("fun"),
        list(&[product_bytes, predefined_type("atom", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn function_type_bounded_fun() {
    // Rep(T) = {type, ANNO, bounded_fun, [Rep(Ft_1), Rep(Fc)]}
    let inner_fun = FunctionType::Fun(
        anno(),
        (ProductType(anno(), vec![]), Box::new(Type::Predefined(anno(), "atom", vec![]))),
    );
    let fc = FunctionConstraint(vec![Constraint(
        anno(),
        (
            IsSubtype(anno(), IsSubtypeMark),
            (Var(anno(), "X"), Type::Predefined(anno(), "atom", vec![])),
        ),
    )]);
    let v = FunctionType::BoundedFun(anno(), (Box::new(inner_fun), fc));

    let product_bytes = tuple(&[atom("type"), anno_bytes(), atom("product"), nil()]);
    let inner_fun_bytes = tuple(&[
        atom("type"),
        anno_bytes(),
        atom("fun"),
        list(&[product_bytes, predefined_type("atom", &[])]),
    ]);
    let is_subtype_bytes = tuple(&[atom("atom"), anno_bytes(), atom("is_subtype")]);
    let constraint_bytes = tuple(&[
        atom("type"),
        anno_bytes(),
        atom("constraint"),
        list(&[is_subtype_bytes, list(&[var_lit("X"), predefined_type("atom", &[])])]),
    ]);
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("bounded_fun"),
        list(&[inner_fun_bytes, list(&[constraint_bytes])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn function_constraint_is_transparent_list() {
    // Rep(Fc) = [Rep(C_1), ..., Rep(C_k)]
    let v = FunctionConstraint(vec![]);
    assert_eq!(to_bytes(&v).unwrap(), term(nil()));
}

#[test]
fn constraint_struct() {
    // {type, ANNO, constraint, [{atom, ANNO, is_subtype}, [Rep(V), Rep(T)]]}
    let v = Constraint(
        anno(),
        (
            IsSubtype(anno(), IsSubtypeMark),
            (Var(anno(), "X"), Type::Predefined(anno(), "atom", vec![])),
        ),
    );
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("constraint"),
        list(&[
            tuple(&[atom("atom"), anno_bytes(), atom("is_subtype")]),
            list(&[var_lit("X"), predefined_type("atom", &[])]),
        ]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn is_subtype_wraps_mark_as_atom() {
    // {atom, ANNO, is_subtype}
    let v = IsSubtype(anno(), IsSubtypeMark);
    let expected = term(tuple(&[atom("atom"), anno_bytes(), atom("is_subtype")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn is_subtype_mark_is_atom() {
    // bare atom `is_subtype`
    assert_eq!(to_bytes(&IsSubtypeMark).unwrap(), term(atom("is_subtype")));
}

/* AssociationType */

#[test]
fn association_type_map_field_assoc() {
    // {type, ANNO, map_field_assoc, [Rep(K), Rep(V)]}
    let v = AssociationType::MapFieldAssoc(
        anno(),
        (Type::Predefined(anno(), "atom", vec![]), Type::Predefined(anno(), "integer", vec![])),
    );
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("map_field_assoc"),
        list(&[predefined_type("atom", &[]), predefined_type("integer", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn association_type_map_field_exact() {
    let v = AssociationType::MapFieldExact(
        anno(),
        (Type::Predefined(anno(), "atom", vec![]), Type::Predefined(anno(), "integer", vec![])),
    );
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("map_field_exact"),
        list(&[predefined_type("atom", &[]), predefined_type("integer", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* RecordFieldType */

#[test]
fn record_field_type_struct() {
    // Doc: {type, ANNO, field_type, [Rep(Name), Rep(Type)]}
    let v = RecordFieldType(anno(), (Atom(anno(), "f"), Type::Predefined(anno(), "atom", vec![])));
    let expected = term(tuple(&[
        atom("type"),
        anno_bytes(),
        atom("field_type"),
        list(&[tuple(&[atom("atom"), anno_bytes(), atom("f")]), predefined_type("atom", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Regression tests for the doc-vs-impl divergences previously identified. */

#[test]
fn guard_test_map_update_with_exact_association() {
    // Doc: each A_i is `Gt_i_1 => Gt_i_2` or `Gt_i_1 := Gt_i_2`.
    let v = GuardTest::MapUpdate(
        anno(),
        Box::new(GuardTest::Var(anno(), "M")),
        vec![MapField::MapFieldExact(
            anno(),
            GuardTest::AtomicLiteral(AtomicLiteral::Atom(anno(), "k")),
            GuardTest::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
        )],
    );
    let expected = term(tuple(&[
        atom("map"),
        anno_bytes(),
        var_lit("M"),
        list(&[tuple(&[atom("map_field_exact"), anno_bytes(), atom_lit("k"), integer_lit(1)])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn exception_kind_integer_literal() {
    // Doc (CatchClause): `X` is "an atomic literal or a variable pattern" —
    // any atomic literal, not just an atom.
    let v = ExceptionKind::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(42u32)));
    assert_eq!(to_bytes(&v).unwrap(), term(integer_lit(42)));
}

#[test]
fn type_range_with_negated_lower_bound() {
    // Doc: L and H are singleton integer types, which include a negated
    // integer literal expressed via unary `op` `-`.
    let v = Type::Range(
        anno(),
        (
            Box::new(Type::UnaryOp(
                anno(),
                "-",
                Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(1u32))),
            )),
            Box::new(Type::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(5u32)))),
        ),
    );
    let neg_one = tuple(&[atom("op"), anno_bytes(), atom("-"), integer_lit(1)]);
    let expected =
        term(tuple(&[atom("type"), anno_bytes(), atom("range"), list(&[neg_one, integer_lit(5)])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Module Declarations and Forms */

/// `{attribute, ANNO, name, rest...}` shortcut.
fn attribute(name: &str, rest: &[Vec<u8>]) -> Vec<u8> {
    let mut parts = vec![atom("attribute"), anno_bytes(), atom(name)];
    parts.extend_from_slice(rest);
    tuple(&parts)
}

#[test]
fn module_struct_is_transparent_list_of_forms() {
    // Doc: Rep(D) = [Rep(F_1), ..., Rep(F_k)] — a bare list, no struct tag.
    let v = Module(vec![ModuleForm::ModuleAttr(anno(), "m")]);
    let expected = term(list(&[attribute("module", &[atom("m")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_struct_empty() {
    let v = Module::<'_>(vec![]);
    assert_eq!(to_bytes(&v).unwrap(), term(nil()));
}

#[test]
fn module_form_module_attr() {
    // Rep(F) = {attribute, ANNO, module, Mod}
    let v = ModuleForm::ModuleAttr(anno(), "mymod");
    let expected = term(attribute("module", &[atom("mymod")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_export_attr() {
    // Rep(F) = {attribute, ANNO, export, [{Fun_1,A_1}, ..., {Fun_k,A_k}]}
    let v = ModuleForm::ExportAttr(anno(), vec![NameArity("foo", 0), NameArity("bar", 2)]);
    let expected = term(attribute(
        "export",
        &[list(&[tuple(&[atom("foo"), small_int(0)]), tuple(&[atom("bar"), small_int(2)])])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_export_attr_empty() {
    // Empty export list — still a 4-tuple, the fourth element is just [].
    let v = ModuleForm::ExportAttr(anno(), vec![]);
    let expected = term(attribute("export", &[nil()]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_import_attr() {
    // Rep(F) = {attribute, ANNO, import, {Mod, [{Fun_1,A_1}, ..., {Fun_k,A_k}]}}
    // The fourth element is an Erlang 2-tuple {Mod, [..]}, NOT a list.
    let v = ModuleForm::ImportAttr(anno(), ImportModFuns("lists", vec![NameArity("map", 2)]));
    let expected = term(attribute(
        "import",
        &[tuple(&[atom("lists"), list(&[tuple(&[atom("map"), small_int(2)])])])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_file_attr() {
    // Rep(F) = {attribute, ANNO, file, {File, Line}}
    // The fourth element is an Erlang 2-tuple {File, Line}, NOT a list.
    let v = ModuleForm::FileAttr(anno(), file_line("foo.erl", 42));
    let expected = term(attribute("file", &[tuple(&[char_list("foo.erl"), small_int(42)])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_function() {
    // Rep(F) = {function, ANNO, Name, Arity, [Rep(Fc_1), ..., Rep(Fc_k)]}
    let clause = Clause {
        anno: anno(),
        patterns: vec![],
        guards: GuardSeq(vec![]),
        body: Body(vec![Expr::AtomicLiteral(AtomicLiteral::Atom(anno(), "ok"))]),
    };
    let v = ModuleForm::Function(anno(), "f", 0, vec![clause]);
    let expected_clause =
        tuple(&[atom("clause"), anno_bytes(), nil(), nil(), list(&[atom_lit("ok")])]);
    let expected = term(tuple(&[
        atom("function"),
        anno_bytes(),
        atom("f"),
        small_int(0),
        list(&[expected_clause]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_spec_attr_spec() {
    // Rep(F) = {attribute, ANNO, spec, {{Name, Arity}, [Rep(Ft)]}}
    let v = ModuleForm::SpecAttr(
        anno(),
        SpecType::Spec,
        Spec(NameArity("f", 1), vec![Type::Predefined(anno(), "atom", vec![])]),
    );
    let expected = term(attribute(
        "spec",
        &[tuple(&[tuple(&[atom("f"), small_int(1)]), list(&[predefined_type("atom", &[])])])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_spec_attr_callback() {
    // Rep(F) = {attribute, ANNO, callback, {{Name, Arity}, [Rep(Ft)]}}
    let v = ModuleForm::SpecAttr(
        anno(),
        SpecType::Callback,
        Spec(NameArity("init", 1), vec![Type::Predefined(anno(), "atom", vec![])]),
    );
    let expected = term(attribute(
        "callback",
        &[tuple(&[tuple(&[atom("init"), small_int(1)]), list(&[predefined_type("atom", &[])])])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_remote_spec_attr() {
    // Rep(F) = {attribute, ANNO, spec, {{Mod, Name, Arity}, [Rep(Ft)]}}
    let v = ModuleForm::RemoteSpecAttr(
        anno(),
        RemoteSpec(ModNameArity("m", "f", 1), vec![Type::Predefined(anno(), "atom", vec![])]),
    );
    let expected = term(attribute(
        "spec",
        &[tuple(&[
            tuple(&[atom("m"), atom("f"), small_int(1)]),
            list(&[predefined_type("atom", &[])]),
        ])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_record_attr_untyped_with_default() {
    // Rep(F) = {attribute, ANNO, record, {Name, [Rep(V)]}}
    // With default: Rep(V) = {record_field, ANNO, Rep(A), Rep(E)}
    let v = ModuleForm::RecordAttr(
        anno(),
        RecordDecl(
            "r",
            vec![RecordFieldDecl::Untyped(RecordField(
                anno(),
                Atom(anno(), "x"),
                Some(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(0u32)))),
            ))],
        ),
    );
    let expected = term(attribute(
        "record",
        &[tuple(&[
            atom("r"),
            list(&[tuple(&[atom("record_field"), anno_bytes(), atom_lit("x"), integer_lit(0)])]),
        ])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_record_attr_untyped_without_default() {
    // Without default: Rep(V) = {record_field, ANNO, Rep(A)} — 3-tuple, no value.
    let v = ModuleForm::RecordAttr(
        anno(),
        RecordDecl(
            "r",
            vec![RecordFieldDecl::Untyped(RecordField(anno(), Atom(anno(), "x"), None))],
        ),
    );
    let expected = term(attribute(
        "record",
        &[tuple(&[
            atom("r"),
            list(&[tuple(&[atom("record_field"), anno_bytes(), atom_lit("x")])]),
        ])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_record_attr_typed_without_default() {
    // Rep(V) = {typed_record_field, {record_field, ANNO, Rep(A)}, Rep(T)}
    let v = ModuleForm::RecordAttr(
        anno(),
        RecordDecl(
            "r",
            vec![RecordFieldDecl::Typed(Box::new(TypedRecordField(
                RecordField(anno(), Atom(anno(), "x"), None),
                Type::Predefined(anno(), "atom", vec![]),
            )))],
        ),
    );
    let inner_record_field = tuple(&[atom("record_field"), anno_bytes(), atom_lit("x")]);
    let expected = term(attribute(
        "record",
        &[tuple(&[
            atom("r"),
            list(&[tuple(&[
                atom("typed_record_field"),
                inner_record_field,
                predefined_type("atom", &[]),
            ])]),
        ])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_record_attr_typed_with_default() {
    // Rep(V) = {typed_record_field, {record_field, ANNO, Rep(A), Rep(E)}, Rep(T)}
    let v = ModuleForm::RecordAttr(
        anno(),
        RecordDecl(
            "r",
            vec![RecordFieldDecl::Typed(Box::new(TypedRecordField(
                RecordField(
                    anno(),
                    Atom(anno(), "x"),
                    Some(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(0u32)))),
                ),
                Type::Predefined(anno(), "atom", vec![]),
            )))],
        ),
    );
    let inner_record_field =
        tuple(&[atom("record_field"), anno_bytes(), atom_lit("x"), integer_lit(0)]);
    let expected = term(attribute(
        "record",
        &[tuple(&[
            atom("r"),
            list(&[tuple(&[
                atom("typed_record_field"),
                inner_record_field,
                predefined_type("atom", &[]),
            ])]),
        ])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_type_attr_type() {
    // Rep(F) = {attribute, ANNO, type, {Name, Rep(T), [Rep(V_1), ..., Rep(V_k)]}}
    let v = ModuleForm::TypeAttr(
        anno(),
        TypeDeclType::Type,
        Box::new(TypeDecl("my_t", Type::Predefined(anno(), "atom", vec![]), vec![])),
    );
    let expected =
        term(attribute("type", &[tuple(&[atom("my_t"), predefined_type("atom", &[]), nil()])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_type_attr_opaque() {
    // Rep(F) = {attribute, ANNO, opaque, {Name, Rep(T), [Rep(V_1), ..., Rep(V_k)]}}
    let v = ModuleForm::TypeAttr(
        anno(),
        TypeDeclType::Opaque,
        Box::new(TypeDecl(
            "handle",
            Type::Predefined(anno(), "reference", vec![]),
            vec![Type::Var(anno(), "T")],
        )),
    );
    let expected = term(attribute(
        "opaque",
        &[tuple(&[atom("handle"), predefined_type("reference", &[]), list(&[var_lit("T")])])],
    ));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn module_form_wild_attr() {
    // Rep(F) = {attribute, ANNO, A, T}
    let v = ModuleForm::WildAttr(anno(), "vsn", vec!["v1"]);
    let expected = term(attribute("vsn", &[list(&[atom("v1")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Helper structs that appear inside module forms */

#[test]
fn name_arity_struct_is_2_tuple() {
    // __elide__: {Name, Arity}
    let v = NameArity("foo", 3);
    let expected = term(tuple(&[atom("foo"), small_int(3)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn mod_name_arity_struct_is_3_tuple() {
    // __elide__: {Mod, Name, Arity}
    let v = ModNameArity("lists", "map", 2);
    let expected = term(tuple(&[atom("lists"), atom("map"), small_int(2)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn import_mod_funs_struct_is_2_tuple() {
    // __elide__: {Mod, [NameArity...]}
    let v = ImportModFuns("lists", vec![NameArity("map", 2)]);
    let expected = term(tuple(&[atom("lists"), list(&[tuple(&[atom("map"), small_int(2)])])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn file_line_struct_is_2_tuple() {
    // __elide__: {File, Line} — must be an Erlang tuple, not a list.
    let v = file_line("foo.erl", 42);
    let expected = term(tuple(&[char_list("foo.erl"), small_int(42)]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn spec_type_atoms() {
    assert_eq!(to_bytes(&SpecType::Spec).unwrap(), term(atom("spec")));
    assert_eq!(to_bytes(&SpecType::Callback).unwrap(), term(atom("callback")));
}

#[test]
fn type_decl_type_atoms() {
    assert_eq!(to_bytes(&TypeDeclType::Type).unwrap(), term(atom("type")));
    assert_eq!(to_bytes(&TypeDeclType::Opaque).unwrap(), term(atom("opaque")));
}

#[test]
fn spec_struct() {
    // __elide__: {NameArity, [Types]} → {{Name, Arity}, [Types]}
    let v = Spec(NameArity("f", 1), vec![Type::Predefined(anno(), "atom", vec![])]);
    let expected =
        term(tuple(&[tuple(&[atom("f"), small_int(1)]), list(&[predefined_type("atom", &[])])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn remote_spec_struct() {
    // __elide__: {ModNameArity, [Types]} → {{Mod, Name, Arity}, [Types]}
    let v = RemoteSpec(ModNameArity("m", "f", 1), vec![Type::Predefined(anno(), "atom", vec![])]);
    let expected = term(tuple(&[
        tuple(&[atom("m"), atom("f"), small_int(1)]),
        list(&[predefined_type("atom", &[])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn record_decl_struct() {
    // __elide__: {Name, [Fields]}
    let v = RecordDecl(
        "r",
        vec![RecordFieldDecl::Untyped(RecordField(anno(), Atom(anno(), "x"), None))],
    );
    let expected = term(tuple(&[
        atom("r"),
        list(&[tuple(&[atom("record_field"), anno_bytes(), atom_lit("x")])]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn type_decl_struct() {
    // __elide__: {Name, Type, [Vars]}
    let v = TypeDecl("t", Type::Predefined(anno(), "atom", vec![]), vec![Type::Var(anno(), "A")]);
    let expected = term(tuple(&[atom("t"), predefined_type("atom", &[]), list(&[var_lit("A")])]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn record_field_decl_untyped_is_transparent() {
    // Newtype variants serialize as the inner value.
    let v = RecordFieldDecl::Untyped(RecordField::<Atom, Expr>(anno(), Atom(anno(), "x"), None));
    let expected = term(tuple(&[atom("record_field"), anno_bytes(), atom_lit("x")]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn record_field_decl_typed_is_transparent() {
    // Newtype variants serialize as the inner value.
    let v = RecordFieldDecl::Typed(Box::new(TypedRecordField::<Atom, Expr, Type>(
        RecordField(anno(), Atom(anno(), "x"), None),
        Type::Predefined(anno(), "atom", vec![]),
    )));
    let inner_record_field = tuple(&[atom("record_field"), anno_bytes(), atom_lit("x")]);
    let expected = term(tuple(&[
        atom("typed_record_field"),
        inner_record_field,
        predefined_type("atom", &[]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn typed_record_field_struct_without_default() {
    // V = A :: T → {typed_record_field, {record_field, ANNO, Rep(A)}, Rep(T)}
    let v = TypedRecordField::<Atom, Expr, Type>(
        RecordField(anno(), Atom(anno(), "x"), None),
        Type::Predefined(anno(), "atom", vec![]),
    );
    let inner_record_field = tuple(&[atom("record_field"), anno_bytes(), atom_lit("x")]);
    let expected = term(tuple(&[
        atom("typed_record_field"),
        inner_record_field,
        predefined_type("atom", &[]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

#[test]
fn typed_record_field_struct_with_default() {
    // V = A = E :: T → {typed_record_field, {record_field, ANNO, Rep(A), Rep(E)}, Rep(T)}
    let v = TypedRecordField::<Atom, Expr, Type>(
        RecordField(
            anno(),
            Atom(anno(), "x"),
            Some(Expr::AtomicLiteral(AtomicLiteral::Integer(anno(), BigUint::from(0u32)))),
        ),
        Type::Predefined(anno(), "atom", vec![]),
    );
    let inner_record_field =
        tuple(&[atom("record_field"), anno_bytes(), atom_lit("x"), integer_lit(0)]);
    let expected = term(tuple(&[
        atom("typed_record_field"),
        inner_record_field,
        predefined_type("atom", &[]),
    ]));
    assert_eq!(to_bytes(&v).unwrap(), expected);
}

/* Expr::Var was added in the same change set as ModuleForm. */

#[test]
fn expr_var() {
    // Rep(E) = {var, ANNO, A} — third element is a bare atom, NOT a {atom, ANNO, _}
    // wrapper. Matches Pattern::Var / GuardTest::Var / Type::Var.
    let v = Expr::Var(anno(), "X");
    assert_eq!(to_bytes(&v).unwrap(), term(var_lit("X")));
}
