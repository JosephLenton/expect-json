//!
//! This module contains everything for implementing your own expectations.
//!

mod context;
pub use self::context::*;

mod expect_magic_id;
pub(crate) use self::expect_magic_id::*;

mod expect_op;
pub use self::expect_op::*;

mod expect_op_error;
pub use self::expect_op_error::*;
