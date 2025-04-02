use super::ExpectOp;
use crate::expects::ops::ArrayContains;
use serde_json::Value;

#[derive(Copy, Clone, Debug)]
pub struct Expect;

impl Expect {
    pub fn contains<I, V>(self, values: I) -> ExpectOp<ArrayContains>
    where
        I: IntoIterator<Item = V>,
        V: Into<Value>,
    {
        ExpectOp::new(ArrayContains::new(values))
    }
}
