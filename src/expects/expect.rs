use crate::expects::ops::Contains;
use crate::expects::ExpectOp;
use crate::ExpectNot;
use serde_json::Value;

#[derive(Copy, Clone, Debug)]
pub struct Expect {
    pub not: ExpectNot,
}

impl Expect {
    pub(crate) const fn new() -> Self {
        Self { not: ExpectNot }
    }

    pub fn contains<V>(self, values: V) -> ExpectOp<Contains>
    where
        V: Into<Value>,
    {
        ExpectOp::new(Contains::new(values))
    }
}
