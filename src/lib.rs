//! This is leafslug

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![warn(
    rust_2018_idioms,
    clippy::pedantic,
    clippy::perf,
    clippy::cargo,
    clippy::clone_on_ref_ptr,
    clippy::alloc_instead_of_core,
    clippy::default_numeric_fallback,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rc_buffer,
    clippy::as_ptr_cast_mut,
    clippy::as_underscore,
    clippy::panic_in_result_fn,
    clippy::multiple_inherent_impl,
    clippy::map_err_ignore,
    clippy::if_then_some_else_none,
    clippy::clone_on_ref_ptr,
    clippy::empty_structs_with_brackets,
    clippy::useless_let_if_seq,
    clippy::use_self,
    clippy::missing_const_for_fn,
    clippy::cognitive_complexity,
    clippy::self_named_constructors,
    clippy::cloned_instead_of_copied,
    clippy::iter_cloned_collect,
    clippy::implicit_clone,
    clippy::map_clone
)]
#![allow(clippy::multiple_crate_versions)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod domain;
pub use domain::*;
pub mod persistence;
pub use persistence::*;
pub mod http;
pub use http::*;
pub mod conf;
pub use conf::*;
