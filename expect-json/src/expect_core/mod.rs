//!
//! This module contains everything for implementing your own expectations.
//!
//! Implementing your own expectation involves working with three main types:
//!
//!  - The [`ExpectOp`] trait which implements the checks.
//!  - The [`expect_op`] macro which wires up internals to make expectations work.
//!  - A [`Context`] type for holding internal data (for managing things like paths of the checks).
//!
//! See the [`ExpectOp`] trait for an example on implementing an expectation.
//!

mod context;
pub use self::context::*;

mod expect_magic_id;
pub(crate) use self::expect_magic_id::*;

mod expect_op;
pub use self::expect_op::*;

mod expect_op_error;
pub use self::expect_op_error::*;

pub use ::expect_json_macros::*;
