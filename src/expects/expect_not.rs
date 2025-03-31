use serde_json::Value;
use crate::expects::Contains;

#[derive(Copy, Clone, Debug)]
pub struct ExpectNot;

impl ExpectNot {
    pub fn contains<I, V>(self, values: I) -> Contains
    where
        I: IntoIterator<Item = V>,
        V: Into<Value>
    {
        Contains::new(values)
    }
}
