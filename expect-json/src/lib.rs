#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![allow(clippy::module_inception)]

mod context;
pub use self::context::*;

pub(crate) mod internals;

mod expects;
pub use self::expects::*;

pub use self::expects::expect;

mod expect_json_error;
pub use self::expect_json_error::*;

mod expect_json_eq;
pub use self::expect_json_eq::*;

mod json_type;
pub use self::json_type::*;

pub use ::expect_json_macros::*;

#[doc(hidden)]
pub mod __private;
