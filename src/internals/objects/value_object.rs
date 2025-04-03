use crate::internals::objects::ArrayObject;
use crate::internals::objects::BooleanObject;
use crate::internals::objects::FloatObject;
use crate::internals::objects::IntegerObject;
use crate::internals::objects::NullObject;
use crate::internals::objects::ObjectObject;
use crate::internals::objects::StringObject;
use serde_json::Number;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueObject {
    Null(NullObject),
    String(StringObject),
    Float(FloatObject),
    Integer(IntegerObject),
    Boolean(BooleanObject),
    Array(ArrayObject),
    Object(ObjectObject),
}

impl From<Value> for ValueObject {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null(NullObject),
            Value::String(inner) => Self::String(inner.into()),
            Value::Number(number) => number.into(),
            Value::Bool(inner) => Self::Boolean(inner.into()),
            Value::Array(inner) => Self::Array(inner.into()),
            Value::Object(inner) => Self::Object(inner.into()),
        }
    }
}

impl From<Number> for ValueObject {
    fn from(number: Number) -> Self {
        if number.is_f64() {
            let n = number
                .as_f64()
                .expect("Expected to convert serde_json::Number to f64");
            Self::Float(n.into())
        } else if number.is_u64() {
            let n = number
                .as_u64()
                .expect("Expected to convert serde_json::Number to u64");
            Self::Integer(n.into())
        } else {
            let n = number
                .as_i64()
                .expect("Expected to convert serde_json::Number to i64");
            Self::Integer(n.into())
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

impl From<FloatObject> for ValueObject {
    fn from(inner: FloatObject) -> Self {
        Self::Float(inner)
    }
}

impl From<IntegerObject> for ValueObject {
    fn from(inner: IntegerObject) -> Self {
        Self::Integer(inner)
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
            Self::Float(inner) => write!(formatter, "{inner}"),
            Self::Integer(inner) => write!(formatter, "{inner}"),
            Self::Boolean(inner) => write!(formatter, "{inner}"),
            Self::Array(inner) => write!(formatter, "{inner}"),
            Self::Object(inner) => write!(formatter, "{inner}"),
        }
    }
}
