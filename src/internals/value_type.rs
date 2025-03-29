use super::BooleanType;
use super::NumberType;
use super::StringType;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Null,
    String(StringType),
    Number(NumberType),
    Boolean(BooleanType),
    Array,
    Object,
}

impl ValueType {
    pub fn type_of(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::String(inner) => Self::String(inner.into()),
            Value::Number(n) => Self::Number(n.into()),
            Value::Bool(inner) => Self::Boolean(inner.into()),
            Value::Array(_) => Self::Array,
            Value::Object(_) => Self::Object,
        }
    }
}

impl From<BooleanType> for ValueType {
    fn from(inner: BooleanType) -> Self {
        Self::Boolean(inner)
    }
}

impl From<StringType> for ValueType {
    fn from(inner: StringType) -> Self {
        Self::String(inner)
    }
}

impl From<NumberType> for ValueType {
    fn from(inner: NumberType) -> Self {
        Self::Number(inner)
    }
}

impl Display for ValueType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Null => write!(formatter, "null"),
            Self::String(inner) => write!(formatter, "{inner}"),
            Self::Number(n) => write!(formatter, "{n}"),
            Self::Boolean(inner) => write!(formatter, "{inner}"),
            Self::Array => write!(formatter, "array"),
            Self::Object => write!(formatter, "object"),
        }
    }
}
