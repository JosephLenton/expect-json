use crate::expects::Contains;
use serde_json::Value;

#[derive(Copy, Clone, Debug)]
pub struct Expect;

impl Expect {
    pub fn contains<I, V>(self, values: I) -> Contains
    where
        I: IntoIterator<Item = V>,
        V: Into<Value>,
    {
        Contains::new(values)
    }
}
