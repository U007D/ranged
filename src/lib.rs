#![feature(bool_to_option, const_fn, const_generics, const_if_match, const_panic)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![allow(
    clippy::match_bool,
    incomplete_features,
    clippy::module_name_repetitions
)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod arith_helpers;
mod consts;
mod error;
mod ranged_i32;
pub use error::Error;
pub use ranged_i32::RangedI32;

pub type Result<T, E = Error> = std::result::Result<T, E>;
