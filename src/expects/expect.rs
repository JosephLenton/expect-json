use super::ExpectOp;
use crate::expects::Contains;
use serde_json::Value;

#[derive(Copy, Clone, Debug)]
pub struct Expect;

impl Expect {
    pub fn contains<I, V>(self, values: I) -> ExpectOp<Contains>
    where
        I: IntoIterator<Item = V>,
        V: Into<Value>,
    {
        ExpectOp::new(Contains::new(values))
    }
}
