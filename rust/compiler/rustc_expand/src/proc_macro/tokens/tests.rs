//! Round-trip tests for the token wire protocol.
//!
//! Every test here encodes a stream and decodes it again, which also checks
//! the element counts written by [`super::count_wire_trees`]: the decoder ends with
//! `Reader::finish`, so a count that disagrees with what was written either
//! desynchronizes the reader or leaves trailing bytes.

use rustc_ast::AttrStyle;
use rustc_ast::token::{self, CommentKind, IdentIsRaw};
use rustc_ast::tokenstream::{DelimSpacing, DelimSpan, Spacing, TokenStream, TokenTree};
use rustc_session::parse::ParseSess;
use rustc_span::{BytePos, Span, Symbol, SyntaxContext, create_default_session_globals_then};

use super::{
    DecodeResult, ResolvedSpan, SpanTable, TokenDecodeError, TokenEncodeError, decode_tokens,
    encode_tokens,
};

/// A span covering `lo..hi`, distinct from every span with another range.
fn span(lo: u32, hi: u32) -> Span {
    Span::new(BytePos(lo), BytePos(hi), SyntaxContext::root(), None)
}

/// The three well-known spans, which [`SpanTable::new`] requires to differ.
fn table() -> SpanTable {
    SpanTable::new(span(0, 1), span(1, 2), span(2, 3))
}

fn encode_decode(stream: &TokenStream) -> TokenStream {
    let psess = ParseSess::new();
    let mut spans = table();
    let bytes = encode_tokens(stream, &mut spans).expect("encoding succeeds");
    match decode_tokens(&bytes, &spans, &psess).expect("decoding succeeds") {
        DecodeResult::Normal(decoded) => decoded,
        DecodeResult::HadUnknownSpan(_) => {
            panic!("every span was interned by the encoder")
        }
    }
}

#[track_caller]
fn assert_eq_unspanned(left: &TokenStream, right: &TokenStream) {
    let eq =
        left.len() == right.len() && left.iter().zip(right.iter()).all(|(l, r)| l.eq_unspanned(r));
    assert!(eq, "streams differ\n left: {left:?}\nright: {right:?}");
}

fn ident(name: &str, sp: Span) -> TokenTree {
    TokenTree::token_alone(token::Ident(Symbol::intern(name), IdentIsRaw::No), sp)
}

fn group(delim: token::Delimiter, inner: Vec<TokenTree>, sp: Span) -> TokenTree {
    TokenTree::Delimited(
        DelimSpan::from_single(sp),
        DelimSpacing::new(Spacing::Alone, Spacing::Alone),
        delim,
        TokenStream::new(inner),
    )
}

/// `[{literal, {err}, <<"error">>, nil, 0}]`.
fn encoded_error_literal() -> Vec<u8> {
    let mut bytes = vec![etf::TERM_HEADER];
    etf::write_list_header(&mut bytes, 1).unwrap();
    etf::write_tuple_header(&mut bytes, 5).unwrap();
    etf::write_atom(&mut bytes, "literal").unwrap();
    etf::write_tuple_header(&mut bytes, 1).unwrap();
    etf::write_atom(&mut bytes, "err").unwrap();
    etf::write_binary(&mut bytes, b"error").unwrap();
    etf::write_atom(&mut bytes, "nil").unwrap();
    etf::write_integer(&mut bytes, 0i32).unwrap();
    etf::write_nil(&mut bytes).unwrap();
    bytes
}

#[test]
fn round_trips_idents_ops_and_literals() {
    create_default_session_globals_then(|| {
        let stream = TokenStream::new(vec![
            ident("foo", span(10, 13)),
            TokenTree::token_alone(token::Plus, span(14, 15)),
            TokenTree::token_alone(
                token::TokenKind::lit(token::Integer, Symbol::intern("42"), None),
                span(16, 18),
            ),
            TokenTree::token_alone(
                token::TokenKind::lit(token::Str, Symbol::intern("hi"), Some(sym_suffix())),
                span(19, 25),
            ),
        ]);
        assert_eq_unspanned(&encode_decode(&stream), &stream);
    });
}

fn sym_suffix() -> Symbol {
    Symbol::intern("suffix")
}

#[test]
fn round_trips_nested_and_empty_groups() {
    create_default_session_globals_then(|| {
        let inner = group(token::Delimiter::Bracket, vec![ident("a", span(12, 13))], span(11, 14));
        let empty = group(token::Delimiter::Brace, vec![], span(15, 17));
        let stream = TokenStream::new(vec![
            group(token::Delimiter::Parenthesis, vec![inner, empty], span(10, 18)),
            ident("after", span(19, 24)),
        ]);
        assert_eq_unspanned(&encode_decode(&stream), &stream);
    });
}

#[test]
fn decodes_zero_length_list_ext_stream() {
    create_default_session_globals_then(|| {
        let psess = ParseSess::new();
        let spans = table();

        // `LIST_EXT(0), NIL_EXT`. Unlike `NIL_EXT` alone, the list header has
        // a tail term that the decoder must consume.
        let bytes = [131, 108, 0, 0, 0, 0, 106];
        let DecodeResult::Normal(decoded) = decode_tokens(&bytes, &spans, &psess).expect("decodes")
        else {
            panic!("all span ids are known");
        };
        assert!(decoded.is_empty());
    });
}

#[test]
fn decodes_zero_length_list_ext_inside_group() {
    create_default_session_globals_then(|| {
        let psess = ParseSess::new();
        let spans = table();

        // `[{group, parenthesis, LIST_EXT(0), NIL_EXT, {delim_span, 0, 0, 0}}]`.
        let bytes = [
            131, 108, 0, 0, 0, 1, 104, 4, 119, 5, b'g', b'r', b'o', b'u', b'p', 119, 11, b'p',
            b'a', b'r', b'e', b'n', b't', b'h', b'e', b's', b'i', b's', 108, 0, 0, 0, 0, 106, 104,
            4, 119, 10, b'd', b'e', b'l', b'i', b'm', b'_', b's', b'p', b'a', b'n', 97, 0, 97, 0,
            97, 0, 106,
        ];
        let DecodeResult::Normal(decoded) = decode_tokens(&bytes, &spans, &psess).expect("decodes")
        else {
            panic!("all span ids are known");
        };
        let TokenTree::Delimited(_, _, token::Delimiter::Parenthesis, contents) =
            decoded.get(0).expect("one group")
        else {
            panic!("expected an empty parenthesis group, got {decoded:?}");
        };
        assert!(contents.is_empty());
    });
}

/// A multi-character operator is one punct per character on the wire, so this
/// covers the case where a token's wire length is not one.
#[test]
fn multi_char_operators_split_and_reglue() {
    create_default_session_globals_then(|| {
        for (kind, len) in
            [(token::ShlEq, 3), (token::Shl, 2), (token::DotDotDot, 3), (token::Le, 2)]
        {
            let sp = span(30, 30 + len);
            let stream = TokenStream::new(vec![TokenTree::token_alone(kind, sp)]);
            let decoded = encode_decode(&stream);
            assert_eq!(decoded.len(), 1, "{kind:?} did not reglue into one token");
            assert_eq_unspanned(&decoded, &stream);
        }
    });
}

/// Lifetimes are rejected until rustc's source has been cleaned up to avoid
/// passing them through the Patina proc-macro protocol.
#[test]
fn rejects_lifetime() {
    create_default_session_globals_then(|| {
        let stream = TokenStream::new(vec![TokenTree::token_alone(
            token::Lifetime(Symbol::intern("'a"), IdentIsRaw::No),
            span(40, 42),
        )]);
        let mut spans = table();
        let error = encode_tokens(&stream, &mut spans).expect_err("lifetimes are unsupported");
        assert!(matches!(error, TokenEncodeError::Lifetime { span: sp } if sp == span(40, 42)));
    });
}

/// The final punctuation character preserves the input token's jointness.
#[test]
fn preserves_joint_punctuation_spacing() {
    create_default_session_globals_then(|| {
        let stream = TokenStream::new(vec![
            TokenTree::token_joint(token::Plus, span(45, 46)),
            ident("rhs", span(46, 49)),
        ]);
        let decoded = encode_decode(&stream);
        let TokenTree::Token(token, spacing) = decoded.get(0).expect("punctuation token") else {
            panic!("expected punctuation, got {decoded:?}");
        };
        assert_eq!(token.kind, token::Plus);
        assert_eq!(*spacing, Spacing::Joint);
    });
}

/// Doc comments are deliberately not round-tripped: they desugar to a `doc`
/// attribute, as the old proc macro bridge did. An outer comment is two wire
/// trees and an inner one is three.
#[test]
fn doc_comment_desugars_to_attribute() {
    create_default_session_globals_then(|| {
        for (style, len) in [(AttrStyle::Outer, 2), (AttrStyle::Inner, 3)] {
            let stream = TokenStream::new(vec![TokenTree::token_alone(
                token::DocComment(CommentKind::Line, style, Symbol::intern(" hi \"there\"")),
                span(50, 60),
            )]);
            let decoded = encode_decode(&stream);
            assert_eq!(decoded.len(), len, "{style:?} doc comment");

            let expected = {
                let body = group(
                    token::Delimiter::Bracket,
                    vec![
                        ident("doc", span(50, 60)),
                        TokenTree::token_alone(token::Eq, span(50, 60)),
                        TokenTree::token_alone(
                            token::TokenKind::lit(
                                token::Str,
                                Symbol::intern(" hi \\\"there\\\""),
                                None,
                            ),
                            span(50, 60),
                        ),
                    ],
                    span(50, 60),
                );
                let mut trees = vec![TokenTree::token_alone(token::Pound, span(50, 60))];
                if style == AttrStyle::Inner {
                    trees.push(TokenTree::token_alone(token::Bang, span(50, 60)));
                }
                trees.push(body);
                TokenStream::new(trees)
            };
            assert_eq_unspanned(&decoded, &expected);
        }
    });
}

/// Spans survive as ids into the table, including ones the encoder had to
/// intern itself.
#[test]
fn spans_survive_the_round_trip() {
    create_default_session_globals_then(|| {
        let sp = span(100, 103);
        let stream = TokenStream::new(vec![ident("foo", sp)]);
        let decoded = encode_decode(&stream);
        let TokenTree::Token(token, _) = decoded.get(0).expect("one token") else {
            panic!("expected a token, got {decoded:?}");
        };
        assert_eq!(token.span, sp);
    });
}

/// Opening and closing delimiter spans are separate wire fields and must not
/// be collapsed to the group's entire span or swapped during decoding.
#[test]
fn delimiter_spans_survive_the_round_trip() {
    create_default_session_globals_then(|| {
        let open = span(110, 111);
        let close = span(114, 115);
        let stream = TokenStream::new(vec![TokenTree::Delimited(
            DelimSpan::from_pair(open, close),
            DelimSpacing::new(Spacing::Alone, Spacing::Alone),
            token::Delimiter::Parenthesis,
            TokenStream::new(vec![ident("body", span(111, 115))]),
        )]);

        let decoded = encode_decode(&stream);
        let TokenTree::Delimited(delim_span, ..) = decoded.get(0).expect("one group") else {
            panic!("expected a group, got {decoded:?}");
        };
        assert_eq!(delim_span.open, open);
        assert_eq!(delim_span.close, close);
    });
}

/// The encoder walks groups with an explicit stack, so nesting depth is
/// bounded by the heap rather than by the call stack.
#[test]
fn round_trips_deeply_nested_groups() {
    create_default_session_globals_then(|| {
        let mut tree = ident("core", span(200, 204));
        for depth in 0..256 {
            let sp = span(300 + depth, 301 + depth);
            tree = group(token::Delimiter::Parenthesis, vec![tree], sp);
        }
        let stream = TokenStream::new(vec![tree]);
        assert_eq_unspanned(&encode_decode(&stream), &stream);
    });
}

/// An invisible group is a `none` delimiter on the wire, and must survive:
/// flattening it would change the precedence of what it wraps.
#[test]
fn invisible_groups_survive() {
    create_default_session_globals_then(|| {
        let inner = vec![
            TokenTree::token_alone(
                token::TokenKind::lit(token::Integer, Symbol::intern("1"), None),
                span(60, 61),
            ),
            TokenTree::token_alone(token::Plus, span(61, 62)),
            TokenTree::token_alone(
                token::TokenKind::lit(token::Integer, Symbol::intern("2"), None),
                span(62, 63),
            ),
        ];
        let invisible = token::Delimiter::Invisible(token::InvisibleOrigin::ProcMacro);
        let stream = TokenStream::new(vec![group(invisible, inner, span(60, 63))]);

        let decoded = encode_decode(&stream);
        assert_eq!(decoded.len(), 1, "the group was flattened away");
        let TokenTree::Delimited(.., delim, contents) = decoded.get(0).expect("one tree") else {
            panic!("expected a group, got {decoded:?}");
        };
        assert!(matches!(delim, token::Delimiter::Invisible(_)), "delimiter became {delim:?}");
        assert_eq!(contents.len(), 3);
    });
}

/// Unknown span ids fall back to the call site and are reported, rather than
/// being rejected.
#[test]
fn unknown_span_id_falls_back_to_call_site() {
    create_default_session_globals_then(|| {
        let psess = ParseSess::new();
        let call_site = span(0, 1);
        let spans = SpanTable::new(call_site, span(1, 2), span(2, 3));

        // `{ident, <<"x">>, false, 9999}` in a one-element list.
        let mut bytes = vec![etf::TERM_HEADER];
        etf::write_list_header(&mut bytes, 1).unwrap();
        etf::write_tuple_header(&mut bytes, 4).unwrap();
        etf::write_atom(&mut bytes, "ident").unwrap();
        etf::write_binary(&mut bytes, b"x").unwrap();
        etf::write_atom(&mut bytes, "false").unwrap();
        etf::write_integer(&mut bytes, 9999i32).unwrap();
        etf::write_nil(&mut bytes).unwrap();

        let DecodeResult::HadUnknownSpan(decoded) =
            decode_tokens(&bytes, &spans, &psess).expect("decodes")
        else {
            panic!("unknown span was not reported");
        };
        let TokenTree::Token(token, _) = decoded.get(0).expect("one token") else {
            panic!("expected a token");
        };
        assert_eq!(token.span, call_site);
    });
}

#[test]
fn rejects_error_literal_without_reported_error() {
    create_default_session_globals_then(|| {
        let psess = ParseSess::new();
        let spans = table();

        let bytes = encoded_error_literal();

        let Err(error) = decode_tokens(&bytes, &spans, &psess) else {
            panic!("an `err` literal needs a real error guarantee");
        };
        assert_eq!(error, TokenDecodeError::UnreportedErrorLiteral);
        assert!(psess.dcx().has_errors().is_none());
    });
}

#[test]
fn error_literal_reuses_prior_error_guarantee() {
    create_default_session_globals_then(|| {
        let psess = ParseSess::new();
        psess.dcx().make_silent();
        let reported = psess.dcx().err("test error");
        let spans = table();

        let DecodeResult::Normal(decoded) =
            decode_tokens(&encoded_error_literal(), &spans, &psess).expect("decodes")
        else {
            panic!("all span ids are known");
        };
        let TokenTree::Token(token, _) = decoded.get(0).expect("one token") else {
            panic!("expected a token");
        };
        assert!(
            matches!(token.kind, token::Literal(token::Lit { kind: token::Err(guar), .. }) if guar == reported)
        );
    });
}

/// A wire term written by hand.
struct Wire(Vec<u8>);

impl Wire {
    fn new() -> Wire {
        Wire(vec![etf::TERM_HEADER])
    }

    fn list(mut self, len: usize) -> Wire {
        etf::write_list_header(&mut self.0, len).expect("write list header");
        self
    }

    fn nil(mut self) -> Wire {
        etf::write_nil(&mut self.0).expect("write nil");
        self
    }

    fn tuple(mut self, len: usize) -> Wire {
        etf::write_tuple_header(&mut self.0, len).expect("write tuple header");
        self
    }

    fn atom(mut self, atom: &str) -> Wire {
        etf::write_atom(&mut self.0, atom).expect("write atom");
        self
    }

    fn bin(mut self, text: &str) -> Wire {
        etf::write_binary(&mut self.0, text.as_bytes()).expect("write binary");
        self
    }

    fn int(mut self, value: i64) -> Wire {
        etf::write_integer(&mut self.0, value).expect("write integer");
        self
    }

    fn boolean(mut self, value: bool) -> Wire {
        etf::write_bool(&mut self.0, value).expect("write bool");
        self
    }

    /// Raw bytes, for a term the builders above cannot express.
    fn raw(mut self, bytes: &[u8]) -> Wire {
        self.0.extend_from_slice(bytes);
        self
    }

    /// A one-element stream wrapping `build`.
    fn one(build: impl FnOnce(Wire) -> Wire) -> Wire {
        build(Wire::new().list(1)).nil()
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }
}

/// The span ids [`table`] pins, in the order [`SpanTable::new`] takes them.
const CALL_SITE_ID: i64 = 0;
const DEF_SITE_ID: i64 = 1;
const MIXED_SITE_ID: i64 = 2;

fn decode_fixture(wire: &Wire) -> Result<DecodeResult, TokenDecodeError> {
    let psess = ParseSess::new();
    decode_tokens(wire.bytes(), &table(), &psess)
}

/// Decodes a fixture that is expected to be valid and fully spanned.
fn decode_ok(wire: &Wire) -> TokenStream {
    match decode_fixture(wire).expect("fixture decodes") {
        DecodeResult::Normal(stream) => stream,
        DecodeResult::HadUnknownSpan(_) => panic!("fixture uses only well-known span ids"),
    }
}

/// Decodes a fixture that is expected to be rejected.
#[track_caller]
fn decode_err(wire: &Wire) -> TokenDecodeError {
    match decode_fixture(wire) {
        Err(error) => error,
        Ok(_) => panic!("expected the fixture to be rejected"),
    }
}

#[track_caller]
fn assert_protocol_error(error: TokenDecodeError, expected: &str) {
    match error {
        TokenDecodeError::Protocol { ref details, .. } if details.contains(expected) => {}
        other => panic!("expected a protocol error mentioning {expected:?}, got {other:?}"),
    }
}

#[test]
fn fixture_decodes_to_the_documented_tokens() {
    create_default_session_globals_then(|| {
        // `[{ident,<<"foo">>,false,1},{punct,43,false,0},{literal,{integer},<<"42">>,nil,2}]`
        let wire = Wire::new()
            .list(3)
            .tuple(4)
            .atom("ident")
            .bin("foo")
            .boolean(false)
            .int(DEF_SITE_ID)
            .tuple(4)
            .atom("punct")
            .int(i64::from(b'+'))
            .boolean(false)
            .int(CALL_SITE_ID)
            .tuple(5)
            .atom("literal")
            .tuple(1)
            .atom("integer")
            .bin("42")
            .atom("nil")
            .int(MIXED_SITE_ID)
            .nil();

        let decoded = decode_ok(&wire);
        let expected = TokenStream::new(vec![
            ident("foo", span(1, 2)),
            TokenTree::token_alone(token::Plus, span(0, 1)),
            TokenTree::token_alone(
                token::TokenKind::lit(token::Integer, Symbol::intern("42"), None),
                span(2, 3),
            ),
        ]);
        assert_eq_unspanned(&decoded, &expected);

        // The spans, which `assert_eq_unspanned` does not compare: each token
        // must come back on the well-known span its id named.
        let spans: Vec<_> = decoded
            .iter()
            .map(|tree| match tree {
                TokenTree::Token(token, _) => token.span,
                TokenTree::Delimited(delim_span, ..) => delim_span.open,
            })
            .collect();
        assert_eq!(spans, vec![span(1, 2), span(0, 1), span(2, 3)]);
    });
}

#[test]
fn encodes_a_stream_to_the_documented_bytes() {
    create_default_session_globals_then(|| {
        let stream = TokenStream::new(vec![ident("foo", span(0, 1))]);
        let mut spans = table();
        let actual = encode_tokens(&stream, &mut spans).expect("encodes");

        // `span(0, 1)` is the call site, which `SpanTable::new` already pinned
        // to id 0, so nothing new is interned.
        let expected =
            Wire::one(|w| w.tuple(4).atom("ident").bin("foo").boolean(false).int(CALL_SITE_ID));
        assert_eq!(
            actual,
            expected.bytes(),
            "encoder output does not match the protocol written out by hand"
        );
    });
}

#[test]
fn rejects_unknown_tree_tag() {
    create_default_session_globals_then(|| {
        let wire =
            Wire::one(|w| w.tuple(4).atom("frobnicate").bin("x").boolean(false).int(CALL_SITE_ID));
        assert_protocol_error(decode_err(&wire), "unknown token tree tag frobnicate");
    });
}

#[test]
fn rejects_known_tag_with_wrong_arity() {
    create_default_session_globals_then(|| {
        // `ident` takes four fields; three is not a different token, it is a
        // malformed one.
        let wire = Wire::one(|w| w.tuple(3).atom("ident").bin("foo").boolean(false));
        assert_protocol_error(decode_err(&wire), "unknown token tree tag ident with arity 3");
    });
}

#[test]
fn rejects_unknown_delimiter() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| w.tuple(4).atom("group").atom("chevron"));
        assert_protocol_error(decode_err(&wire), "unknown delimiter chevron");
    });
}

#[test]
fn rejects_non_boolean_punctuation_jointness() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(4).atom("punct").int(i64::from(b'+')).atom("joint").int(CALL_SITE_ID)
        });
        assert_protocol_error(decode_err(&wire), "expected boolean atom, found `joint`");
    });
}

/// `nil` is `Option::None`: an empty stream, at the top level and in a group.
#[test]
fn decodes_nil_as_an_empty_stream() {
    create_default_session_globals_then(|| {
        assert!(decode_ok(&Wire::new().atom("nil")).is_empty());

        let wire = Wire::one(|w| {
            w.tuple(4)
                .atom("group")
                .atom("brace")
                .atom("nil")
                .tuple(4)
                .atom("delim_span")
                .int(CALL_SITE_ID)
                .int(CALL_SITE_ID)
                .int(CALL_SITE_ID)
        });
        let decoded = decode_ok(&wire);
        let TokenTree::Delimited(_, _, token::Delimiter::Brace, contents) =
            decoded.get(0).expect("one group")
        else {
            panic!("expected an empty brace group, got {decoded:?}");
        };
        assert!(contents.is_empty());
    });
}

#[test]
fn encodes_empty_streams_as_nil() {
    create_default_session_globals_then(|| {
        let empty = encode_tokens(&TokenStream::default(), &mut table()).expect("encodes");
        assert_eq!(empty, Wire::new().atom("nil").bytes());

        let group = TokenStream::new(vec![group(token::Delimiter::Brace, vec![], span(0, 1))]);
        let actual = encode_tokens(&group, &mut table()).expect("encodes");
        let expected = Wire::one(|w| {
            w.tuple(4)
                .atom("group")
                .atom("brace")
                .atom("nil")
                .tuple(4)
                .atom("delim_span")
                .int(CALL_SITE_ID)
                .int(CALL_SITE_ID)
                .int(CALL_SITE_ID)
        });
        assert_eq!(actual, expected.bytes());
    });
}

#[test]
fn rejects_group_without_delim_span() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(4)
                .atom("group")
                .atom("brace")
                .atom("nil")
                .tuple(3)
                .int(CALL_SITE_ID)
                .int(CALL_SITE_ID)
                .int(CALL_SITE_ID)
        });
        assert!(matches!(decode_err(&wire), TokenDecodeError::Protocol { .. }));
    });
}

#[test]
fn rejects_non_nil_atom_as_suffix() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(5)
                .atom("literal")
                .tuple(1)
                .atom("integer")
                .bin("1")
                .atom("none")
                .int(CALL_SITE_ID)
        });
        assert_protocol_error(decode_err(&wire), "expected `nil`, found `none`");
    });
}

#[test]
fn rejects_nonempty_string_as_stream() {
    create_default_session_globals_then(|| {
        // `STRING_EXT` of two bytes. ETF encodes a list of small integers this
        // way, and it is not a token stream.
        let psess = ParseSess::new();
        let bytes = [131, 107, 0, 2, 1, 2];
        let error = decode_tokens(&bytes, &table(), &psess).expect_err("rejected");
        assert_protocol_error(error, "nonempty string is not a token stream");
    });
}

#[test]
fn rejects_fieldless_literal_kind_carrying_fields() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(5)
                .atom("literal")
                .tuple(2)
                .atom("integer")
                .int(1)
                .bin("42")
                .atom("nil")
                .int(CALL_SITE_ID)
        });
        assert_protocol_error(decode_err(&wire), "literal kind must be fieldless");
    });
}

#[test]
fn rejects_raw_literal_kind_without_hash_count() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(5)
                .atom("literal")
                .tuple(1)
                .atom("str_raw")
                .bin("x")
                .atom("nil")
                .int(CALL_SITE_ID)
        });
        assert_protocol_error(decode_err(&wire), "raw literal kind must carry a hash count");
    });
}

#[test]
fn rejects_unknown_literal_kind() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(5)
                .atom("literal")
                .tuple(1)
                .atom("rational")
                .bin("1/2")
                .atom("nil")
                .int(CALL_SITE_ID)
        });
        assert_protocol_error(decode_err(&wire), "unknown literal kind rational");
    });
}

#[test]
fn rejects_raw_literal_hash_count_out_of_range() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(5)
                .atom("literal")
                .tuple(2)
                .atom("str_raw")
                .int(9001)
                .bin("x")
                .atom("nil")
                .int(CALL_SITE_ID)
        });
        assert_eq!(
            decode_err(&wire),
            TokenDecodeError::RawLiteralHashCountOutOfRange { kind: "str_raw", count: 9001 }
        );
    });
}

#[test]
fn rejects_invalid_identifier() {
    create_default_session_globals_then(|| {
        let wire =
            Wire::one(|w| w.tuple(4).atom("ident").bin("1abc").boolean(false).int(CALL_SITE_ID));
        assert_eq!(
            decode_err(&wire),
            TokenDecodeError::InvalidIdentifier { name: "1abc".to_string(), is_raw: false }
        );
    });
}

#[test]
fn rejects_unsupported_punctuation() {
    create_default_session_globals_then(|| {
        // A space is a valid byte but not punctuation.
        let wire = Wire::one(|w| {
            w.tuple(4).atom("punct").int(i64::from(b' ')).boolean(false).int(CALL_SITE_ID)
        });
        assert_eq!(
            decode_err(&wire),
            TokenDecodeError::UnsupportedPunctuation { value: i64::from(b' ') }
        );
    });
}

#[test]
fn rejects_punctuation_that_is_not_a_byte() {
    create_default_session_globals_then(|| {
        let wire =
            Wire::one(|w| w.tuple(4).atom("punct").int(70_000).boolean(false).int(CALL_SITE_ID));
        assert_eq!(decode_err(&wire), TokenDecodeError::UnsupportedPunctuation { value: 70_000 });
    });
}

/// A lifetime in a macro's *output* must be reported the same way as one in its
/// input: pointing at the token, not at the whole invocation.
#[test]
fn rejects_lifetime_in_output_with_the_offending_span() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(4).atom("punct").int(i64::from(b'\'')).boolean(false).int(DEF_SITE_ID)
        });
        assert_eq!(decode_err(&wire), TokenDecodeError::Lifetime { span: span(1, 2) });
    });
}

#[test]
fn dollar_crate_decodes_to_the_keyword() {
    create_default_session_globals_then(|| {
        let wire =
            Wire::one(|w| w.tuple(4).atom("ident").bin("$pkg").boolean(false).int(CALL_SITE_ID));
        let decoded = decode_ok(&wire);
        let expected = TokenStream::new(vec![TokenTree::token_alone(
            token::Ident(rustc_span::kw::DollarPkg, token::IdentIsRaw::No),
            span(0, 1),
        )]);
        assert_eq_unspanned(&decoded, &expected);
    });
}

#[test]
fn round_trips_bare_and_escaped_pkg() {
    create_default_session_globals_then(|| {
        let stream = TokenStream::new(vec![
            TokenTree::token_alone(token::Ident(rustc_span::kw::Pkg, IdentIsRaw::No), span(4, 5)),
            TokenTree::token_alone(
                token::Ident(Symbol::intern("`pkg`"), IdentIsRaw::No),
                span(6, 7),
            ),
            TokenTree::token_alone(
                token::Ident(Symbol::intern("`self`"), IdentIsRaw::No),
                span(8, 9),
            ),
        ]);
        assert_eq_unspanned(&encode_decode(&stream), &stream);
    });
}

/// A raw `$pkg` is an ordinary identifier, not the keyword.
#[test]
fn rejects_raw_dollar_pkg() {
    create_default_session_globals_then(|| {
        let wire =
            Wire::one(|w| w.tuple(4).atom("ident").bin("$pkg").boolean(true).int(CALL_SITE_ID));
        assert_eq!(
            decode_err(&wire),
            TokenDecodeError::InvalidIdentifier { name: "$pkg".to_string(), is_raw: true }
        );
    });
}

#[test]
fn negative_literal_splits_into_minus_and_literal() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(5)
                .atom("literal")
                .tuple(1)
                .atom("integer")
                .bin("-5")
                .atom("nil")
                .int(CALL_SITE_ID)
        });
        let decoded = decode_ok(&wire);
        let expected = TokenStream::new(vec![
            TokenTree::token_joint_hidden(token::Minus, span(0, 1)),
            TokenTree::token_alone(
                token::TokenKind::lit(token::Integer, Symbol::intern("5"), None),
                span(0, 1),
            ),
        ]);
        assert_eq_unspanned(&decoded, &expected);
    });
}

/// The element count on the wire is load bearing: a stream that claims fewer
/// elements than it carries leaves bytes the decoder has to reject.
#[test]
fn rejects_trailing_bytes_after_the_stream() {
    create_default_session_globals_then(|| {
        let wire = Wire::new()
            .list(1)
            .tuple(4)
            .atom("ident")
            .bin("a")
            .boolean(false)
            .int(CALL_SITE_ID)
            .tuple(4)
            .atom("ident")
            .bin("b")
            .boolean(false)
            .int(CALL_SITE_ID)
            .nil();
        // The second tuple is read as the list tail, which is not a tail.
        assert!(matches!(decode_err(&wire), TokenDecodeError::Protocol { .. }));
    });
}

/// An attribute macro is called with two streams encoded through one span
/// table. Ids handed out while encoding the first argument must stay valid, and
/// mean the same thing, while encoding the second.
#[test]
fn span_table_is_shared_across_both_arguments() {
    create_default_session_globals_then(|| {
        let mut spans = table();
        let shared = span(30, 34);
        let only_in_second = span(40, 44);

        let annotation = TokenStream::new(vec![ident("attr", shared)]);
        let annotated =
            TokenStream::new(vec![ident("item", shared), ident("more", only_in_second)]);
        encode_tokens(&annotation, &mut spans).expect("encodes the annotation");
        encode_tokens(&annotated, &mut spans).expect("encodes the annotated item");

        // Ids 0..=2 are the well-known spans, so the first span interned by
        // either argument is 3. `shared` appears in both arguments and must
        // have been interned once.
        let by_id = |id: i64| {
            let wire = Wire::one(|w| w.tuple(4).atom("ident").bin("x").boolean(false).int(id));
            let psess = ParseSess::new();
            match decode_tokens(wire.bytes(), &spans, &psess).expect("decodes") {
                DecodeResult::Normal(stream) => match stream.iter().next().expect("one token") {
                    TokenTree::Token(token, _) => token.span,
                    TokenTree::Delimited(delim_span, ..) => delim_span.open,
                },
                DecodeResult::HadUnknownSpan(_) => panic!("id {id} was never interned"),
            }
        };
        assert_eq!(by_id(3), shared);
        assert_eq!(by_id(4), only_in_second);
    });
}

/// An invisible metavariable group around `inner`.
fn meta_group(kind: token::MetaVarKind, sp: Span, inner: Vec<TokenTree>) -> TokenTree {
    TokenTree::Delimited(
        DelimSpan::from_single(sp),
        DelimSpacing::new(Spacing::Alone, Spacing::Alone),
        token::Delimiter::Invisible(token::InvisibleOrigin::MetaVar(kind)),
        TokenStream::new(inner),
    )
}

fn round_trip(stream: TokenStream) -> TokenStream {
    let decoded = encode_decode(&stream);
    assert_eq_unspanned(&decoded, &stream);
    decoded
}

/// A metavariable group whose only child is another metavariable group carries
/// no information the inner one does not, so the encoder drops the outer layer.
/// The surviving group must keep the *outer* spans.
#[test]
fn collapses_nested_single_child_metavar_groups() {
    create_default_session_globals_then(|| {
        let inner =
            meta_group(token::MetaVarKind::Block, span(20, 24), vec![ident("x", span(21, 22))]);
        // A different kind on each layer: collapsing must not depend on them
        // matching.
        let outer = meta_group(token::MetaVarKind::Item, span(10, 14), vec![inner]);

        let decoded = encode_decode(&TokenStream::new(vec![outer]));
        let TokenTree::Delimited(delim_span, _, _, contents) =
            decoded.iter().next().expect("one tree")
        else {
            panic!("expected a group, got {decoded:?}");
        };
        assert_eq!((delim_span.open, delim_span.close), (span(10, 14), span(10, 14)));
        assert_eq!(contents.len(), 1);
        assert!(matches!(contents.iter().next(), Some(TokenTree::Token(..))));
    });
}

/// Only metavariable groups collapse. A proc-macro-origin group is something
/// the macro asked for, so it survives even wrapping a single child.
#[test]
fn does_not_collapse_procmacro_origin_groups() {
    create_default_session_globals_then(|| {
        let inner =
            meta_group(token::MetaVarKind::Block, span(20, 24), vec![ident("x", span(21, 22))]);
        let outer = TokenTree::Delimited(
            DelimSpan::from_single(span(10, 14)),
            DelimSpacing::new(Spacing::Alone, Spacing::Alone),
            token::Delimiter::Invisible(token::InvisibleOrigin::ProcMacro),
            TokenStream::new(vec![inner]),
        );

        let decoded = encode_decode(&TokenStream::new(vec![outer]));
        let TokenTree::Delimited(_, _, _, contents) = decoded.iter().next().expect("one tree")
        else {
            panic!("expected a group, got {decoded:?}");
        };
        assert!(matches!(contents.iter().next(), Some(TokenTree::Delimited(..))));
    });
}

#[test]
fn does_not_collapse_multi_child_metavar_groups() {
    create_default_session_globals_then(|| {
        let nested =
            meta_group(token::MetaVarKind::Block, span(20, 24), vec![ident("x", span(21, 22))]);
        let outer = meta_group(
            token::MetaVarKind::Item,
            span(10, 14),
            vec![nested, ident("y", span(25, 26))],
        );

        let decoded = encode_decode(&TokenStream::new(vec![outer]));
        let TokenTree::Delimited(_, _, _, contents) = decoded.iter().next().expect("one tree")
        else {
            panic!("expected a group, got {decoded:?}");
        };
        assert_eq!(contents.len(), 2);
    });
}

#[test]
fn round_trips_every_delimiter() {
    create_default_session_globals_then(|| {
        for delimiter in [
            token::Delimiter::Parenthesis,
            token::Delimiter::Brace,
            token::Delimiter::Bracket,
            token::Delimiter::Invisible(token::InvisibleOrigin::ProcMacro),
        ] {
            let stream = TokenStream::new(vec![group(
                delimiter,
                vec![ident("x", span(11, 12))],
                span(10, 13),
            )]);
            let decoded = round_trip(stream);
            let TokenTree::Delimited(_, _, decoded_delimiter, _) =
                decoded.iter().next().expect("one tree")
            else {
                panic!("expected a group, got {decoded:?}");
            };
            assert_eq!(*decoded_delimiter, delimiter, "delimiter changed on the wire");
        }
    });
}

#[test]
fn round_trips_every_literal_kind() {
    create_default_session_globals_then(|| {
        for kind in [
            token::Byte,
            token::Char,
            token::Integer,
            token::Float,
            token::Str,
            token::StrRaw(2),
            token::AtomStr,
            token::AtomStrRaw(0),
            token::ByteStr,
            token::ByteStrRaw(255),
            token::CStr,
            token::CStrRaw(1),
        ] {
            let stream = TokenStream::new(vec![TokenTree::token_alone(
                token::TokenKind::lit(kind, Symbol::intern("v"), None),
                span(10, 11),
            )]);
            round_trip(stream);
        }
    });
}

/// Two list segments forming one proper stream: `[x] ++ [y]`. Erlang is free to
/// split a list this way, so the decoder has to follow the tail.
#[test]
fn accepts_proper_list_continuation_tails() {
    create_default_session_globals_then(|| {
        let wire = Wire::new()
            .list(1)
            .tuple(4)
            .atom("ident")
            .bin("x")
            .boolean(false)
            .int(CALL_SITE_ID)
            .list(1)
            .tuple(4)
            .atom("ident")
            .bin("y")
            .boolean(false)
            .int(CALL_SITE_ID)
            .nil();
        let decoded = decode_ok(&wire);
        let expected = TokenStream::new(vec![ident("x", span(0, 1)), ident("y", span(0, 1))]);
        assert_eq_unspanned(&decoded, &expected);
    });
}

#[test]
fn rejects_improper_list_tail() {
    create_default_session_globals_then(|| {
        // A list header claiming no elements, followed by an integer where nil
        // belongs.
        let wire = Wire::new().raw(&[etf::LIST_EXT, 0, 0, 0, 0]).int(7);
        assert!(matches!(decode_err(&wire), TokenDecodeError::Protocol { .. }));
    });
}

/// A span id that does not fit in an `i64` is malformed, not merely unknown.
#[test]
fn rejects_bignum_span_id() {
    create_default_session_globals_then(|| {
        // `SMALL_BIG_EXT` encoding of 2^64.
        let bignum = [etf::SMALL_BIG_EXT, 9, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let wire = Wire::one(|w| w.tuple(4).atom("ident").bin("x").boolean(false).raw(&bignum));
        assert!(matches!(decode_err(&wire), TokenDecodeError::Protocol { .. }));
    });
}

/// A fieldless literal kind is a one-element tagged tuple, not a bare atom.
/// Serde's unit-variant encoding used to produce the latter.
#[test]
fn rejects_bare_atom_literal_kind() {
    create_default_session_globals_then(|| {
        let wire = Wire::one(|w| {
            w.tuple(5).atom("literal").atom("str").bin("s").atom("nil").int(CALL_SITE_ID)
        });
        assert!(matches!(decode_err(&wire), TokenDecodeError::Protocol { .. }));
    });
}

/// Out-of-range ids fall back to the call site rather than failing, because a
/// macro may legitimately mint spans rustc never handed it.
#[test]
fn negative_and_absent_span_ids_fall_back_to_call_site() {
    create_default_session_globals_then(|| {
        for id in [-1, 9999] {
            let wire = Wire::one(|w| w.tuple(4).atom("ident").bin("x").boolean(false).int(id));
            let psess = ParseSess::new();
            let stream = match decode_tokens(wire.bytes(), &table(), &psess).expect("decodes") {
                DecodeResult::HadUnknownSpan(stream) => stream,
                DecodeResult::Normal(_) => panic!("span id {id} should not be known"),
            };
            let TokenTree::Token(token, _) = stream.iter().next().expect("one token") else {
                panic!("expected a token");
            };
            assert_eq!(token.span, span(0, 1), "id {id} should resolve to the call site");
        }
    });
}

#[test]
fn reserved_span_ids_resolve_to_the_well_known_sites() {
    create_default_session_globals_then(|| {
        let spans = table();
        assert_eq!(spans.resolve(CALL_SITE_ID), ResolvedSpan::Found(span(0, 1)));
        assert_eq!(spans.resolve(DEF_SITE_ID), ResolvedSpan::Found(span(1, 2)));
        assert_eq!(spans.resolve(MIXED_SITE_ID), ResolvedSpan::Found(span(2, 3)));
        assert_eq!(spans.resolve(3), ResolvedSpan::Unknown(span(0, 1)));
    });
}

/// The decoder is iterative, so deep input must be rejected cleanly rather than
/// overflowing the stack.
#[test]
fn deeply_nested_truncated_input_is_rejected() {
    create_default_session_globals_then(|| {
        let mut wire = Wire::new();
        for _ in 0..512 {
            wire = wire.list(1).tuple(4).atom("group").atom("parenthesis");
        }
        // The innermost stream, and every group's trailing spans, are missing.
        assert!(matches!(decode_err(&wire), TokenDecodeError::Protocol { .. }));
    });
}
