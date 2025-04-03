use serde_json::Number;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValueType {
    Null,
    String,
    Float,
    Integer,
    Boolean,
    Array,
    Object,
}

impl From<&Value> for ValueType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::String(_) => Self::String,
            Value::Number(number) => number.into(),
            Value::Bool(_) => Self::Boolean,
            Value::Array(_) => Self::Array,
            Value::Object(_) => Self::Object,
        }
    }
}

impl From<&Number> for ValueType {
    fn from(number: &Number) -> Self {
        if number.is_f64() {
            ValueType::Float
        } else {
            ValueType::Integer
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::Null => write!(formatter, "null"),
            Self::String => write!(formatter, "string"),
            Self::Float => write!(formatter, "float"),
            Self::Integer => write!(formatter, "integer"),
            Self::Boolean => write!(formatter, "boolean"),
            Self::Array => write!(formatter, "array"),
            Self::Object => write!(formatter, "object"),
        }
    }
}
