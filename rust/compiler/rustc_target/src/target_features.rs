//! Declares Rust's target feature names for each target.
//! Note that these are similar to but not always identical to LLVM's feature names,
//! and Rust adds some features that do not correspond to LLVM features at all.
use rustc_data_structures::fx::FxHashSet;
use rustc_macros::StableHash;
use rustc_span::Symbol;

use crate::spec::Target;

/// Features that control behaviour of rustc, rather than the codegen.
/// These exist globally and are not in the target-specific lists below.
pub const RUSTC_SPECIFIC_FEATURES: &[&str] = &["crt-static"];

/// Stability information for target features.
#[derive(Debug, Copy, Clone, StableHash)]
pub enum Stability {
    /// This target feature is stable, it can be used in `#[target_feature]` and
    /// `#[cfg(target_feature)]`.
    Stable,
    /// This target feature is unstable. It is only present in `#[cfg(target_feature)]` on
    /// nightly and using it in `#[target_feature]` requires enabling the given nightly feature.
    Unstable(
        /// This must be a *language* feature, or else rustc will ICE when reporting a missing
        /// feature gate!
        Symbol,
    ),
    /// This feature can not be set via `-Ctarget-feature` or `#[target_feature]`, it can only be
    /// set in the target spec. It is never set in `cfg(target_feature)`. Used in
    /// particular for features are actually ABI configuration flags (not all targets are as nice as
    /// RISC-V and have an explicit way to set the ABI separate from target features).
    Forbidden { reason: &'static str },
}

impl Stability {
    /// Returns whether the feature can be used in `cfg(target_feature)` ever.
    /// (It might still be nightly-only even if this returns `true`, so make sure to also check
    /// `requires_nightly`.)
    pub fn in_cfg(&self) -> bool {
        matches!(self, Stability::Stable | Stability::Unstable { .. })
    }

    /// Returns the nightly feature that is required to toggle this target feature via
    /// `#[target_feature]`/`-Ctarget-feature` or to test it via `cfg(target_feature)`.
    /// (For `cfg` we only care whether the feature is nightly or not, we don't require
    /// the feature gate to actually be enabled when using a nightly compiler.)
    ///
    /// Before calling this, ensure the feature is even permitted for this use:
    /// - for `#[target_feature]`/`-Ctarget-feature`, check `toggle_allowed()`
    /// - for `cfg(target_feature)`, check `in_cfg()`
    pub fn requires_nightly(&self) -> Option<Symbol> {
        match *self {
            Stability::Unstable(nightly_feature) => Some(nightly_feature),
            Stability::Stable { .. } => None,
            Stability::Forbidden { .. } => panic!("forbidden features should not reach this far"),
        }
    }

    /// Returns whether the feature may be toggled via `#[target_feature]` or `-Ctarget-feature`.
    /// (It might still be nightly-only even if this returns `true`, so make sure to also check
    /// `requires_nightly`.)
    pub fn toggle_allowed(&self) -> Result<(), &'static str> {
        match self {
            Stability::Unstable(_) | Stability::Stable { .. } => Ok(()),
            Stability::Forbidden { reason } => Err(reason),
        }
    }
}

// Here we list target features that rustc "understands": they can be used in `#[target_feature]`
// and `#[cfg(target_feature)]`. They also do not trigger any warnings when used with
// `-Ctarget-feature`.
//
// Note that even unstable (and even entirely unlisted) features can be used with `-Ctarget-feature`
// on stable. Using a feature not on the list of Rust target features only emits a warning.
// Only `cfg(target_feature)` and `#[target_feature]` actually do any stability gating.
// `cfg(target_feature)` for unstable features just works on nightly without any feature gate.
// `#[target_feature]` requires a feature gate.
//
// When adding features to the below lists
// check whether they're named already elsewhere in rust
// e.g. in stdarch and whether the given name matches LLVM's
// if it doesn't, to_llvm_feature in llvm_util in rustc_codegen_llvm needs to be adapted.
// Additionally, if the feature is not available in older version of LLVM supported by the current
// rust, the same function must be updated to filter out these features to avoid triggering
// warnings.
//
// Also note that all target features listed here must be purely additive: for target_feature 1.1 to
// be sound, we can never allow features like `+soft-float` (on x86) to be controlled on a
// per-function level, since we would then allow safe calls from functions with `+soft-float` to
// functions without that feature!
//
// It is important for soundness to consider the interaction of targets features and the function
// call ABI. For example, disabling the `x87` feature on x86 changes how scalar floats are passed as
// arguments, so letting people toggle that feature would be unsound. To this end, the
// `abi_required_features` function computes which target features must and must not be enabled for
// any given target, and individual features can also be marked as `Forbidden`.
// See https://github.com/rust-lang/rust/issues/116344 for some more context.
//
// The one exception to features that change the ABI is features that enable larger vector
// registers. Those are permitted to be listed here. The `*_FOR_CORRECT_VECTOR_ABI` arrays store
// information about which target feature is ABI-required for which vector size; this is used to
// ensure that vectors can only be passed via `extern "C"` when the right feature is enabled. (For
// the "Rust" ABI we generally pass vectors by-ref exactly to avoid these issues.)
// Also see https://github.com/rust-lang/rust/issues/116558.
//
// Stabilizing a target feature requires t-lang approval.
type ImpliedFeatures = &'static [&'static str];

/// When rustdoc is running, provide a list of all known features so that all their respective
/// primitives may be documented.
///
/// IMPORTANT: If you're adding another feature list above, make sure to add it to this iterator!
pub fn all_rust_features() -> impl Iterator<Item = (&'static str, Stability)> {
    RUSTC_SPECIFIC_FEATURES.iter().copied().map(|f| (f, Stability::Stable))
}

#[derive(Copy, Clone, Debug)]
pub struct FeatureConstraints {
    /// Features that must be enabled.
    pub required: &'static [&'static str],
    /// Features that must be disabled.
    pub incompatible: &'static [&'static str],
}

impl Target {
    pub fn rust_target_features(&self) -> &'static [(&'static str, Stability, ImpliedFeatures)] {
        &[]
    }

    pub fn features_for_correct_fixed_length_vector_abi(&self) -> &'static [(u64, &'static str)] {
        &[]
    }

    pub fn features_for_correct_scalable_vector_abi(&self) -> Option<&'static str> {
        None
    }

    pub fn tied_target_features(&self) -> &'static [&'static [&'static str]] {
        &[]
    }

    pub fn implied_target_features<'a>(&self, base_feature: &'a str) -> FxHashSet<&'a str> {
        let mut features = FxHashSet::default();
        features.insert(base_feature);
        features
    }

    pub fn abi_required_features(&self) -> FeatureConstraints {
        FeatureConstraints { required: &[], incompatible: &[] }
    }
}
