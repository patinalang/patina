use rustc_hir::attrs::WindowsSubsystemKind;
use rustc_span::Symbol;

use super::prelude::*;

pub(crate) struct RecursionLimitParser;

impl SingleAttributeParser for RecursionLimitParser {
    const PATH: &[Symbol] = &[sym::recursion_limit];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::WarnButFutureError;
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: "N", "https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute");
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        let nv = cx.expect_name_value(args, cx.attr_span, None)?;

        Some(AttributeKind::RecursionLimit { limit: cx.parse_limit_int(nv)? })
    }
}

pub(crate) struct MoveSizeLimitParser;

impl SingleAttributeParser for MoveSizeLimitParser {
    const PATH: &[Symbol] = &[sym::move_size_limit];
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: "N");
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        let nv = cx.expect_name_value(args, cx.attr_span, None)?;

        Some(AttributeKind::MoveSizeLimit { limit: cx.parse_limit_int(nv)? })
    }
}

pub(crate) struct TypeLengthLimitParser;

impl SingleAttributeParser for TypeLengthLimitParser {
    const PATH: &[Symbol] = &[sym::type_length_limit];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::WarnButFutureError;
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: "N");
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        let nv = cx.expect_name_value(args, cx.attr_span, None)?;

        Some(AttributeKind::TypeLengthLimit { limit: cx.parse_limit_int(nv)? })
    }
}

pub(crate) struct PatternComplexityLimitParser;

impl SingleAttributeParser for PatternComplexityLimitParser {
    const PATH: &[Symbol] = &[sym::pattern_complexity_limit];
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: "N");
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        let nv = cx.expect_name_value(args, cx.attr_span, None)?;

        Some(AttributeKind::PatternComplexityLimit { limit: cx.parse_limit_int(nv)? })
    }
}

pub(crate) struct NoCoreParser;

impl NoArgsAttributeParser for NoCoreParser {
    const PATH: &[Symbol] = &[sym::no_core];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::NoCore;
}

pub(crate) struct NoStdParser;

impl NoArgsAttributeParser for NoStdParser {
    const PATH: &[Symbol] = &[sym::no_std];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::NoStd;
}

pub(crate) struct NoMainParser;

impl NoArgsAttributeParser for NoMainParser {
    const PATH: &[Symbol] = &[sym::no_main];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::NoMain;
}

pub(crate) struct RustcCoherenceIsCoreParser;

impl NoArgsAttributeParser for RustcCoherenceIsCoreParser {
    const PATH: &[Symbol] = &[sym::rustc_coherence_is_core];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::RustcCoherenceIsCore;
}

pub(crate) struct WindowsSubsystemParser;

impl SingleAttributeParser for WindowsSubsystemParser {
    const PATH: &[Symbol] = &[sym::windows_subsystem];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::WarnButFutureError;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: ["windows", "console"], "https://doc.rust-lang.org/reference/runtime.html#the-windows_subsystem-attribute");

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        let nv = cx.expect_name_value(args, cx.inner_span, Some(sym::windows_subsystem))?;

        let kind = match nv.value_as_str() {
            Some(sym::console) => WindowsSubsystemKind::Console,
            Some(sym::windows) => WindowsSubsystemKind::Windows,
            Some(_) | None => {
                cx.adcx().expected_specific_argument_strings(
                    nv.value_span,
                    &[sym::console, sym::windows],
                );
                return None;
            }
        };

        Some(AttributeKind::WindowsSubsystem(kind))
    }
}

pub(crate) struct PanicRuntimeParser;

impl NoArgsAttributeParser for PanicRuntimeParser {
    const PATH: &[Symbol] = &[sym::panic_runtime];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::PanicRuntime;
}

pub(crate) struct NeedsPanicRuntimeParser;

impl NoArgsAttributeParser for NeedsPanicRuntimeParser {
    const PATH: &[Symbol] = &[sym::needs_panic_runtime];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::NeedsPanicRuntime;
}

pub(crate) struct ProfilerRuntimeParser;

impl NoArgsAttributeParser for ProfilerRuntimeParser {
    const PATH: &[Symbol] = &[sym::profiler_runtime];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::ProfilerRuntime;
}

pub(crate) struct NoBuiltinsParser;

impl NoArgsAttributeParser for NoBuiltinsParser {
    const PATH: &[Symbol] = &[sym::no_builtins];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::NoBuiltins;
}

pub(crate) struct RustcPreserveUbChecksParser;

impl NoArgsAttributeParser for RustcPreserveUbChecksParser {
    const PATH: &[Symbol] = &[sym::rustc_preserve_ub_checks];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::RustcPreserveUbChecks;
}

pub(crate) struct RustcNoImplicitBoundsParser;

impl NoArgsAttributeParser for RustcNoImplicitBoundsParser {
    const PATH: &[Symbol] = &[sym::rustc_no_implicit_bounds];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::RustcNoImplicitBounds;
}

pub(crate) struct DefaultLibAllocatorParser;

impl NoArgsAttributeParser for DefaultLibAllocatorParser {
    const PATH: &[Symbol] = &[sym::default_lib_allocator];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::DefaultLibAllocator;
}

pub(crate) struct FeatureParser;

impl CombineAttributeParser for FeatureParser {
    const PATH: &[Symbol] = &[sym::feature];
    type Item = Ident;
    const CONVERT: ConvertFn<Self::Item> = AttributeKind::Feature;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const TEMPLATE: AttributeTemplate = template!(List: &["feature1, feature2, ..."]);

    fn extend(
        cx: &mut AcceptContext<'_, '_>,
        args: &ArgParser,
    ) -> impl IntoIterator<Item = Self::Item> {
        let Some(list) = cx.expect_list(args, cx.attr_span) else {
            return Vec::new();
        };

        if list.is_empty() {
            let attr_span = cx.attr_span;
            cx.adcx().warn_empty_attribute(attr_span);
        }

        let mut res = Vec::new();

        for elem in list.mixed() {
            let Some(elem) = elem.meta_item() else {
                cx.adcx().expected_identifier(elem.span());
                continue;
            };
            let Some(()) = cx.expect_no_args(elem.args()) else {
                continue;
            };
            let path = elem.path();
            let Some(ident) = path.word() else {
                cx.adcx().expected_identifier(path.span());
                continue;
            };
            res.push(ident);
        }

        res
    }
}

pub(crate) struct RegisterToolParser;

impl CombineAttributeParser for RegisterToolParser {
    const PATH: &[Symbol] = &[sym::register_tool];
    type Item = Ident;
    const CONVERT: ConvertFn<Self::Item> = |tools, _span| AttributeKind::RegisterTool(tools);
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(ALL_TARGETS);
    const TEMPLATE: AttributeTemplate = template!(List: &["tool1, tool2, ..."]);

    fn extend(
        cx: &mut AcceptContext<'_, '_>,
        args: &ArgParser,
    ) -> impl IntoIterator<Item = Self::Item> {
        let Some(list) = cx.expect_list(args, cx.attr_span) else {
            return Vec::new();
        };

        if list.is_empty() {
            let attr_span = cx.attr_span;
            cx.adcx().warn_empty_attribute(attr_span);
        }

        let mut res = Vec::new();

        for elem in list.mixed() {
            let Some(elem) = elem.meta_item() else {
                cx.adcx().expected_identifier(elem.span());
                continue;
            };
            let Some(()) = cx.expect_no_args(elem.args()) else {
                continue;
            };

            let path = elem.path();
            let Some(ident) = path.word() else {
                cx.adcx().expected_identifier(path.span());
                continue;
            };

            res.push(ident);
        }

        res
    }
}
