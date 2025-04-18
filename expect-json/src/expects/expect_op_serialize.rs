use crate::ExpectOp;

#[typetag::serde(tag = "type")]
pub trait ExpectOpSerialize: ExpectOp {}
