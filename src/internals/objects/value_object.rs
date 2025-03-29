use super::ArrayObject;
use super::BooleanObject;
use super::NullObject;
use super::NumberObject;
use super::ObjectObject;
use super::StringObject;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueObject {
    Null(NullObject),
    String(StringObject),
    Number(NumberObject),
    Boolean(BooleanObject),
    Array(ArrayObject),
    Object(ObjectObject),
}

impl From<Value> for ValueObject {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null(NullObject),
            Value::String(inner) => Self::String(inner.into()),
            Value::Number(n) => Self::Number(n.into()),
            Value::Bool(inner) => Self::Boolean(inner.into()),
            Value::Array(inner) => Self::Array(inner.into()),
            Value::Object(inner) => Self::Object(inner.into()),
        }
    }
}

impl From<NullObject> for ValueObject {
    fn from(inner: NullObject) -> Self {
        Self::Null(inner)
    }
}

impl From<ArrayObject> for ValueObject {
    fn from(inner: ArrayObject) -> Self {
        Self::Array(inner)
    }
}

impl From<BooleanObject> for ValueObject {
    fn from(inner: BooleanObject) -> Self {
        Self::Boolean(inner)
    }
}

impl From<StringObject> for ValueObject {
    fn from(inner: StringObject) -> Self {
        Self::String(inner)
    }
}

impl From<NumberObject> for ValueObject {
    fn from(inner: NumberObject) -> Self {
        Self::Number(inner)
    }
}

impl From<ObjectObject> for ValueObject {
    fn from(inner: ObjectObject) -> Self {
        Self::Object(inner)
    }
}

impl Display for ValueObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Null(inner) => write!(formatter, "{inner}"),
            Self::String(inner) => write!(formatter, "{inner}"),
            Self::Number(inner) => write!(formatter, "{inner}"),
            Self::Boolean(inner) => write!(formatter, "{inner}"),
            Self::Array(inner) => write!(formatter, "{inner}"),
            Self::Object(inner) => write!(formatter, "{inner}"),
        }
    }
}
