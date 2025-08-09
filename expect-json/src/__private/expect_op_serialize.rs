use crate::expect_core::ExpectOp;

#[doc(hidden)]
#[typetag::serde(tag = "type")]
pub trait ExpectOpSerialize: ExpectOp {}
