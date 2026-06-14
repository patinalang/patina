use rustc_arena::DroplessArena;
use rustc_span::source_map::SourceMap;
use rustc_span::{Span, Symbol};

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

    pub(crate) fn expr_atom(&self, span: impl IntoAnno, name: Symbol) -> eaf::Expr<'arena> {
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::Atom(
            span.into_anno(self.source_map),
            name.into(),
        ))
    }

    // TODO: BigUInt literals
    pub(crate) fn expr_integer(&self, span: impl IntoAnno, value: u128) -> eaf::Expr<'arena> {
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
        let chars = self.arena.alloc_from_iter(s.as_str().chars());
        let value = eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::String(anno, chars));
        let elem = eaf::BinElement {
            anno,
            value,
            size: eaf::BinElemSize::Default,
            tsl: eaf::BinElemTsl::Default,
        };
        eaf::Expr::Bin(anno, self.arena.alloc_slice(&[elem]))
    }

    pub(crate) fn expr_var(&self, span: impl IntoAnno, name: Symbol) -> eaf::Expr<'arena> {
        eaf::Expr::Var(span.into_anno(self.source_map), name.into())
    }

    pub(crate) fn pat_var(&self, span: impl IntoAnno, name: Symbol) -> eaf::Pattern<'arena> {
        eaf::Pattern::Var(span.into_anno(self.source_map), name.into())
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

    /// A block expression `begin ... end` over the given expressions.
    pub(crate) fn expr_block(
        &self,
        span: impl IntoAnno,
        exprs: impl IntoIterator<Item = eaf::Expr<'arena>>,
    ) -> eaf::Expr<'arena> {
        let exprs = self.arena.alloc_from_iter(exprs);
        eaf::Expr::Block(span.into_anno(self.source_map), eaf::Body(exprs))
    }

    pub(crate) fn clause_body(&self, expr: eaf::Expr<'arena>) -> eaf::Body<'arena> {
        match expr {
            eaf::Expr::Block(_, body) => body,
            other => eaf::Body(self.arena.alloc_slice(&[other])),
        }
    }

    pub(crate) fn clause(
        &self,
        span: impl IntoAnno,
        patterns: impl IntoIterator<Item = eaf::Pattern<'arena>>,
        body: eaf::Body<'arena>,
    ) -> eaf::Clause<'arena> {
        eaf::Clause {
            anno: span.into_anno(self.source_map),
            patterns: self.arena.alloc_from_iter(patterns),
            guards: eaf::GuardSeq(&[]),
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

    pub(crate) fn module(
        &self,
        forms: impl IntoIterator<Item = eaf::ModuleForm<'arena>>,
    ) -> eaf::Module<'arena> {
        let forms = self.arena.alloc_from_iter(forms);
        eaf::Module(forms)
    }
}
