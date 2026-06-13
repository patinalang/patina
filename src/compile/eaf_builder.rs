use num_bigint::BigUint;
use rustc_span::Span;
use rustc_span::source_map::SourceMap;

use crate::erlang::abstract_format as eaf;

/// Convenience constructors for Erlang Abstract Format nodes.
pub(crate) struct EafBuilder<'sm> {
    source_map: &'sm SourceMap,
}

impl<'sm> EafBuilder<'sm> {
    pub(crate) fn new(source_map: &'sm SourceMap) -> Self {
        Self { source_map }
    }

    pub(crate) fn anno(&self, span: Span) -> eaf::Anno {
        let loc = self.source_map.lookup_char_pos(span.lo());
        eaf::Anno { line: loc.line, column: loc.col.0 + 1 }
    }

    pub(crate) fn expr_atom<'a>(&self, span: Span, name: &'a str) -> eaf::Expr<'a> {
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::Atom(self.anno(span), name))
    }

    pub(crate) fn expr_integer<'a>(&self, span: Span, value: impl Into<BigUint>) -> eaf::Expr<'a> {
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::Integer(self.anno(span), value.into()))
    }

    /// List-based string literal.
    pub(crate) fn expr_string<'a>(&self, span: Span, s: &str) -> eaf::Expr<'a> {
        eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::String(self.anno(span), s.chars().collect()))
    }

    /// Binary string literal.
    pub(crate) fn expr_binary_string<'a>(&self, span: Span, s: &str) -> eaf::Expr<'a> {
        let anno = self.anno(span);
        let value = eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::String(anno, s.chars().collect()));
        let elem = eaf::BinElement {
            anno,
            value,
            size: eaf::BinElemSize::Default,
            tsl: eaf::BinElemTsl::Default,
        };
        eaf::Expr::Bin(anno, vec![elem])
    }

    pub(crate) fn expr_var<'a>(&self, span: Span, name: &'a str) -> eaf::Expr<'a> {
        eaf::Expr::Var(self.anno(span), name)
    }

    pub(crate) fn pat_var<'a>(&self, span: Span, name: &'a str) -> eaf::Pattern<'a> {
        eaf::Pattern::Var(self.anno(span), name)
    }

    pub(crate) fn expr_remote_call<'a>(
        &self,
        span: Span,
        module: impl Into<Box<eaf::Expr<'a>>>,
        func: impl Into<Box<eaf::Expr<'a>>>,
        args: Vec<eaf::Expr<'a>>,
    ) -> eaf::Expr<'a> {
        let anno = self.anno(span);
        let remote = eaf::Remote(anno, module.into(), func.into());
        eaf::Expr::RemoteCall(anno, remote, args)
    }

    pub(crate) fn expr_local_call<'a>(
        &self,
        span: Span,
        func: impl Into<Box<eaf::Expr<'a>>>,
        args: Vec<eaf::Expr<'a>>,
    ) -> eaf::Expr<'a> {
        let anno = self.anno(span);
        eaf::Expr::LocalCall(anno, func.into(), args)
    }

    /// The empty list `[]`.
    pub(crate) fn expr_nil<'a>(&self, span: Span) -> eaf::Expr<'a> {
        eaf::Expr::Nil { anno: self.anno(span) }
    }

    /// `[head | tail]`.
    pub(crate) fn expr_cons<'a>(
        &self,
        span: Span,
        head: impl Into<Box<eaf::Expr<'a>>>,
        tail: impl Into<Box<eaf::Expr<'a>>>,
    ) -> eaf::Expr<'a> {
        eaf::Expr::Cons(self.anno(span), head.into(), tail.into())
    }

    pub(crate) fn expr_list<'a>(
        &self,
        span: Span,
        items: impl IntoIterator<Item = eaf::Expr<'a>, IntoIter: DoubleEndedIterator>,
    ) -> eaf::Expr<'a> {
        items
            .into_iter()
            .rev()
            .fold(self.expr_nil(span), |acc, item| self.expr_cons(span, item, acc))
    }

    pub(crate) fn expr_tuple<'a>(&self, span: Span, elements: Vec<eaf::Expr<'a>>) -> eaf::Expr<'a> {
        eaf::Expr::Tuple(self.anno(span), elements)
    }

    /// `Pattern = Rhs`.
    pub(crate) fn expr_match<'a>(
        &self,
        span: Span,
        pat: eaf::Pattern<'a>,
        rhs: impl Into<Box<eaf::Expr<'a>>>,
    ) -> eaf::Expr<'a> {
        eaf::Expr::Match(self.anno(span), pat, rhs.into())
    }

    /// A block expression `begin ... end` over the given expressions.
    pub(crate) fn expr_block<'a>(&self, span: Span, exprs: Vec<eaf::Expr<'a>>) -> eaf::Expr<'a> {
        eaf::Expr::Block(self.anno(span), eaf::Body(exprs))
    }

    pub(crate) fn clause_body<'a>(&self, expr: eaf::Expr<'a>) -> eaf::Body<'a> {
        match expr {
            eaf::Expr::Block(_, body) => body,
            other => eaf::Body(vec![other]),
        }
    }

    pub(crate) fn clause<'a>(
        &self,
        span: Span,
        patterns: Vec<eaf::Pattern<'a>>,
        body: eaf::Body<'a>,
    ) -> eaf::Clause<'a> {
        eaf::Clause { anno: self.anno(span), patterns, guards: eaf::GuardSeq(vec![]), body }
    }

    pub(crate) fn file_attr<'a>(&self, span: Span, filename: &str) -> eaf::ModuleForm<'a> {
        let anno = self.anno(span);
        eaf::ModuleForm::FileAttr(anno, eaf::FileLine(filename.chars().collect(), anno.line))
    }

    pub(crate) fn module_attr<'a>(&self, span: Span, name: &'a str) -> eaf::ModuleForm<'a> {
        eaf::ModuleForm::ModuleAttr(self.anno(span), name)
    }

    pub(crate) fn wild_attr<'a>(
        &self,
        span: Span,
        name: &'a str,
        values: Vec<&'a str>,
    ) -> eaf::ModuleForm<'a> {
        eaf::ModuleForm::WildAttr(self.anno(span), name, values)
    }

    pub(crate) fn export_attr<'a>(
        &self,
        span: Span,
        funs: Vec<(&'a str, usize)>,
    ) -> eaf::ModuleForm<'a> {
        let funs = funs.into_iter().map(|(name, arity)| eaf::NameArity(name, arity)).collect();
        eaf::ModuleForm::ExportAttr(self.anno(span), funs)
    }

    pub(crate) fn function<'a>(
        &self,
        span: Span,
        name: &'a str,
        arity: usize,
        clauses: Vec<eaf::Clause<'a>>,
    ) -> eaf::ModuleForm<'a> {
        eaf::ModuleForm::Function(self.anno(span), name, arity, clauses)
    }

    pub(crate) fn module<'a>(&self, forms: Vec<eaf::ModuleForm<'a>>) -> eaf::Module<'a> {
        eaf::Module(forms)
    }
}
