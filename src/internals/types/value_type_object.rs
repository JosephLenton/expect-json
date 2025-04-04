use crate::internals::objects::ArrayObject;
use crate::internals::objects::BooleanObject;
use crate::internals::objects::FloatObject;
use crate::internals::objects::IntegerObject;
use crate::internals::objects::NullObject;
use crate::internals::objects::ObjectObject;
use crate::internals::objects::StringObject;
use crate::internals::objects::ValueObject;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueTypeObject(ValueObject);

impl ValueTypeObject {
    pub fn is_number(&self) -> bool {
        self.0.is_number()
    }
}

impl From<Value> for ValueTypeObject {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self(NullObject.into()),
            Value::String(inner) => Self(StringObject::from(inner).into()),
            Value::Number(inner) => Self(inner.into()),
            Value::Bool(inner) => Self(BooleanObject::from(inner).into()),
            Value::Array(inner) => Self(ArrayObject::from(inner).into()),
            Value::Object(inner) => Self(ObjectObject::from(inner).into()),
        }
    }
}

impl From<ArrayObject> for ValueTypeObject {
    fn from(inner: ArrayObject) -> Self {
        Self(inner.into())
    }
}

impl From<BooleanObject> for ValueTypeObject {
    fn from(inner: BooleanObject) -> Self {
        Self(inner.into())
    }
}

impl From<StringObject> for ValueTypeObject {
    fn from(inner: StringObject) -> Self {
        Self(inner.into())
    }
}

impl From<FloatObject> for ValueTypeObject {
    fn from(inner: FloatObject) -> Self {
        Self(inner.into())
    }
}

impl From<IntegerObject> for ValueTypeObject {
    fn from(inner: IntegerObject) -> Self {
        Self(inner.into())
    }
}

impl From<ObjectObject> for ValueTypeObject {
    fn from(inner: ObjectObject) -> Self {
        Self(inner.into())
    }
}

impl From<ValueObject> for ValueTypeObject {
    fn from(inner: ValueObject) -> Self {
        Self(inner)
    }
}

impl Display for ValueTypeObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            ValueObject::Null(inner) => write!(formatter, "{inner}"),
            ValueObject::String(inner) => write!(formatter, "string {inner}"),
            ValueObject::Float(inner) => write!(formatter, "float {inner}"),
            ValueObject::Integer(inner) => write!(formatter, "integer {inner}"),
            ValueObject::Boolean(inner) => write!(formatter, "boolean {inner}"),
            ValueObject::Array(inner) => write!(formatter, "array {inner}"),
            ValueObject::Object(inner) => write!(formatter, "object {inner}"),
        }
    }
}
