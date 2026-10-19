//! Construction of MIR from HIR.

// tidy-alphabetical-start
#![feature(box_patterns)]
#![feature(try_blocks)]
// tidy-alphabetical-end

// The `builder` module used to be named `build`, but that was causing GitHub's
// "Go to file" feature to silently ignore all files in the module, probably
// because it assumes that "build" is a build-output directory. See #134365.
mod check_fnrefs;
mod check_mutability;
mod check_tail_calls;
mod check_unsafety;
mod errors;
pub mod thir;

use rustc_middle::util::Providers;

pub fn provide(providers: &mut Providers) {
    providers.queries.check_match = thir::pattern::check_match;
    providers.queries.lit_to_const = thir::constant::lit_to_const;
    providers.queries.eval_to_valtree = thir::constant::eval_to_valtree;
    providers.queries.eval_to_const_value_raw = thir::constant::eval_to_const_value_raw;
    providers.queries.valtree_to_const_val = thir::constant::valtree_to_const_value;
    providers.queries.check_mutability = check_mutability::check_mutability;
    providers.queries.check_fnrefs = check_fnrefs::check_fnrefs;
    providers.queries.check_unsafety = check_unsafety::check_unsafety;
    providers.queries.check_tail_calls = check_tail_calls::check_tail_calls;
    providers.queries.thir_body = thir::cx::thir_body;
}
