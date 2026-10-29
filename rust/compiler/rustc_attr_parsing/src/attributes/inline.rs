// FIXME(jdonszelmann): merge these two parsers and error when both attributes are present here.
//                      note: need to model better how duplicate attr errors work when not using
//                      SingleAttributeParser which is what we have two of here.

use rustc_hir::attrs::{AttributeKind, InlineAttr};

use super::prelude::*;

pub(crate) struct InlineParser;

impl SingleAttributeParser for InlineParser {
    const PATH: &[Symbol] = &[sym::inline];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::WarnButFutureError;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Fn),
        Allow(Target::Method(MethodKind::Inherent)),
        Allow(Target::Method(MethodKind::Trait { body: true })),
        Allow(Target::Method(MethodKind::TraitImpl)),
    ]);
    const TEMPLATE: AttributeTemplate = template!(Word);

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        match args {
            ArgParser::NoArgs => Some(AttributeKind::Inline(InlineAttr::Hint, cx.attr_span)),
            ArgParser::List(list) => {
                cx.adcx().expected_no_args(list.span);
                None
            }
            ArgParser::NameValue(_) => {
                let span = cx.attr_span;
                cx.adcx().expected_no_args(span);
                None
            }
        }
    }
}

pub(crate) struct RustcForceInlineParser;

impl SingleAttributeParser for RustcForceInlineParser {
    const PATH: &[Symbol] = &[sym::rustc_force_inline];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Fn),
        Allow(Target::Method(MethodKind::Inherent)),
    ]);

    const TEMPLATE: AttributeTemplate = template!(Word, List: &["reason"], NameValueStr: "reason");

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        let reason = match args {
            ArgParser::NoArgs => None,
            ArgParser::List(list) => {
                let l = cx.expect_single(list)?;

                let Some(reason) = l.lit().and_then(|i| i.kind.str()) else {
                    cx.adcx().expected_string_literal(l.span(), l.lit());
                    return None;
                };

                Some(reason)
            }
            ArgParser::NameValue(v) => {
                let Some(reason) = v.value_as_str() else {
                    cx.adcx().expected_string_literal(v.value_span, Some(v.value_as_lit()));
                    return None;
                };

                Some(reason)
            }
        };

        Some(AttributeKind::Inline(
            InlineAttr::Force { attr_span: cx.attr_span, reason },
            cx.attr_span,
        ))
    }
}
