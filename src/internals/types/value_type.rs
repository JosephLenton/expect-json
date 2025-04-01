use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValueType {
    Null,
    String,
    Number,
    Boolean,
    Array,
    Object,
}

impl From<&Value> for ValueType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::String(_) => Self::String,
            Value::Number(_) => Self::Number,
            Value::Bool(_) => Self::Boolean,
            Value::Array(_) => Self::Array,
            Value::Object(_) => Self::Object,
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::Null => write!(formatter, "null"),
            Self::String => write!(formatter, "string"),
            Self::Number => write!(formatter, "number"),
            Self::Boolean => write!(formatter, "boolean"),
            Self::Array => write!(formatter, "array"),
            Self::Object => write!(formatter, "object"),
        }
    }
}
