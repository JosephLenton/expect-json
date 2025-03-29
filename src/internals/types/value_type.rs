use crate::internals::objects::ArrayObject;
use crate::internals::objects::BooleanObject;
use crate::internals::objects::NullObject;
use crate::internals::objects::NumberObject;
use crate::internals::objects::ObjectObject;
use crate::internals::objects::StringObject;
use crate::internals::objects::ValueObject;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueType(ValueObject);

impl ValueType {
    pub fn type_of(value: Value) -> Self {
        match value {
            Value::Null => Self(NullObject.into()),
            Value::String(inner) => Self(StringObject::from(inner).into()),
            Value::Number(n) => Self(NumberObject::from(n).into()),
            Value::Bool(inner) => Self(BooleanObject::from(inner).into()),
            Value::Array(inner) => Self(ArrayObject::from(inner).into()),
            Value::Object(inner) => Self(ObjectObject::from(inner).into()),
        }
    }
}

impl From<ArrayObject> for ValueType {
    fn from(inner: ArrayObject) -> Self {
        Self(inner.into())
    }
}

impl From<BooleanObject> for ValueType {
    fn from(inner: BooleanObject) -> Self {
        Self(inner.into())
    }
}

impl From<StringObject> for ValueType {
    fn from(inner: StringObject) -> Self {
        Self(inner.into())
    }
}

impl From<NumberObject> for ValueType {
    fn from(inner: NumberObject) -> Self {
        Self(inner.into())
    }
}

impl From<ObjectObject> for ValueType {
    fn from(inner: ObjectObject) -> Self {
        Self(inner.into())
    }
}

impl Display for ValueType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            ValueObject::Null(inner) => write!(formatter, "{inner}"),
            ValueObject::String(inner) => write!(formatter, "string {inner}"),
            ValueObject::Number(inner) => match inner {
                NumberObject::Float(_) => write!(formatter, "float {inner}"),
                NumberObject::PositiveInteger(_) => write!(formatter, "integer {inner}"),
                NumberObject::NegativeInteger(_) => write!(formatter, "integer {inner}"),
            },
            ValueObject::Boolean(inner) => write!(formatter, "boolean {inner}"),
            ValueObject::Array(inner) => write!(formatter, "array {inner}"),
            ValueObject::Object(inner) => write!(formatter, "object {inner}"),
        }
    }
}
