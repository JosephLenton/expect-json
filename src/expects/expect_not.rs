use crate::expects::ops::Contains;
use serde_json::Value;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExpectNot;

impl ExpectNot {
    pub fn contains<V>(self, values: V) -> Contains
    where
        V: Into<Value>,
    {
        Contains::new_not(values)
    }
}
