//! Direct ETF encoding and decoding for the Patina proc-macro token protocol.
//!
//! The protocol is:
//!
//!     TokenStream = nil | [TokenTree]
//!     TokenTree   = {group, Delimiter, TokenStream, DelimSpan}
//!                 | {ident, Binary, IsRaw :: boolean(), Span}
//!                 | {punct, Integer, Joint :: boolean(), Span}
//!                 | {literal, LitKind, Binary, nil | Binary, Span}
//!     DelimSpan   = {delim_span, Open :: Span, Close :: Span, Entire :: Span}
//!     LitKind     = {byte} | {char} | ... | {str_raw, HashCount} | ...
//!
//! Spans are integer ids into a per-invocation [`SpanTable`].

use std::fmt;

use etf::{
    TERM_HEADER, write_atom, write_binary, write_bool, write_integer, write_list_header, write_nil,
    write_tuple_header,
};
use rustc_ast as ast;
use rustc_ast::token::{self, IdentIsRaw, InvisibleOrigin, TokenKind};
use rustc_ast::tokenstream::{
    self, DelimSpacing, DelimSpan, Spacing, TokenStream, TokenTreeCursor,
};
use rustc_data_structures::fx::FxIndexSet;
use rustc_parse::lexer::nfc_normalize;
use rustc_session::parse::ParseSess;
use rustc_span::{BytePos, ErrorGuaranteed, Pos, Span, Symbol, kw};
use smallvec::{SmallVec, smallvec};

/// A value accepted by the BEAM API but rejected by rustc's native
/// `proc_macro` constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenDecodeError {
    InvalidIdentifier { name: String, is_raw: bool },
    UnsupportedPunctuation { value: i64 },
    RawLiteralHashCountOutOfRange { kind: &'static str, count: i64 },
    UnreportedErrorLiteral,
    Lifetime { span: Span },
    Protocol { offset: usize, details: String },
}

impl fmt::Display for TokenDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenDecodeError::InvalidIdentifier { name, is_raw: false } => {
                write!(f, "`{name:?}` is not a valid identifier")
            }
            TokenDecodeError::InvalidIdentifier { name, is_raw: true } => {
                write!(f, "`{name}` cannot be a raw identifier")
            }
            TokenDecodeError::UnsupportedPunctuation { value } => {
                write!(f, "unsupported punctuation character with code point {value}")
            }
            TokenDecodeError::RawLiteralHashCountOutOfRange { kind, count } => {
                write!(f, "`{kind}` hash count is out of range: {count}")
            }
            TokenDecodeError::UnreportedErrorLiteral => {
                write!(f, "`err` literal kind requires a previously reported error")
            }
            TokenDecodeError::Lifetime { span: _ } => {
                write!(f, "`'` is not valid punctuation: Patina has no lifetimes")
            }
            TokenDecodeError::Protocol { offset, details } => {
                write!(f, "invalid token protocol at byte {offset}: {details}")
            }
        }
    }
}

impl From<etf::Error> for TokenDecodeError {
    fn from(error: etf::Error) -> TokenDecodeError {
        TokenDecodeError::Protocol {
            offset: error.offset().unwrap_or(0),
            details: error.to_string(),
        }
    }
}

/// A rustc token that Patina's wire protocol cannot represent.
#[derive(Debug, Clone)]
pub enum TokenEncodeError {
    /// Lifetimes are not yet cleaned up in rustc's source code.
    Lifetime { span: Span },
    /// Writing the ETF term failed.
    Etf { details: String },
}

impl fmt::Display for TokenEncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenEncodeError::Lifetime { .. } => write!(f, "lifetimes are not supported"),
            TokenEncodeError::Etf { details } => write!(f, "{details}"),
        }
    }
}

impl From<etf::Error> for TokenEncodeError {
    fn from(error: etf::Error) -> TokenEncodeError {
        TokenEncodeError::Etf { details: error.to_string() }
    }
}

/// Side table mapping integer ids to real spans. Ids `0`, `1` and `2` are the
/// `call_site`, `def_site` and `mixed_site` spans respectively.
pub struct SpanTable(FxIndexSet<Span>);

#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedSpan {
    Found(Span),
    Unknown(Span),
}

impl SpanTable {
    pub fn new(call_site: Span, def_site: Span, mixed_site: Span) -> SpanTable {
        let spans = FxIndexSet::from_iter([call_site, def_site, mixed_site]);
        // Just sanity-checking that the spans are distinct.
        debug_assert_eq!(spans.len(), 3, "the well-known spans must take ids 0, 1 and 2");
        SpanTable(spans)
    }

    fn intern(&mut self, span: Span) -> i32 {
        let (index, _) = self.0.insert_full(span);
        index.try_into().expect("a span id fits in an `i32`")
    }

    /// Resolves a wire id, reporting whether it was found in the table.
    pub fn resolve(&self, id: i64) -> ResolvedSpan {
        if let Ok(index) = usize::try_from(id)
            && let Some(&span) = self.0.get_index(index)
        {
            ResolvedSpan::Found(span)
        } else {
            ResolvedSpan::Unknown(self.0[0])
        }
    }
}

// ---------------------------------------------------------------------------
// Encoding
// ---------------------------------------------------------------------------

/// Converts a rustc token stream to ETF, interning spans into `spans`.
pub fn encode_tokens(
    stream: &TokenStream,
    spans: &mut SpanTable,
) -> Result<Vec<u8>, TokenEncodeError> {
    let mut encoder = Encoder { spans, output: vec![TERM_HEADER], frames: Vec::new() };
    encoder.open_stream(stream, None)?;
    encoder.run()?;
    Ok(encoder.output)
}

enum Wire {
    Op { op: &'static str, span: Span },
    Ident { name: Symbol, is_raw: IdentIsRaw, span: Span },
    Literal { lit: token::Lit, span: Span },
    DocComment { style: ast::AttrStyle, data: Symbol, span: Span },
}

impl Wire {
    /// How many wire trees [`Encoder::encode_token`] writes for this token.
    fn trees(&self) -> usize {
        match self {
            Wire::Op { op, .. } => op.len(),
            Wire::DocComment { style: ast::AttrStyle::Inner, .. } => 3,
            Wire::DocComment { style: ast::AttrStyle::Outer, .. } => 2,
            Wire::Ident { .. } | Wire::Literal { .. } => 1,
        }
    }
}

impl TryFrom<token::Token> for Wire {
    type Error = TokenEncodeError;

    /// Classifies a token by how it is written to the wire.
    fn try_from(token: token::Token) -> Result<Wire, TokenEncodeError> {
        use rustc_ast::token::*;

        let token::Token { kind, span } = token;
        let op = match kind {
            Ident(name, is_raw) => return Ok(Wire::Ident { name, is_raw, span }),
            NtIdent(ident, is_raw) => {
                return Ok(Wire::Ident { name: ident.name, is_raw, span: ident.span });
            }
            Literal(lit) => return Ok(Wire::Literal { lit, span }),
            DocComment(_, style, data) => return Ok(Wire::DocComment { style, data, span }),

            SingleQuote | Lifetime(..) => return Result::Err(TokenEncodeError::Lifetime { span }),
            NtLifetime(ident, _) => {
                return Result::Err(TokenEncodeError::Lifetime { span: ident.span });
            }

            Eq => "=",
            Lt => "<",
            Le => "<=",
            EqEq => "==",
            Ne => "!=",
            Ge => ">=",
            Gt => ">",
            AndAnd => "&&",
            OrOr => "||",
            Bang => "!",
            Tilde => "~",
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Percent => "%",
            Caret => "^",
            And => "&",
            Or => "|",
            Shl => "<<",
            Shr => ">>",
            PlusEq => "+=",
            MinusEq => "-=",
            StarEq => "*=",
            SlashEq => "/=",
            PercentEq => "%=",
            CaretEq => "^=",
            AndEq => "&=",
            OrEq => "|=",
            ShlEq => "<<=",
            ShrEq => ">>=",
            At => "@",
            Dot => ".",
            DotDot => "..",
            DotDotDot => "...",
            DotDotEq => "..=",
            Comma => ",",
            Semi => ";",
            Colon => ":",
            PathSep => "::",
            RArrow => "->",
            LArrow => "<-",
            FatArrow => "=>",
            Pound => "#",
            Dollar => "$",
            Question => "?",

            OpenParen | CloseParen | OpenBrace | CloseBrace | OpenBracket | CloseBracket
            | OpenInvisible(_) | CloseInvisible(_) | Eof => {
                unreachable!("unexpected token kind in proc macro input")
            }
        };
        Ok(Wire::Op { op, span })
    }
}

fn punct_kind(ch: u8) -> Option<token::TokenKind> {
    use rustc_ast::token::*;
    // Only single characters appear here: the encoder splits a multi-character
    // operator into one punct per character, and `TokenStream::push_tree` glues
    // them back together.
    Some(match ch {
        b'=' => Eq,
        b'<' => Lt,
        b'>' => Gt,
        b'!' => Bang,
        b'~' => Tilde,
        b'+' => Plus,
        b'-' => Minus,
        b'*' => Star,
        b'/' => Slash,
        b'%' => Percent,
        b'^' => Caret,
        b'&' => And,
        b'|' => Or,
        b'@' => At,
        b'.' => Dot,
        b',' => Comma,
        b';' => Semi,
        b':' => Colon,
        b'#' => Pound,
        b'$' => Dollar,
        b'?' => Question,
        _ => return None,
    })
}

fn count_wire_trees(tree: &tokenstream::TokenTree) -> Result<usize, TokenEncodeError> {
    match tree {
        tokenstream::TokenTree::Delimited(..) => Ok(1),
        tokenstream::TokenTree::Token(token, _spacing) => Ok(Wire::try_from(*token)?.trees()),
    }
}

struct StackFrame {
    /// The trees left to encode.
    trees: TokenTreeCursor,
    /// The delimiter spans to write after the elements, if this stream is a
    /// group's contents.
    delim: Option<DelimSpan>,
}

struct Encoder<'a> {
    spans: &'a mut SpanTable,
    output: Vec<u8>,
    frames: Vec<StackFrame>,
}

impl Encoder<'_> {
    /// Encodes the open streams depth first until all of them are closed.
    fn run(&mut self) -> Result<(), TokenEncodeError> {
        loop {
            let Some(frame) = self.frames.last_mut() else { return Ok(()) };
            if let Some(tree) = frame.trees.curr().cloned() {
                frame.trees.bump();
                self.encode_tree(&tree)?;
            } else {
                let frame = self.frames.pop().expect("a frame is open");
                self.close_stream(frame)?;
            };
        }
    }

    /// Starts encoding a stream as a proper list, or as `nil` when it is empty.
    fn open_stream(
        &mut self,
        stream: &TokenStream,
        delim: Option<DelimSpan>,
    ) -> Result<(), TokenEncodeError> {
        if stream.is_empty() {
            write_atom(&mut self.output, "nil")?;
            return self.write_delim_spans(delim);
        }
        let trees = stream.iter().map(count_wire_trees).sum::<Result<usize, _>>()?;
        write_list_header(&mut self.output, trees)?;
        self.frames.push(StackFrame { trees: TokenTreeCursor::new(stream.clone()), delim });
        Ok(())
    }

    /// Terminates a fully encoded stream and writes its delimiter spans.
    fn close_stream(&mut self, frame: StackFrame) -> Result<(), TokenEncodeError> {
        write_nil(&mut self.output)?;
        self.write_delim_spans(frame.delim)
    }

    /// Writes the head of a `{group, Delimiter, TokenStream, DelimSpan}` tuple.
    /// The remaining two elements are the contents, written by
    /// [`Self::open_stream`], and the spans, written by
    /// [`Self::write_delim_spans`] once those contents are complete.
    fn write_group_header(&mut self, delimiter: &str) -> Result<(), TokenEncodeError> {
        write_tuple_header(&mut self.output, 4)?;
        write_atom(&mut self.output, "group")?;
        write_atom(&mut self.output, delimiter)?;
        Ok(())
    }

    /// Writes the `{delim_span, Open, Close, Entire}` that follows a group's
    /// contents on the wire.
    fn write_delim_spans(&mut self, delim: Option<DelimSpan>) -> Result<(), TokenEncodeError> {
        let Some(delim) = delim else { return Ok(()) };
        write_tuple_header(&mut self.output, 4)?;
        write_atom(&mut self.output, "delim_span")?;
        self.write_span(delim.open)?;
        self.write_span(delim.close)?;
        self.write_span(delim.entire())
    }

    fn write_span(&mut self, span: Span) -> Result<(), TokenEncodeError> {
        let id = self.spans.intern(span);
        write_integer(&mut self.output, id)?;
        Ok(())
    }

    fn encode_tree(&mut self, tree: &tokenstream::TokenTree) -> Result<(), TokenEncodeError> {
        match tree {
            tokenstream::TokenTree::Delimited(span, _spacing, delimiter, inner) => {
                // In `mk_delimited` we avoid nesting invisible delimited
                // of the same `MetaVarKind`. Here we do the same but
                // ignore the `MetaVarKind` because it is discarded when we
                // convert it to a `Group`.
                let mut delimiter = *delimiter;
                let mut inner = inner;
                while let token::Delimiter::Invisible(InvisibleOrigin::MetaVar(_)) = delimiter
                    && inner.len() == 1
                    && let Some(tokenstream::TokenTree::Delimited(_, _, nested, stream)) =
                        inner.get(0)
                    && let token::Delimiter::Invisible(InvisibleOrigin::MetaVar(_)) = nested
                {
                    delimiter = *nested;
                    inner = stream;
                }

                self.write_group_header(delimiter_atom(delimiter))?;
                self.open_stream(inner, Some(*span))
            }
            tokenstream::TokenTree::Token(token, spacing) => self.encode_token(*token, *spacing),
        }
    }

    fn encode_token(
        &mut self,
        token: token::Token,
        spacing: Spacing,
    ) -> Result<(), TokenEncodeError> {
        // Do not be tempted to check here that the `spacing`
        // values are "correct" w.r.t. the token stream (e.g. that
        // `Spacing::Joint` is actually followed by a `Punct` token
        // tree). Because the problem in #76399 was introduced that
        // way.
        //
        // This is where the `Hidden` in `JointHidden` applies,
        // because the jointness is effectively hidden from proc
        // macros.
        let joint = matches!(spacing, Spacing::Joint);
        match Wire::try_from(token)? {
            Wire::Op { op, span } => self.write_op(op, joint, span),
            Wire::Ident { name, is_raw, span } => self.write_ident(name.as_str(), is_raw, span),
            Wire::Literal { lit: token::Lit { kind, symbol, suffix }, span } => {
                let kind = lit_kind_value(kind);
                self.write_literal(kind, symbol.as_str(), suffix.as_ref().map(Symbol::as_str), span)
            }
            Wire::DocComment { style, data, span } => self.write_doc_comment(style, data, span),
        }
    }

    /// Split the operator into one or more `Punct`s, one per character.
    /// The final one inherits the jointness of the original token. Any
    /// before that get `joint = true`.
    fn write_op(&mut self, op: &str, joint: bool, span: Span) -> Result<(), TokenEncodeError> {
        // Split the token span into single chars. Unless the span
        // is an unusual one, e.g. due to proc macro expansion. We
        // determine this by assuming any span with a length that
        // matches the operator length is a normal one, and any
        // span with a different length is an unusual one.
        let span_per_char = (span.hi() - span.lo()).to_usize() == op.len();
        for (index, ch) in op.bytes().enumerate() {
            let final_char = index + 1 == op.len();
            let char_span = if span_per_char {
                let lo = span.lo() + BytePos::from_usize(index);
                span.with_lo(lo).with_hi(lo + BytePos::from_usize(1))
            } else {
                span
            };
            self.write_punct(ch, !final_char || joint, char_span)?;
        }
        Ok(())
    }

    fn write_doc_comment(
        &mut self,
        style: ast::AttrStyle,
        data: Symbol,
        span: Span,
    ) -> Result<(), TokenEncodeError> {
        let mut escaped = String::new();
        for ch in data.as_str().chars() {
            escaped.extend(ch.escape_debug());
        }

        self.write_punct(b'#', false, span)?;
        if style == ast::AttrStyle::Inner {
            self.write_punct(b'!', false, span)?;
        }

        self.write_group_header("bracket")?;
        write_list_header(&mut self.output, 3)?;
        self.write_ident("doc", IdentIsRaw::No, span)?;
        self.write_punct(b'=', false, span)?;
        self.write_literal(LitKindWire::Fieldless("str"), &escaped, None, span)?;
        write_nil(&mut self.output)?;
        self.write_delim_spans(Some(DelimSpan::from_single(span)))
    }

    fn write_punct(&mut self, ch: u8, joint: bool, span: Span) -> Result<(), TokenEncodeError> {
        write_tuple_header(&mut self.output, 4)?;
        write_atom(&mut self.output, "punct")?;
        write_integer(&mut self.output, i32::from(ch))?;
        write_bool(&mut self.output, joint)?;
        self.write_span(span)
    }

    fn write_ident(
        &mut self,
        name: &str,
        is_raw: IdentIsRaw,
        span: Span,
    ) -> Result<(), TokenEncodeError> {
        write_tuple_header(&mut self.output, 4)?;
        write_atom(&mut self.output, "ident")?;
        write_binary(&mut self.output, name.as_bytes())?;
        write_bool(&mut self.output, is_raw == IdentIsRaw::Yes)?;
        self.write_span(span)
    }

    fn write_literal(
        &mut self,
        kind: LitKindWire,
        text: &str,
        suffix: Option<&str>,
        span: Span,
    ) -> Result<(), TokenEncodeError> {
        write_tuple_header(&mut self.output, 5)?;
        write_atom(&mut self.output, "literal")?;
        self.write_lit_kind(kind)?;
        write_binary(&mut self.output, text.as_bytes())?;
        match suffix {
            Some(suffix) => write_binary(&mut self.output, suffix.as_bytes())?,
            None => write_atom(&mut self.output, "nil")?,
        }
        self.write_span(span)
    }

    fn write_lit_kind(&mut self, kind: LitKindWire) -> Result<(), TokenEncodeError> {
        match kind {
            LitKindWire::Fieldless(tag) => {
                write_tuple_header(&mut self.output, 1)?;
                write_atom(&mut self.output, tag)?;
            }
            LitKindWire::Raw(tag, hashes) => {
                write_tuple_header(&mut self.output, 2)?;
                write_atom(&mut self.output, tag)?;
                write_integer(&mut self.output, i32::from(hashes))?;
            }
        }
        Ok(())
    }
}

fn delimiter_atom(delimiter: token::Delimiter) -> &'static str {
    match delimiter {
        token::Delimiter::Parenthesis => "parenthesis",
        token::Delimiter::Brace => "brace",
        token::Delimiter::Bracket => "bracket",
        token::Delimiter::Invisible(_) => "none",
    }
}

#[derive(Clone, Copy)]
enum LitKindWire {
    Fieldless(&'static str),
    Raw(&'static str, u8),
}

fn lit_kind_value(kind: token::LitKind) -> LitKindWire {
    use LitKindWire::*;
    match kind {
        token::Byte => Fieldless("byte"),
        token::Char => Fieldless("char"),
        token::Integer => Fieldless("integer"),
        token::Float => Fieldless("float"),
        token::Str => Fieldless("str"),
        token::StrRaw(n) => Raw("str_raw", u8::from(n)),
        token::AtomStr => Fieldless("atom_str"),
        token::AtomStrRaw(n) => Raw("atom_str_raw", u8::from(n)),
        token::ByteStr => Fieldless("byte_str"),
        token::ByteStrRaw(n) => Raw("byte_str_raw", u8::from(n)),
        token::CStr => Fieldless("c_str"),
        token::CStrRaw(n) => Raw("c_str_raw", u8::from(n)),
        token::Err(_) => Fieldless("err"),
        token::Bool => unreachable!("`token::Bool` is never a literal kind"),
    }
}

// ---------------------------------------------------------------------------
// Decoding
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum ListState {
    ElementsLeft(usize),
    Done,
}

struct GroupFrame {
    delimiter: Option<token::Delimiter>,
    state: ListState,
    stream: TokenStream,
}

/// One token tree read off the wire.
enum Decoded {
    /// A group's header. Its contents follow, so a frame opens for them.
    Group { delimiter: token::Delimiter, state: ListState },
    /// Trees to append to the stream being decoded. We use a `SmallVec`
    /// because the output size is always one or two `TokenTree`s.
    Trees(SmallVec<[tokenstream::TokenTree; 2]>),
}

#[derive(Debug)]
pub enum DecodeResult {
    Normal(TokenStream),
    HadUnknownSpan(TokenStream),
}

struct Decoder<'a> {
    reader: etf::Reader<'a>,
    spans: &'a SpanTable,
    psess: &'a ParseSess,
    /// The streams being decoded, innermost last.
    stack: Vec<GroupFrame>,
    had_unknown_span: bool,
}

/// Decodes ETF bytes back into a rustc token stream, validating token values
/// the way the native `proc_macro` constructors would.
pub fn decode_tokens(
    bytes: &[u8],
    spans: &SpanTable,
    psess: &ParseSess,
) -> Result<DecodeResult, TokenDecodeError> {
    Decoder {
        reader: etf::Reader::new(bytes),
        spans,
        psess,
        stack: Vec::new(),
        had_unknown_span: false,
    }
    .run()
}

impl Decoder<'_> {
    /// Reads the streams depth first until the outermost one is complete.
    fn run(&mut self) -> Result<DecodeResult, TokenDecodeError> {
        let state = read_stream_start(&mut self.reader)?;
        self.stack.push(GroupFrame { delimiter: None, state, stream: TokenStream::default() });

        loop {
            match self.top_frame().state {
                ListState::ElementsLeft(0) => {
                    self.top_frame().state = read_stream_state(&mut self.reader)?;
                }
                ListState::ElementsLeft(remaining) => {
                    self.top_frame().state = ListState::ElementsLeft(remaining - 1);
                    match self.decode_token()? {
                        Decoded::Group { delimiter, state } => self.stack.push(GroupFrame {
                            delimiter: Some(delimiter),
                            state,
                            stream: TokenStream::default(),
                        }),
                        Decoded::Trees(trees) => {
                            let stream = &mut self.top_frame().stream;
                            for tree in trees {
                                stream.push_tree(tree);
                            }
                        }
                    }
                }
                ListState::Done => {
                    match self.stack.pop().expect("a frame is open") {
                        GroupFrame { delimiter: Some(delimiter), stream, state: _ } => {
                            let tree = self.close_group(delimiter, stream)?;
                            self.top_frame().stream.push_tree(tree);
                        }
                        GroupFrame { delimiter: None, stream, state: _ } => {
                            // Only the outermost stream has no delimiter.
                            self.reader.finish()?;
                            return Ok(if self.had_unknown_span {
                                DecodeResult::HadUnknownSpan(stream)
                            } else {
                                DecodeResult::Normal(stream)
                            });
                        }
                    }
                }
            }
        }
    }

    fn top_frame(&mut self) -> &mut GroupFrame {
        self.stack.last_mut().expect("a frame is open")
    }

    /// Reads the `DelimSpan` that follows a group's contents and rebuilds the group.
    fn close_group(
        &mut self,
        delimiter: token::Delimiter,
        stream: TokenStream,
    ) -> Result<tokenstream::TokenTree, TokenDecodeError> {
        let arity = self.reader.read_tuple_header()?;
        let tag = self.reader.read_atom()?;
        if (tag.0, arity) != ("delim_span", 4) {
            return Err(protocol(&self.reader, "expected a `delim_span` after a group's contents"));
        }
        let open = self.read_id_and_resolve_span()?;
        let close = self.read_id_and_resolve_span()?;
        // The entire-group span is validated even though the
        // reconstruction only keeps the opening and closing spans.
        let _entire = self.read_id_and_resolve_span()?;
        Ok(tokenstream::TokenTree::Delimited(
            DelimSpan::from_pair(open, close),
            DelimSpacing::new(Spacing::Alone, Spacing::Alone),
            delimiter,
            stream,
        ))
    }

    fn read_id_and_resolve_span(&mut self) -> Result<Span, TokenDecodeError> {
        let id = self.reader.read_integer()?.as_i64()?;
        match self.spans.resolve(id) {
            ResolvedSpan::Found(span) => Ok(span),
            ResolvedSpan::Unknown(call_site) => {
                self.had_unknown_span = true;
                Ok(call_site)
            }
        }
    }

    fn decode_token(&mut self) -> Result<Decoded, TokenDecodeError> {
        // The code below is conservative, using `token_alone`/`Spacing::Alone`
        // in most places. It's hard in general to do better when working at
        // the token level. When the resulting code is pretty-printed by
        // `print_tts` the `space_between` function helps avoid a lot of
        // unnecessary whitespace, so the results aren't too bad.
        let arity = self.reader.read_tuple_header()?;
        let tag = self.reader.read_atom()?;

        match (tag.0, arity) {
            ("group", 4) => {
                let delimiter = read_delimiter(&mut self.reader)?;
                let state = read_stream_start(&mut self.reader)?;
                Ok(Decoded::Group { delimiter, state })
            }
            ("ident", 4) => {
                let name = self.reader.read_string()?;
                let is_raw = self.reader.read_bool()?;
                let span = self.read_id_and_resolve_span()?;
                let symbol = intern_identifier(name, is_raw)?;
                self.psess.symbol_gallery.insert(symbol, span);
                Ok(Decoded::Trees(smallvec![tokenstream::TokenTree::token_alone(
                    token::Ident(symbol, IdentIsRaw::from(is_raw)),
                    span,
                )]))
            }
            ("punct", 4) => {
                let value = self.reader.read_integer()?.as_i64()?;
                let joint = self.reader.read_bool()?;
                let span = self.read_id_and_resolve_span()?;
                let ch = u8::try_from(value)
                    .map_err(|_| TokenDecodeError::UnsupportedPunctuation { value })?;
                if ch == b'\'' {
                    return Err(TokenDecodeError::Lifetime { span });
                }
                let kind =
                    punct_kind(ch).ok_or(TokenDecodeError::UnsupportedPunctuation { value })?;
                // We never produce `token::Spacing::JointHidden` here, which
                // means the pretty-printing of code produced by proc macros is
                // ugly, with lots of whitespace between tokens. This is
                // unavoidable because `proc_macro::Spacing` only applies to
                // `Punct` token trees.
                Ok(Decoded::Trees(smallvec![if joint {
                    tokenstream::TokenTree::token_joint(kind, span)
                } else {
                    tokenstream::TokenTree::token_alone(kind, span)
                }]))
            }
            ("literal", 5) => {
                let kind = read_lit_kind(&mut self.reader, self.psess.dcx().has_errors())?;
                let text = self.reader.read_string()?;
                let suffix = if self.reader.next_is_atom()? {
                    read_nil(&mut self.reader)?;
                    None
                } else {
                    Some(Symbol::intern(self.reader.read_string()?))
                };
                let span = self.read_id_and_resolve_span()?;
                if matches!(kind, token::Integer | token::Float)
                    && let Some(rest) = text.strip_prefix('-')
                {
                    let literal = TokenKind::lit(kind, Symbol::intern(rest), suffix);
                    Ok(Decoded::Trees(smallvec![
                        tokenstream::TokenTree::token_joint_hidden(token::Minus, span),
                        tokenstream::TokenTree::token_alone(literal, span),
                    ]))
                } else {
                    let literal = TokenKind::lit(kind, Symbol::intern(text), suffix);
                    Ok(Decoded::Trees(smallvec![tokenstream::TokenTree::token_alone(
                        literal, span
                    )]))
                }
            }
            (tag, arity) => Err(protocol(
                &self.reader,
                &format!("unknown token tree tag {tag} with arity {arity}"),
            )),
        }
    }
}

/// Reads the start of a stream: `nil` for an empty one, or else its list
/// header.
fn read_stream_start(reader: &mut etf::Reader<'_>) -> Result<ListState, TokenDecodeError> {
    if reader.next_is_atom()? {
        read_nil(reader)?;
        return Ok(ListState::Done);
    }
    read_stream_state(reader)
}

/// Reads the atom `nil`, which is `Option::None`.
fn read_nil(reader: &mut etf::Reader<'_>) -> Result<(), TokenDecodeError> {
    match reader.read_atom()?.0 {
        "nil" => Ok(()),
        atom => Err(protocol(reader, &format!("expected `nil`, found `{atom}`"))),
    }
}

/// Reads a stream's list header, or the tail that follows its elements, as the
/// number of elements still to come.
fn read_stream_state(reader: &mut etf::Reader<'_>) -> Result<ListState, TokenDecodeError> {
    match reader.read_list_header()? {
        etf::ListHeader::Empty => Ok(ListState::Done),
        etf::ListHeader::Proper(n) => Ok(ListState::ElementsLeft(n)),
        etf::ListHeader::String(_) => {
            Err(protocol(reader, "nonempty string is not a token stream"))
        }
    }
}

fn protocol(reader: &etf::Reader<'_>, details: &str) -> TokenDecodeError {
    TokenDecodeError::Protocol { offset: reader.position(), details: details.to_string() }
}

fn read_delimiter(reader: &mut etf::Reader<'_>) -> Result<token::Delimiter, TokenDecodeError> {
    match reader.read_atom()?.0 {
        "parenthesis" => Ok(token::Delimiter::Parenthesis),
        "brace" => Ok(token::Delimiter::Brace),
        "bracket" => Ok(token::Delimiter::Bracket),
        "none" => Ok(token::Delimiter::Invisible(InvisibleOrigin::ProcMacro)),
        delim => Err(protocol(reader, &format!("unknown delimiter {delim}"))),
    }
}

fn read_lit_kind(
    reader: &mut etf::Reader<'_>,
    prior_error: Option<ErrorGuaranteed>,
) -> Result<token::LitKind, TokenDecodeError> {
    let arity = reader.read_tuple_header()?;
    let tag = reader.read_atom()?;
    let fieldless = |kind| {
        if arity == 1 { Ok(kind) } else { Err(protocol(reader, "literal kind must be fieldless")) }
    };
    let raw = |kind: &'static str, reader: &mut etf::Reader<'_>| {
        if arity != 2 {
            return Err(protocol(reader, "raw literal kind must carry a hash count"));
        }
        let count = reader.read_integer()?.as_i64()?;
        let hashes = u8::try_from(count)
            .map_err(|_| TokenDecodeError::RawLiteralHashCountOutOfRange { kind, count })?;
        Ok(hashes)
    };

    match tag.0 {
        "byte" => fieldless(token::Byte),
        "char" => fieldless(token::Char),
        "integer" => fieldless(token::Integer),
        "float" => fieldless(token::Float),
        "str" => fieldless(token::Str),
        "str_raw" => Ok(token::StrRaw(raw("str_raw", reader)?)),
        "atom_str" => fieldless(token::AtomStr),
        "atom_str_raw" => Ok(token::AtomStrRaw(raw("atom_str_raw", reader)?)),
        "byte_str" => fieldless(token::ByteStr),
        "byte_str_raw" => Ok(token::ByteStrRaw(raw("byte_str_raw", reader)?)),
        "c_str" => fieldless(token::CStr),
        "c_str_raw" => Ok(token::CStrRaw(raw("c_str_raw", reader)?)),
        "err" => {
            // A macro cannot report an error through this protocol, but it can
            // return an error literal that rustc previously passed in during
            // error recovery. Never let arbitrary ETF fabricate the guarantee.
            fieldless(token::Err(prior_error.ok_or(TokenDecodeError::UnreportedErrorLiteral)?))
        }
        kind => Err(protocol(reader, &format!("unknown literal kind {kind}"))),
    }
}

fn intern_identifier(name: &str, is_raw: bool) -> Result<Symbol, TokenDecodeError> {
    if name == "$crate" && !is_raw {
        return Ok(kw::DollarCrate);
    }
    let normalized = nfc_normalize(name);
    let valid =
        if is_raw { normalized.can_be_raw() } else { rustc_lexer::is_ident(normalized.as_str()) };
    if !valid {
        return Err(TokenDecodeError::InvalidIdentifier { name: name.to_string(), is_raw });
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests;
