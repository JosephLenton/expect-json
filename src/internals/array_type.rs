use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayType(Vec<Value>);

impl From<Vec<Value>> for ArrayType {
    fn from(inner: Vec<Value>) -> Self {
        Self(inner)
    }
}

impl Display for ArrayType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, r#"array {:?}"#, self.0)
    }
}
