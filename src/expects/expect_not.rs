use crate::expects::ops::ContainsNot;
use crate::expects::ExpectOpContainer;
use serde_json::Value;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExpectNot;

impl ExpectNot {
    pub fn contains<V>(self, values: V) -> ExpectOpContainer<ContainsNot>
    where
        V: Into<Value>,
    {
        ExpectOpContainer::new(ContainsNot::new(values))
    }
}
