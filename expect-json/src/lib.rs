#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![allow(clippy::module_inception)]

// pub(crate) mod internals;
pub mod internals;

#[allow(non_upper_case_globals)]
pub const expect: Expect = Expect::new();

mod expects;
pub use self::expects::*;

mod expect_json_eq_error;
pub use self::expect_json_eq_error::*;

mod expect_json_eq;
pub use self::expect_json_eq::*;

mod json_type;
pub use self::json_type::*;

pub use ::expect_json_macros::*;

#[doc(hidden)]
pub mod __private;
