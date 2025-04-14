use crate::expects::ops::Contains;
use crate::expects::ExpectOpContainer;
use serde_json::Value;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExpectNot;

impl ExpectNot {
    pub fn contains<V>(self, values: V) -> ExpectOpContainer<Contains>
    where
        V: Into<Value>,
    {
        ExpectOpContainer::new(Contains::new_not(values))
    }
}
