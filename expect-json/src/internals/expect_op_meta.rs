use crate::expect_op::ExpectOp;
use crate::JsonType;

#[derive(Debug, Clone, PartialEq)]
pub struct ExpectOpMeta {
    pub name: &'static str,
    pub types: &'static [JsonType],
}

impl ExpectOpMeta {
    pub fn new<O>(op: &O) -> Self
    where
        O: ExpectOp + ?Sized,
    {
        Self {
            name: op.name(),
            types: op.supported_types(),
        }
    }
}
