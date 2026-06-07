//! Various checks
//!
//! # Note
//!
//! This API is completely unstable and subject to change.

// tidy-alphabetical-start
#![feature(associated_type_defaults)]
#![feature(box_patterns)]
#![feature(never_type)]
// tidy-alphabetical-end

use rustc_middle::query::Providers;

mod assoc;
mod common_traits;
mod consts;
mod errors;
mod implied_bounds;
mod instance;
mod needs_drop;
mod nested_bodies;
mod opaque_types;
mod representability;
pub mod sig_types;
mod structural_match;
mod ty;

pub fn provide(providers: &mut Providers) {
    assoc::provide(providers);
    common_traits::provide(providers);
    consts::provide(providers);
    implied_bounds::provide(providers);
    needs_drop::provide(providers);
    opaque_types::provide(providers);
    representability::provide(providers);
    ty::provide(providers);
    instance::provide(providers);
    structural_match::provide(providers);
    nested_bodies::provide(providers);
}
