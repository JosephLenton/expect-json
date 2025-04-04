use crate::expects::ops::ContainsNot;
use crate::expects::ExpectOp;
use serde_json::Value;

#[derive(Copy, Clone, Debug)]
pub struct ExpectNot;

impl ExpectNot {
    pub fn contains<V>(self, values: V) -> ExpectOp<ContainsNot>
    where
        V: Into<Value>,
    {
        ExpectOp::new(ContainsNot::new(values))
    }
}
