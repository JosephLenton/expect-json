#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub(crate) mod internals;

#[allow(non_upper_case_globals)]
pub const expect: Expect = Expect;

mod expects;
pub use self::expects::*;

mod expect_json_eq_error;
pub use self::expect_json_eq_error::*;

mod expect_json_eq;
pub use self::expect_json_eq::*;
