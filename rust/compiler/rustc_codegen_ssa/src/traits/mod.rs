//! Interface of a Rust codegen backend
//!
//! This crate defines all the traits that have to be implemented by a codegen backend in order to
//! use the backend-agnostic codegen code in `rustc_codegen_ssa`.
//!
//! The interface is designed around two backend-specific data structures, the codegen context and
//! the builder. The codegen context is supposed to be read-only after its creation and during the
//! actual codegen, while the builder stores the information about the function during codegen and
//! is used to produce the instructions of the backend IR.
//!
//! The traits contain associated types that are backend-specific, such as the backend's value or
//! basic blocks.

mod backend;

pub use self::backend::CodegenBackend;
