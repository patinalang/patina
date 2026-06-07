//! Errors emitted by codegen_ssa

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use rustc_errors::{
    Diag, DiagCtxtHandle, DiagSymbolList, Diagnostic, EmissionGuarantee, Level, msg,
};
use rustc_macros::{Diagnostic, Subdiagnostic};
use rustc_span::Span;

#[derive(Diagnostic)]
// Public for ArchiveBuilderBuilder::extract_bundled_libs
pub enum ExtractBundledLibsError<'a> {
    #[diag("failed to open file '{$rlib}': {$error}")]
    OpenFile { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag("failed to mmap file '{$rlib}': {$error}")]
    MmapFile { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag("failed to parse archive '{$rlib}': {$error}")]
    ParseArchive { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag("failed to read entry '{$rlib}': {$error}")]
    ReadEntry { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag("failed to get data from archive member '{$rlib}': {$error}")]
    ArchiveMember { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag("failed to convert name '{$rlib}': {$error}")]
    ConvertName { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag("failed to write file '{$rlib}': {$error}")]
    WriteFile { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag("failed to write file '{$rlib}': {$error}")]
    ExtractSection { rlib: &'a Path, error: Box<dyn std::error::Error> },
}

#[derive(Diagnostic)]
#[diag("failed to build archive at `{$path}`: {$error}")]
pub(crate) struct ArchiveBuildFailure {
    pub path: PathBuf,
    pub error: std::io::Error,
}

#[derive(Diagnostic)]
#[diag("don't know how to build archive of type: {$kind}")]
pub(crate) struct UnknownArchiveKind<'a> {
    pub kind: &'a str,
}

#[derive(Diagnostic)]
#[diag("archive `{$path}` was built as {$actual} format, but the target expects {$expected}")]
#[help(
    "this often occurs when using BSD-format archive tools on a Linux target; \
    rebuild the archive with the correct format for the target platform"
)]
pub(crate) struct IncompatibleArchiveFormat {
    pub path: PathBuf,
    pub actual: String,
    pub expected: String,
}

#[derive(Diagnostic)]
#[diag("`#[target_feature(..)]` cannot be applied to safe trait method")]
pub(crate) struct TargetFeatureSafeTrait {
    #[primary_span]
    #[label("cannot be applied to safe trait method")]
    pub span: Span,
    #[label("not an `unsafe` function")]
    pub def: Span,
}

#[derive(Diagnostic)]
#[diag("target feature `{$feature}` cannot be enabled with `#[target_feature]`: {$reason}")]
pub(crate) struct ForbiddenTargetFeatureAttr<'a> {
    #[primary_span]
    pub span: Span,
    pub feature: &'a str,
    pub reason: &'a str,
}

#[derive(Diagnostic)]
#[diag(
    "dlltool could not create import library with {$dlltool_path} {$dlltool_args}:
{$stdout}
{$stderr}"
)]
pub(crate) struct DlltoolFailImportLibrary<'a> {
    pub dlltool_path: Cow<'a, str>,
    pub dlltool_args: String,
    pub stdout: Cow<'a, str>,
    pub stderr: Cow<'a, str>,
}

#[derive(Diagnostic)]
#[diag("error writing .DEF file: {$error}")]
pub(crate) struct ErrorWritingDEFFile {
    pub error: std::io::Error,
}

#[derive(Diagnostic)]
#[diag("error calling dlltool '{$dlltool_path}': {$error}")]
pub(crate) struct ErrorCallingDllTool<'a> {
    pub dlltool_path: Cow<'a, str>,
    pub error: std::io::Error,
}

#[derive(Diagnostic)]
#[diag("error creating import library for {$lib_name}: {$error}")]
pub(crate) struct ErrorCreatingImportLibrary<'a> {
    pub lib_name: &'a str,
    pub error: String,
}

#[derive(Diagnostic)]
#[diag("enabling the `neon` target feature on the current target is unsound due to ABI issues")]
pub(crate) struct Aarch64SoftfloatNeon;

#[derive(Diagnostic)]
#[diag("unknown feature specified for `-Ctarget-feature`: `{$feature}`")]
#[note("features must begin with a `+` to enable or `-` to disable it")]
pub(crate) struct UnknownCTargetFeaturePrefix<'a> {
    pub feature: &'a str,
}

#[derive(Subdiagnostic)]
pub(crate) enum PossibleFeature<'a> {
    #[help("you might have meant: `{$rust_feature}`")]
    Some { rust_feature: &'a str },
    #[help("consider filing a feature request")]
    None,
}

#[derive(Diagnostic)]
#[diag("unknown and unstable feature specified for `-Ctarget-feature`: `{$feature}`")]
#[note(
    "it is still passed through to the codegen backend, but use of this feature might be unsound and the behavior of this feature can change in the future"
)]
pub(crate) struct UnknownCTargetFeature<'a> {
    pub feature: &'a str,
    #[subdiagnostic]
    pub rust_feature: PossibleFeature<'a>,
}

#[derive(Diagnostic)]
#[diag("unstable feature specified for `-Ctarget-feature`: `{$feature}`")]
#[note("this feature is not stably supported; its behavior can change in the future")]
pub(crate) struct UnstableCTargetFeature<'a> {
    pub feature: &'a str,
}

#[derive(Diagnostic)]
#[diag("target feature `{$feature}` cannot be {$enabled} with `-Ctarget-feature`: {$reason}")]
#[note(
    "this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!"
)]
#[note("for more information, see issue #116344 <https://github.com/rust-lang/rust/issues/116344>")]
pub(crate) struct ForbiddenCTargetFeature<'a> {
    pub feature: &'a str,
    pub enabled: &'a str,
    pub reason: &'a str,
}

pub(crate) struct TargetFeatureDisableOrEnable<'a> {
    pub features: &'a [&'a str],
    pub span: Option<Span>,
    pub missing_features: Option<MissingFeatures>,
}

#[derive(Subdiagnostic)]
#[help("add the missing features in a `target_feature` attribute")]
pub(crate) struct MissingFeatures;

impl<G: EmissionGuarantee> Diagnostic<'_, G> for TargetFeatureDisableOrEnable<'_> {
    fn into_diag(self, dcx: DiagCtxtHandle<'_>, level: Level) -> Diag<'_, G> {
        let mut diag = Diag::new(
            dcx,
            level,
            msg!("the target features {$features} must all be either enabled or disabled together"),
        );
        if let Some(span) = self.span {
            diag.span(span);
        };
        if let Some(missing_features) = self.missing_features {
            diag.subdiagnostic(missing_features);
        }
        diag.arg("features", self.features.join(", "));
        diag
    }
}

#[derive(Diagnostic)]
#[diag("the feature named `{$feature}` is not valid for this target")]
pub(crate) struct FeatureNotValid<'a> {
    pub feature: &'a str,
    #[primary_span]
    #[label("`{$feature}` is not valid for this target")]
    pub span: Span,
    #[subdiagnostic]
    pub hint: FeatureNotValidHint<'a>,
}

#[derive(Subdiagnostic)]
pub(crate) enum FeatureNotValidHint<'a> {
    #[suggestion(
        "consider removing the leading `+` in the feature name",
        code = "enable = \"{stripped}\"",
        applicability = "maybe-incorrect",
        style = "verbose"
    )]
    RemovePlusFromFeatureName {
        #[primary_span]
        span: Span,
        stripped: &'a str,
    },
    #[help(
        "valid names are: {$possibilities}{$and_more ->
            [0] {\"\"}
            *[other] {\" \"}and {$and_more} more
        }"
    )]
    ValidFeatureNames { possibilities: DiagSymbolList<&'a str>, and_more: usize },
}
