use super::ArrayType;
use super::BooleanType;
use super::NumberType;
use super::ObjectType;
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
    Array(ArrayType),
    Object(ObjectType),
}

impl ValueType {
    pub fn type_of(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::String(inner) => Self::String(inner.into()),
            Value::Number(n) => Self::Number(n.into()),
            Value::Bool(inner) => Self::Boolean(inner.into()),
            Value::Array(inner) => Self::Array(inner.into()),
            Value::Object(inner) => Self::Object(inner.into()),
        }
    }
}

impl From<ArrayType> for ValueType {
    fn from(inner: ArrayType) -> Self {
        Self::Array(inner)
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

impl From<ObjectType> for ValueType {
    fn from(inner: ObjectType) -> Self {
        Self::Object(inner)
    }
}

impl Display for ValueType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Null => write!(formatter, "null"),
            Self::String(inner) => write!(formatter, "{inner}"),
            Self::Number(n) => write!(formatter, "{n}"),
            Self::Boolean(inner) => write!(formatter, "{inner}"),
            Self::Array(inner) => write!(formatter, "{inner}"),
            Self::Object(inner) => write!(formatter, "{inner}"),
        }
    }
}
