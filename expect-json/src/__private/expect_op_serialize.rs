use crate::expect_op::ExpectOp;

#[doc(hidden)]
#[typetag::serde(tag = "type")]
pub trait ExpectOpSerialize: ExpectOp {}
