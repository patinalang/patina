use super::prelude::*;

pub(crate) struct ErlModuleParser;

impl SingleAttributeParser for ErlModuleParser {
    const PATH: &[Symbol] = &[sym::module];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Crate),
        Allow(Target::Struct),
        Allow(Target::Enum),
        Allow(Target::Mod),
        Allow(Target::Trait),
    ]);
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: "name");

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        let nv = cx.expect_name_value(args, cx.attr_span, None)?;
        let Some(name) = nv.value_as_str() else {
            cx.adcx().expected_string_literal(nv.value_span, Some(nv.value_as_lit()));
            return None;
        };

        Some(AttributeKind::ErlModule(name, cx.attr_span))
    }
}
