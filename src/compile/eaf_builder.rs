//! Convenience functions for building Erlang Abstract Format (eaf) nodes.
use rustc_arena::DroplessArena;
use rustc_span::source_map::SourceMap;
use rustc_span::{ByteSymbol, Ident, Span, Symbol};

use crate::erlang::abstract_format as eaf;
use crate::num_utils::trim_trailing_zeroes;

pub(crate) trait IntoAnno {
    fn into_anno(self, source_map: &SourceMap) -> eaf::Anno;
}

impl IntoAnno for Span {
    fn into_anno(self, source_map: &SourceMap) -> eaf::Anno {
        // If we're in a macro, point to the macro's outermost callsite.
        let loc = source_map.lookup_char_pos(self.source_callsite().lo());
        eaf::Anno { line: loc.line, column: loc.col.0 + 1 }
    }
}

impl IntoAnno for eaf::Anno {
    fn into_anno(self, _source_map: &SourceMap) -> eaf::Anno {
        self
    }
}

/// Convenience constructors for Erlang Abstract Format nodes.
pub(crate) struct EafBuilder<'sm, 'arena> {
    source_map: &'sm SourceMap,
    arena: &'arena DroplessArena,
}

impl<'sm, 'arena> EafBuilder<'sm, 'arena> {
    pub(crate) fn new(source_map: &'sm SourceMap, arena: &'arena DroplessArena) -> Self {
        Self { source_map, arena }
    }

    pub(crate) fn pat_atom(&self, span: impl IntoAnno, name: Symbol) -> eaf::Pattern<'arena> {
        eaf::Pattern::AtomicLiteral(eaf::AtomicLiteral::Atom(
            span.into_anno(self.source_map),
            name.into(),
        ))
    }

    pub(crate) fn pat_int(&self, span: impl IntoAnno, value: u128) -> eaf::Pattern<'arena> {
        eaf::Pattern::AtomicLiteral(eaf::AtomicLiteral::Integer(
            span.into_anno(self.source_map),
            self.biguint(value),
        ))
    }

    pub(crate) fn expr_atom(&self, span: impl IntoAnno, name: Symbol) -> eaf::Expr<'arena> {
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::Atom(
            span.into_anno(self.source_map),
            name.into(),
        ))
    }

    // TODO: BigUInt literals
    pub(crate) fn expr_int(&self, span: impl IntoAnno, value: u128) -> eaf::Expr<'arena> {
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::Integer(
            span.into_anno(self.source_map),
            self.biguint(value),
        ))
    }

    fn biguint(&self, value: u128) -> eaf::BigUInt<'arena> {
        let bytes = value.to_le_bytes();
        let slice = self.arena.alloc_slice(trim_trailing_zeroes(&bytes));
        eaf::BigUInt(slice)
    }

    pub(crate) fn expr_char(&self, span: impl IntoAnno, value: char) -> eaf::Expr<'arena> {
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::Char(span.into_anno(self.source_map), value))
    }

    pub(crate) fn expr_float(&self, span: impl IntoAnno, value: f64) -> eaf::Expr<'arena> {
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::Float(span.into_anno(self.source_map), value))
    }

    /// List-based string literal.
    pub(crate) fn expr_string(&self, span: impl IntoAnno, s: Symbol) -> eaf::Expr<'arena> {
        let chars = self.arena.alloc_from_iter(s.as_str().chars());
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::String(span.into_anno(self.source_map), chars))
    }

    /// Binary string literal.
    pub(crate) fn expr_binary_string(&self, span: impl IntoAnno, s: Symbol) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);
        let value = self.expr_string(anno, s);
        let elem = self.expr_binary_element(anno, value, None, None::<[eaf::Tsl; 0]>);
        self.expr_binary(anno, [elem])
    }

    /// Binary literal made out of single-byte ints.
    pub(crate) fn expr_binary_bytes(
        &self,
        span: impl IntoAnno,
        s: ByteSymbol,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);

        let elems = s.as_byte_str().iter().map(|&b| {
            let value = self.expr_int(anno, b as u128);
            self.expr_binary_element(anno, value, None, None::<[eaf::Tsl; 0]>)
        });

        self.expr_binary(anno, elems)
    }

    pub(crate) fn expr_binary_element(
        &self,
        span: impl IntoAnno,
        value: eaf::Expr<'arena>,
        size: Option<eaf::BinElemSize<'arena>>,
        tsl: Option<impl IntoIterator<Item = eaf::Tsl>>,
    ) -> eaf::BinElement<'arena, eaf::Expr<'arena>> {
        let anno = span.into_anno(self.source_map);
        let size = size.unwrap_or(eaf::BinElemSize::Default);
        let tsl = tsl
            .map(|tsl| eaf::BinElemTsl::Tsl(self.arena.alloc_from_iter(tsl)))
            .unwrap_or(eaf::BinElemTsl::Default);
        eaf::BinElement { anno, value, size, tsl }
    }

    pub(crate) fn expr_binary(
        &self,
        span: impl IntoAnno,
        elements: impl IntoIterator<Item = eaf::BinElement<'arena, eaf::Expr<'arena>>>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);
        let elements = self.arena.alloc_from_iter(elements);
        eaf::Expr::Bin(anno, elements)
    }

    pub(crate) fn expr_binary_op(
        &self,
        span: impl IntoAnno,
        op: eaf::BinaryOp,
        left: eaf::Expr<'arena>,
        right: eaf::Expr<'arena>,
    ) -> eaf::Expr<'arena> {
        eaf::Expr::BinaryOp(
            span.into_anno(self.source_map),
            op,
            self.arena.alloc(left),
            self.arena.alloc(right),
        )
    }

    pub(crate) fn expr_unary_op(
        &self,
        span: impl IntoAnno,
        op: eaf::UnaryOp,
        expr: eaf::Expr<'arena>,
    ) -> eaf::Expr<'arena> {
        eaf::Expr::UnaryOp(span.into_anno(self.source_map), op, self.arena.alloc(expr))
    }

    pub(crate) fn expr_var(&self, span: impl IntoAnno, name: Symbol) -> eaf::Expr<'arena> {
        eaf::Expr::Var(span.into_anno(self.source_map), name.into())
    }

    pub(crate) fn pat_var(&self, span: impl IntoAnno, name: Symbol) -> eaf::Pattern<'arena> {
        eaf::Pattern::Var(span.into_anno(self.source_map), name.into())
    }

    /// `Lhs = Rhs` pattern.
    pub(crate) fn pat_match(
        &self,
        span: impl IntoAnno,
        lhs: eaf::Pattern<'arena>,
        rhs: eaf::Pattern<'arena>,
    ) -> eaf::Pattern<'arena> {
        let anno = span.into_anno(self.source_map);
        eaf::Pattern::Match(anno, self.arena.alloc(lhs), self.arena.alloc(rhs))
    }

    pub(crate) fn pat_tuple(
        &self,
        span: impl IntoAnno,
        elements: impl IntoIterator<Item = eaf::Pattern<'arena>>,
    ) -> eaf::Pattern<'arena> {
        let anno = span.into_anno(self.source_map);
        let elements = self.arena.alloc_from_iter(elements);
        eaf::Pattern::Tuple(anno, elements)
    }

    pub(crate) fn pat_map(
        &self,
        span: impl IntoAnno,
        assocs: impl IntoIterator<Item = (eaf::Pattern<'arena>, eaf::Pattern<'arena>)>,
    ) -> eaf::Pattern<'arena> {
        let anno = span.into_anno(self.source_map);

        let iter = assocs.into_iter().map(|(key, value)| eaf::MapFieldExact(anno, key, value));
        let assocs = self.arena.alloc_from_iter(iter);

        eaf::Pattern::Map(anno, assocs)
    }

    pub(crate) fn expr_remote_fun(
        &self,
        span: impl IntoAnno,
        module: Symbol,
        name: Symbol,
        arity: usize,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);
        let module = self.expr_atom(anno, module);
        let name = self.expr_atom(anno, name);
        let arity = self.expr_int(anno, arity as u128);
        eaf::Expr::RemoteFun(
            anno,
            eaf::RemoteFunction(
                self.arena.alloc(module),
                self.arena.alloc(name),
                self.arena.alloc(arity),
            ),
        )
    }

    pub(crate) fn expr_remote_call(
        &self,
        span: impl IntoAnno,
        module: eaf::Expr<'arena>,
        func: eaf::Expr<'arena>,
        args: impl IntoIterator<Item = eaf::Expr<'arena>>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);
        let remote = eaf::Remote(anno, self.arena.alloc(module), self.arena.alloc(func));
        let args = self.arena.alloc_from_iter(args);
        eaf::Expr::RemoteCall(anno, remote, args)
    }

    pub(crate) fn expr_local_call(
        &self,
        span: impl IntoAnno,
        func: eaf::Expr<'arena>,
        args: impl IntoIterator<Item = eaf::Expr<'arena>>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);
        let args = self.arena.alloc_from_iter(args);
        eaf::Expr::LocalCall(anno, self.arena.alloc(func), args)
    }

    /// The empty list `[]`.
    pub(crate) fn expr_nil(&self, span: impl IntoAnno) -> eaf::Expr<'arena> {
        eaf::Expr::Nil { anno: span.into_anno(self.source_map) }
    }

    /// `[head | tail]`.
    pub(crate) fn expr_cons(
        &self,
        span: impl IntoAnno,
        head: eaf::Expr<'arena>,
        tail: eaf::Expr<'arena>,
    ) -> eaf::Expr<'arena> {
        eaf::Expr::Cons(
            span.into_anno(self.source_map),
            self.arena.alloc(head),
            self.arena.alloc(tail),
        )
    }

    pub(crate) fn expr_list(
        &self,
        span: impl IntoAnno,
        items: impl IntoIterator<Item = eaf::Expr<'arena>, IntoIter: DoubleEndedIterator>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);
        items
            .into_iter()
            .rev()
            .fold(self.expr_nil(anno), |acc, item| self.expr_cons(anno, item, acc))
    }

    pub(crate) fn expr_tuple(
        &self,
        span: impl IntoAnno,
        elements: impl IntoIterator<Item = eaf::Expr<'arena>>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);
        let elements = self.arena.alloc_from_iter(elements);
        eaf::Expr::Tuple(anno, elements)
    }

    pub(crate) fn expr_fun(
        &self,
        span: impl IntoAnno,
        clauses: impl IntoIterator<Item = eaf::Clause<'arena>>,
    ) -> eaf::Expr<'arena> {
        let clauses = self.arena.alloc_from_iter(clauses);
        eaf::Expr::ClausedFun(span.into_anno(self.source_map), eaf::Clauses { clauses })
    }

    pub(crate) fn expr_map(
        &self,
        span: impl IntoAnno,
        assocs: impl IntoIterator<Item = (eaf::Expr<'arena>, eaf::Expr<'arena>)>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);

        let iter = assocs.into_iter().map(|(key, value)| eaf::MapFieldAssoc(anno, key, value));
        let assocs = self.arena.alloc_from_iter(iter);

        eaf::Expr::MapCreate(anno, assocs)
    }

    pub(crate) fn expr_map_update_exact(
        &self,
        span: impl IntoAnno,
        base: eaf::Expr<'arena>,
        updates: impl IntoIterator<Item = (eaf::Expr<'arena>, eaf::Expr<'arena>)>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);

        let iter =
            updates.into_iter().map(|(key, value)| eaf::MapField::MapFieldExact(anno, key, value));
        let updates = self.arena.alloc_from_iter(iter);

        eaf::Expr::MapUpdate(anno, self.arena.alloc(base), updates)
    }

    /// `Pattern = Rhs`.
    pub(crate) fn expr_match(
        &self,
        span: impl IntoAnno,
        pat: eaf::Pattern<'arena>,
        rhs: eaf::Expr<'arena>,
    ) -> eaf::Expr<'arena> {
        eaf::Expr::Match(span.into_anno(self.source_map), pat, self.arena.alloc(rhs))
    }

    pub(crate) fn expr_case(
        &self,
        span: impl IntoAnno,
        expr: eaf::Expr<'arena>,
        clauses: impl IntoIterator<Item = eaf::Clause<'arena>>,
    ) -> eaf::Expr<'arena> {
        eaf::Expr::Case(
            span.into_anno(self.source_map),
            self.arena.alloc(expr),
            self.arena.alloc_from_iter(clauses),
        )
    }

    /// A block expression `begin ... end` over the given expressions.
    pub(crate) fn expr_block(
        &self,
        span: impl IntoAnno,
        exprs: impl IntoIterator<Item = eaf::Expr<'arena>>,
    ) -> eaf::Expr<'arena> {
        let exprs = self.arena.alloc_from_iter(exprs);
        eaf::Expr::Block(span.into_anno(self.source_map), eaf::Body(exprs))
    }

    pub(crate) fn clause(
        &self,
        span: impl IntoAnno,
        patterns: impl IntoIterator<Item = eaf::Pattern<'arena>>,
        guard: Option<eaf::Expr<'arena>>,
        body: eaf::Expr<'arena>,
    ) -> eaf::Clause<'arena> {
        let body = match body {
            eaf::Expr::Block(_, body) => body,
            other => eaf::Body(self.arena.alloc_slice(&[other])),
        };

        let guards = if let Some(test) = guard {
            let guard = eaf::Guard(self.arena.alloc_slice(&[test]));
            eaf::GuardSeq(self.arena.alloc_slice(&[guard]))
        } else {
            eaf::GuardSeq(&[])
        };

        eaf::Clause {
            anno: span.into_anno(self.source_map),
            patterns: self.arena.alloc_from_iter(patterns),
            guards,
            body,
        }
    }

    pub(crate) fn file_attr(&self, span: impl IntoAnno, filename: &str) -> eaf::ModuleForm<'arena> {
        let anno = span.into_anno(self.source_map);
        let chars = self.arena.alloc_from_iter(filename.chars());
        eaf::ModuleForm::FileAttr(anno, eaf::FileLine(chars, anno.line))
    }

    pub(crate) fn module_attr(&self, span: impl IntoAnno, name: Symbol) -> eaf::ModuleForm<'arena> {
        eaf::ModuleForm::ModuleAttr(span.into_anno(self.source_map), name.into())
    }

    pub(crate) fn wild_attr(
        &self,
        span: impl IntoAnno,
        name: Symbol,
        values: impl IntoIterator<Item = Symbol>,
    ) -> eaf::ModuleForm<'arena> {
        let iter = values.into_iter().map(Into::into);
        let values = self.arena.alloc_from_iter(iter);
        eaf::ModuleForm::WildAttr(span.into_anno(self.source_map), name.into(), values)
    }

    pub(crate) fn export_attr(
        &self,
        span: impl IntoAnno,
        funs: impl IntoIterator<Item = (Symbol, usize)>,
    ) -> eaf::ModuleForm<'arena> {
        let iter = funs.into_iter().map(|(name, arity)| eaf::NameArity(name.into(), arity));
        let funs = self.arena.alloc_from_iter(iter);
        eaf::ModuleForm::ExportAttr(span.into_anno(self.source_map), funs)
    }

    pub(crate) fn native_record_attr(
        &self,
        span: impl IntoAnno,
        name: Symbol,
        fields: impl IntoIterator<Item = eaf::RecordFieldDecl<'arena>>,
    ) -> eaf::ModuleForm<'arena> {
        let fields = self.arena.alloc_from_iter(fields);
        eaf::ModuleForm::NativeRecordAttr(
            span.into_anno(self.source_map),
            eaf::RecordDecl(name.into(), fields),
        )
    }

    pub(crate) fn record_field_decl(
        &self,
        span: impl IntoAnno,
        field_name: Ident,
        default: Option<eaf::Expr<'arena>>,
    ) -> eaf::RecordFieldDecl<'arena> {
        let name_atom =
            eaf::Atom(field_name.span.into_anno(self.source_map), field_name.name.into());

        eaf::RecordFieldDecl::Untyped(eaf::RecordField(
            span.into_anno(self.source_map),
            name_atom,
            default,
        ))
    }

    pub(crate) fn export_record_attr(
        &self,
        span: impl IntoAnno,
        records: impl IntoIterator<Item = Symbol>,
    ) -> eaf::ModuleForm<'arena> {
        let records = self.arena.alloc_from_iter(records.into_iter().map(Into::into));
        eaf::ModuleForm::ExportRecordAttr(span.into_anno(self.source_map), records)
    }

    pub(crate) fn expr_record_create(
        &self,
        span: impl IntoAnno,
        module: Option<Symbol>,
        name: Symbol,
        fields: impl IntoIterator<Item = (Ident, eaf::Expr<'arena>)>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);

        let iter = fields.into_iter().map(|(field, value)| {
            let anno = field.span.into_anno(self.source_map);
            let key = eaf::AtomOrVar::Atom(anno, field.name.into());
            eaf::RecordField(anno, key, Some(value))
        });
        let fields = self.arena.alloc_from_iter(iter);

        let record_name = match module {
            None => eaf::RecordName::Local(name.into()),
            Some(module) => eaf::RecordName::Extern(module.into(), name.into()),
        };

        eaf::Expr::RecordCreate(anno, record_name, fields)
    }

    pub(crate) fn expr_record_update(
        &self,
        span: impl IntoAnno,
        base: eaf::Expr<'arena>,
        module: Option<Symbol>,
        name: Symbol,
        fields: impl IntoIterator<Item = (Ident, eaf::Expr<'arena>)>,
    ) -> eaf::Expr<'arena> {
        let anno = span.into_anno(self.source_map);

        let base = self.arena.alloc(base);

        let iter = fields.into_iter().map(|(field, value)| {
            let anno = field.span.into_anno(self.source_map);
            let key = eaf::Atom(anno, field.name.into());
            eaf::RecordField(anno, key, Some(value))
        });
        let fields = self.arena.alloc_from_iter(iter);

        let record_name = match module {
            None => eaf::RecordName::Local(name.into()),
            Some(module) => eaf::RecordName::Extern(module.into(), name.into()),
        };
        eaf::Expr::RecordUpdate(anno, base, record_name, fields)
    }

    pub(crate) fn pat_record(
        &self,
        span: impl IntoAnno,
        module: Option<Symbol>,
        name: Symbol,
        fields: impl IntoIterator<Item = (Ident, eaf::Pattern<'arena>)>,
    ) -> eaf::Pattern<'arena> {
        let anno = span.into_anno(self.source_map);

        let iter = fields.into_iter().map(|(field, value)| {
            let anno = field.span.into_anno(self.source_map);
            let key = eaf::AtomOrVar::Atom(anno, field.name.into());
            eaf::RecordField(anno, key, Some(value))
        });
        let fields = self.arena.alloc_from_iter(iter);

        let record_name = match module {
            None => eaf::RecordName::Local(name.into()),
            Some(module) => eaf::RecordName::Extern(module.into(), name.into()),
        };
        eaf::Pattern::Record(anno, record_name, fields)
    }

    pub(crate) fn function(
        &self,
        span: impl IntoAnno,
        name: Symbol,
        arity: usize,
        clauses: impl IntoIterator<Item = eaf::Clause<'arena>>,
    ) -> eaf::ModuleForm<'arena> {
        let clauses = self.arena.alloc_from_iter(clauses);
        eaf::ModuleForm::Function(span.into_anno(self.source_map), name.into(), arity, clauses)
    }

    pub(crate) fn simple_function(
        &self,
        span: impl IntoAnno,
        name: Symbol,
        params: impl IntoIterator<Item = eaf::Pattern<'arena>>,
        body: eaf::Expr<'arena>,
    ) -> eaf::ModuleForm<'arena> {
        let anno = span.into_anno(self.source_map);
        let clause = self.clause(anno, params, None, body);
        self.function(anno, name, clause.patterns.len(), [clause])
    }

    pub(crate) fn module(
        &self,
        forms: impl IntoIterator<Item = eaf::ModuleForm<'arena>>,
    ) -> eaf::Module<'arena> {
        let forms = self.arena.alloc_from_iter(forms);
        eaf::Module(forms)
    }

    pub(crate) fn expr_unit(&self, span: impl IntoAnno) -> eaf::Expr<'arena> {
        eaf::Expr::Tuple(span.into_anno(self.source_map), &[])
    }
}
