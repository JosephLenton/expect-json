#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::module_inception)]

pub(crate) mod internals;

pub mod expect;
pub mod expect_core;

mod expect_json_error;
pub use self::expect_json_error::*;

mod expect_json_eq;
pub use self::expect_json_eq::*;

mod json_integer;
pub use self::json_integer::*;

mod json_type;
pub use self::json_type::*;

#[doc(hidden)]
pub mod __private;
