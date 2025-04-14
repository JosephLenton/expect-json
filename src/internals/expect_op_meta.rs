use crate::expects::ExpectOp;
use crate::internals::types::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub struct ExpectOpMeta {
    pub name: &'static str,
    pub types: &'static [ValueType],
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
