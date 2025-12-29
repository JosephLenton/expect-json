use crate::JsonType;
use crate::expect_core::ExpectOp;

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
            types: op.debug_supported_types(),
        }
    }
}
