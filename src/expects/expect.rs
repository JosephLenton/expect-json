use crate::expects::ops::Contains;
use crate::expects::ExpectOp;
use serde_json::Value;

#[derive(Copy, Clone, Debug)]
pub struct Expect;

impl Expect {
    pub fn contains<V>(self, values: V) -> ExpectOp<Contains>
    where
        V: Into<Value>,
    {
        ExpectOp::new(Contains::new(values))
    }
}
