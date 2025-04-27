use crate::ExpectOp;

#[doc(hidden)]
#[typetag::serde(tag = "type")]
pub trait ExpectOpSerialize: ExpectOp {}
