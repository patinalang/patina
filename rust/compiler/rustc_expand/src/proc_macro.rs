//! BEAM-backed procedural macros.
//!
//! Patina compiles a `proc-macro` package to ordinary BEAM functions. This module connects rustc's
//! expansion machinery to those functions: it converts token streams to ETF,
//! asks a [`BeamMacroRunner`] to invoke `module:function`, and converts the result back.
//! This module owns the token protocol and the driver only owns transport.

mod tokens;

use std::sync::Arc;

use rustc_ast as ast;
use rustc_ast::tokenstream::TokenStream;
use rustc_data_structures::sync;
use rustc_errors::ErrorGuaranteed;
use rustc_middle::ty::{self, TyCtxt};
use rustc_parse::parser::{AllowConstBlockItems, ForceCollect, Parser};
use rustc_span::hygiene::MacroKind;
use rustc_span::{LocalExpnId, Span, Symbol};
use tokens::{
    DecodeResult, SpanTable, TokenDecodeError, TokenEncodeError, decode_tokens, encode_tokens,
};

use crate::base::{self, *};
use crate::errors;

/// A failure reported by the macro runner.
#[derive(Debug, Clone)]
pub enum MacroRunError {
    /// The macro raised an exception in the VM, or its process died.
    Raised(String),
    /// The macro exceeded the runner's execution timeout.
    Timeout,
    /// The runner itself failed: setup, I/O, or protocol handling.
    /// We assume that the transport is unusable from then on.
    Transport(String),
}

/// Why expanding a BEAM proc macro failed.
enum ExpansionFailure {
    /// The runner could not produce output.
    Run(MacroRunError),
    /// The macro's input could not be put on the wire.
    Encode(TokenEncodeError),
    /// The macro's output was not a token stream rustc would accept.
    Decode(TokenDecodeError),
}

impl From<MacroRunError> for ExpansionFailure {
    fn from(error: MacroRunError) -> ExpansionFailure {
        ExpansionFailure::Run(error)
    }
}

impl From<TokenEncodeError> for ExpansionFailure {
    fn from(error: TokenEncodeError) -> ExpansionFailure {
        ExpansionFailure::Encode(error)
    }
}

impl From<TokenDecodeError> for ExpansionFailure {
    fn from(error: TokenDecodeError) -> ExpansionFailure {
        ExpansionFailure::Decode(error)
    }
}

/// Reports a failed expansion of a `kind` macro invoked at `span`.
fn emit_expansion_failure(
    ecx: &ExtCtxt<'_>,
    kind: MacroKind,
    span: Span,
    failure: ExpansionFailure,
) -> ErrorGuaranteed {
    let raised = |message: String| match kind {
        MacroKind::Bang => ecx.dcx().emit_err(errors::ProcMacroPanicked {
            span,
            message: errors::ProcMacroPanickedHelp { message },
        }),
        MacroKind::Attr => ecx.dcx().emit_err(errors::CustomAttributePanicked {
            span,
            message: errors::CustomAttributePanickedHelp { message },
        }),
        MacroKind::Derive => ecx.dcx().emit_err(errors::ProcMacroDerivePanicked {
            span,
            message: errors::ProcMacroDerivePanickedHelp { message },
        }),
    };
    let execution_error =
        |reason: String| ecx.dcx().emit_err(errors::ProcMacroExecutionError { span, reason });
    // A lifetime is reported at the offending token, which is not the
    // invocation `span` every other arm here uses.
    let lifetime =
        |token_span| ecx.dcx().emit_err(errors::LifetimeInMacroTokens { span: token_span });

    match failure {
        ExpansionFailure::Run(MacroRunError::Raised(message)) => raised(message),
        ExpansionFailure::Run(MacroRunError::Timeout) => {
            ecx.dcx().emit_err(errors::ProcMacroTimeout { span })
        }
        ExpansionFailure::Run(MacroRunError::Transport(reason)) => {
            ecx.dcx().emit_fatal(errors::ProcMacroExecutionError { span, reason })
        }
        ExpansionFailure::Encode(TokenEncodeError::Lifetime { span }) => lifetime(span),
        ExpansionFailure::Encode(TokenEncodeError::Etf { details }) => execution_error(details),
        ExpansionFailure::Decode(TokenDecodeError::Lifetime { span }) => lifetime(span),
        ExpansionFailure::Decode(error @ TokenDecodeError::Protocol { .. }) => {
            execution_error(format!("invalid proc-macro token protocol: {error}"))
        }
        ExpansionFailure::Decode(error) => raised(error.to_string()),
    }
}

/// Invokes BEAM proc-macro entry points. Installed by the driver.
pub trait BeamMacroRunner: Send + Sync + sync::DynSend + sync::DynSync {
    /// Call `module:function(Args...)` in the Erlang VM.
    fn call(
        &self,
        module: &str,
        function: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>, MacroRunError>;
}

/// Converts `args` to terms, invokes the macro, and converts its output.
fn run(
    ecx: &mut ExtCtxt<'_>,
    runner: &dyn BeamMacroRunner,
    module: Symbol,
    function: Symbol,
    args: &[TokenStream],
    span: Span,
) -> Result<TokenStream, ExpansionFailure> {
    let expn_data = ecx.current_expansion.id.expn_data();
    let def_site = ecx.with_def_site_ctxt(expn_data.def_site);
    let call_site = ecx.with_call_site_ctxt(expn_data.call_site);
    let mixed_site = ecx.with_mixed_site_ctxt(expn_data.call_site);
    let mut table = SpanTable::new(call_site, def_site, mixed_site);

    let args =
        args.iter().map(|arg| encode_tokens(arg, &mut table)).collect::<Result<Vec<_>, _>>()?;

    let result = runner.call(module.as_str(), function.as_str(), &args)?;

    let stream = match decode_tokens(&result, &table, ecx.psess())? {
        DecodeResult::Normal(stream) => stream,
        DecodeResult::HadUnknownSpan(stream) => {
            ecx.dcx().emit_warn(errors::ProcMacroUnknownSpan { span });
            stream
        }
    };

    Ok(stream)
}

pub struct BeamBangProcMacro {
    pub module: Symbol,
    pub function: Symbol,
    pub runner: Arc<dyn BeamMacroRunner>,
}

impl base::BangProcMacro for BeamBangProcMacro {
    fn expand(
        &self,
        ecx: &mut ExtCtxt<'_>,
        span: Span,
        input: TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        run(ecx, self.runner.as_ref(), self.module, self.function, &[input], span)
            .map_err(|failure| emit_expansion_failure(ecx, MacroKind::Bang, span, failure))
    }
}

pub struct BeamAttrProcMacro {
    pub module: Symbol,
    pub function: Symbol,
    pub runner: Arc<dyn BeamMacroRunner>,
}

impl base::AttrProcMacro for BeamAttrProcMacro {
    fn expand(
        &self,
        ecx: &mut ExtCtxt<'_>,
        span: Span,
        annotation: TokenStream,
        annotated: TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        run(ecx, self.runner.as_ref(), self.module, self.function, &[annotation, annotated], span)
            .map_err(|failure| emit_expansion_failure(ecx, MacroKind::Attr, span, failure))
    }
}

pub struct BeamDeriveProcMacro {
    pub module: Symbol,
    pub function: Symbol,
    pub runner: Arc<dyn BeamMacroRunner>,
}

impl MultiItemModifier for BeamDeriveProcMacro {
    fn expand(
        &self,
        ecx: &mut ExtCtxt<'_>,
        span: Span,
        _meta_item: &ast::MetaItem,
        item: Annotatable,
        _is_derive_const: bool,
    ) -> ExpandResult<Vec<Annotatable>, Annotatable> {
        // We need special handling for statement items
        // (e.g. `fn foo() { #[derive(Debug)] struct Bar; }`)
        let is_stmt = matches!(item, Annotatable::Stmt(..));

        let input = item.to_tokens();

        let invoc_id = ecx.current_expansion.id;

        let res = if ecx.sess.opts.incremental.is_some()
            && ecx.sess.opts.unstable_opts.cache_proc_macros
        {
            ty::tls::with(|tcx| {
                let input = &*tcx.arena.alloc(input);
                let key: (LocalExpnId, &TokenStream) = (invoc_id, input);
                QueryDeriveExpandCtx::enter(
                    ecx,
                    self.runner.clone(),
                    self.module,
                    self.function,
                    span,
                    || tcx.derive_macro_expansion(key).cloned(),
                )
            })
        } else {
            expand_derive_macro(ecx, self.runner.as_ref(), self.module, self.function, input, span)
        };

        let Ok(output) = res else {
            // error will already have been emitted
            return ExpandResult::Ready(vec![]);
        };

        let error_count_before = ecx.dcx().err_count();
        let mut parser = Parser::new(&ecx.sess.psess, output, Some("proc-macro derive"));
        let mut items = vec![];

        loop {
            match parser.parse_item(
                ForceCollect::No,
                if is_stmt { AllowConstBlockItems::No } else { AllowConstBlockItems::Yes },
            ) {
                Ok(None) => break,
                Ok(Some(item)) => {
                    if is_stmt {
                        items.push(Annotatable::Stmt(Box::new(ecx.stmt_item(span, item))));
                    } else {
                        items.push(Annotatable::Item(item));
                    }
                }
                Err(err) => {
                    err.emit();
                    break;
                }
            }
        }

        // fail if there have been errors emitted
        if ecx.dcx().err_count() > error_count_before {
            ecx.dcx().emit_err(errors::ProcMacroDeriveTokens { span });
        }

        ExpandResult::Ready(items)
    }
}

fn expand_derive_macro(
    ecx: &mut ExtCtxt<'_>,
    runner: &dyn BeamMacroRunner,
    module: Symbol,
    function: Symbol,
    input: TokenStream,
    span: Span,
) -> Result<TokenStream, ()> {
    // The `ErrorGuaranteed` is dropped here so that the result stays a
    // cacheable key for the `derive_macro_expansion` query.
    run(ecx, runner, module, function, &[input], span).map_err(|failure| {
        emit_expansion_failure(ecx, MacroKind::Derive, span, failure);
    })
}

/// Provide a query for computing the output of a derive macro.
pub(super) fn provide_derive_macro_expansion<'tcx>(
    tcx: TyCtxt<'tcx>,
    key: (LocalExpnId, &'tcx TokenStream),
) -> Result<&'tcx TokenStream, ()> {
    let (invoc_id, input) = key;

    // Make sure that we invalidate the query when the crate defining the proc macro changes
    let _ = tcx.crate_hash(invoc_id.expn_data().macro_def_id.unwrap().krate);
    QueryDeriveExpandCtx::with(|ecx, runner, module, function, span| {
        expand_derive_macro(ecx, runner, module, function, input.clone(), span)
            .map(|tokens| &*tcx.arena.alloc(tokens))
    })
}

/// Stores the context necessary to expand a derive proc macro via a query.
struct QueryDeriveExpandCtx {
    /// Type-erased version of `&mut ExtCtxt`
    expansion_ctx: *mut (),
    runner: Arc<dyn BeamMacroRunner>,
    module: Symbol,
    function: Symbol,
    span: Span,
}

impl QueryDeriveExpandCtx {
    fn enter<R>(
        ecx: &mut ExtCtxt<'_>,
        runner: Arc<dyn BeamMacroRunner>,
        module: Symbol,
        function: Symbol,
        span: Span,
        f: impl FnOnce() -> R,
    ) -> R {
        let ctx = Self { expansion_ctx: ecx as *mut _ as *mut (), runner, module, function, span };
        DERIVE_EXPAND_CTX.set(&ctx, f)
    }

    fn with<R>(
        f: impl for<'a, 'b> FnOnce(&'b mut ExtCtxt<'a>, &dyn BeamMacroRunner, Symbol, Symbol, Span) -> R,
    ) -> R {
        DERIVE_EXPAND_CTX.with(|ctx| {
            let ectx = {
                let casted = ctx.expansion_ctx.cast::<ExtCtxt<'_>>();
                // SAFETY: We can only get the value from `with` while the `enter` function
                // is active (on the callstack), and that function's signature ensures that the
                // lifetime is valid.
                // If `with` is called at some other time, it will panic due to usage of
                // `scoped_tls::with`.
                unsafe { casted.as_mut().unwrap() }
            };

            f(ectx, ctx.runner.as_ref(), ctx.module, ctx.function, ctx.span)
        })
    }
}

// When we invoke a query to expand a derive proc macro, we need to provide it with the expansion
// context and runner. We do that using a thread-local.
scoped_tls::scoped_thread_local!(static DERIVE_EXPAND_CTX: QueryDeriveExpandCtx);
