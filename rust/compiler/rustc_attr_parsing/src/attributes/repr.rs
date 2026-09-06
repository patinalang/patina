use rustc_hir::attrs::{ErlTagStyle, ReprAttr};

use super::prelude::*;
use crate::session_diagnostics;

/// Parse #[repr(...)] forms.
///
/// Valid repr contents are Patina's BEAM representation hints, plus `int` for
/// enums with literal discriminants.
// FIXME(jdonszelmann): is a vec the right representation here even? isn't it just a struct?
pub(crate) struct ReprParser;

impl CombineAttributeParser for ReprParser {
    type Item = (ReprAttr, Span);
    const PATH: &[Symbol] = &[sym::repr];
    const CONVERT: ConvertFn<Self::Item> =
        |items, first_span| AttributeKind::Repr { reprs: items, first_span };
    // FIXME(jdonszelmann): never used
    const TEMPLATE: AttributeTemplate = template!(List: &[
        "Patina",
        "Elixir",
        "tagged",
        "atom",
        "transparent",
        "dynamic_wrap",
        "native_record",
        "tag = \"...\"",
        "tag_style = \"snake_case\"",
        "record_module = \"...\"",
        "int",
    ]);

    fn extend(
        cx: &mut AcceptContext<'_, '_>,
        args: &ArgParser,
    ) -> impl IntoIterator<Item = Self::Item> {
        let mut reprs = Vec::new();

        let Some(list) = cx.expect_list(args, cx.attr_span) else {
            return reprs;
        };

        if list.is_empty() {
            let attr_span = cx.attr_span;
            cx.adcx().warn_empty_attribute(attr_span);
            return reprs;
        }

        for param in list.mixed() {
            if let Some(_) = param.lit() {
                cx.emit_err(session_diagnostics::ReprIdent { span: cx.attr_span });
                continue;
            }

            reprs.extend(
                param.meta_item().and_then(|mi| parse_repr(cx, &mi)).map(|r| (r, param.span())),
            );
        }

        reprs
    }

    //FIXME Still checked fully in `check_attr.rs`
    //This one is slightly more complicated because the allowed targets depend on the arguments
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(ALL_TARGETS);
}

fn parse_repr(cx: &mut AcceptContext<'_, '_>, param: &MetaItemParser) -> Option<ReprAttr> {
    use ReprAttr::*;

    let name = param.path().word().map(|ident| ident.name);

    let args = param.args();

    match (name, args) {
        (Some(sym::Patina), ArgParser::NoArgs) => Some(Patina),
        (Some(sym::Elixir), ArgParser::NoArgs) => Some(Elixir),
        (Some(sym::tagged), ArgParser::NoArgs) => Some(Tagged),
        (Some(sym::atom), ArgParser::NoArgs) => Some(Atom),
        (Some(sym::transparent), ArgParser::NoArgs) => Some(Transparent),
        (Some(sym::dynamic_wrap), ArgParser::NoArgs) => Some(DynamicWrap),
        (Some(sym::native_record), ArgParser::NoArgs) => Some(NativeRecord),
        (Some(sym::tag), ArgParser::NameValue(value)) => {
            repr_string_value(cx, value).map(|value| Tag(value))
        }
        (Some(sym::tag_style), ArgParser::NameValue(value)) => match repr_string_value(cx, value) {
            Some(sym::snake_case) => Some(TagStyle(ErlTagStyle::SnakeCase)),
            Some(_) => {
                cx.emit_err(session_diagnostics::UnrecognizedReprHint { span: param.span() });
                None
            }
            None => None,
        },
        (Some(sym::record_module), ArgParser::NameValue(value)) => {
            repr_string_value(cx, value).map(|value| RecordModule(value))
        }
        (Some(sym::int), ArgParser::NoArgs) => Some(ReprInt),

        (
            Some(
                name @ sym::Patina
                | name @ sym::Elixir
                | name @ sym::tagged
                | name @ sym::atom
                | name @ sym::transparent
                | name @ sym::dynamic_wrap
                | name @ sym::native_record
                | name @ sym::int,
            ),
            ArgParser::NameValue(_),
        ) => {
            cx.emit_err(session_diagnostics::InvalidReprHintNoValue { span: param.span(), name });
            None
        }
        (
            Some(
                name @ sym::Patina
                | name @ sym::Elixir
                | name @ sym::tagged
                | name @ sym::atom
                | name @ sym::transparent
                | name @ sym::dynamic_wrap
                | name @ sym::native_record
                | name @ sym::int,
            ),
            ArgParser::List(_),
        ) => {
            cx.emit_err(session_diagnostics::InvalidReprHintNoParen { span: param.span(), name });
            None
        }

        _ => {
            cx.emit_err(session_diagnostics::UnrecognizedReprHint { span: param.span() });
            None
        }
    }
}

fn repr_string_value(cx: &mut AcceptContext<'_, '_>, value: &NameValueParser) -> Option<Symbol> {
    let Some(value) = value.value_as_str() else {
        cx.adcx().expected_string_literal(value.value_span, Some(value.value_as_lit()));
        return None;
    };
    Some(value)
}
