use super::prelude::*;

pub(crate) struct ExternCallParser;

impl NoArgsAttributeParser for ExternCallParser {
    const PATH: &[Symbol] = &[sym::extern_call];

    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Expression)]);

    const CREATE: fn(Span) -> AttributeKind = AttributeKind::ExternCall;
}
