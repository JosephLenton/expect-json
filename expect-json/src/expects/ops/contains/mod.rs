use crate::expect_op;
use crate::Context;
use crate::ExpectOp;
use crate::ExpectOpResult;
use crate::JsonType;
use serde_json::Value;

mod array_contains_not;
pub use self::array_contains_not::*;

mod object_contains_not;
pub use self::object_contains_not::*;

mod string_contains_not;
pub use self::string_contains_not::*;
use crate::ExpectOpError;

#[expect_op(internal)]
#[derive(Clone, Debug, PartialEq)]
pub enum Contains {
    ArrayNot(ArrayContainsNot),
    ObjectNot(ObjectContainsNot),
    StringNot(StringContainsNot),
}

impl Contains {
    pub(crate) fn new_not<V>(values: V) -> Self
    where
        V: Into<Value>,
    {
        let value = Into::<Value>::into(values);
        match value {
            Value::Array(values_array) => Self::ArrayNot(ArrayContainsNot::new(values_array)),
            Value::String(values_string) => Self::StringNot(StringContainsNot::new(values_string)),
            Value::Object(values_object) => Self::ObjectNot(ObjectContainsNot::new(values_object)),
            _ => {
                let value_type = JsonType::from(&value);
                panic!(
                    ".not.contains expected to take array, string, or object. Received: {value_type}"
                );
            }
        }
    }
}

impl ExpectOp for Contains {
    fn on_object(
        &self,
        context: &mut Context,
        received: &serde_json::Map<String, Value>,
    ) -> ExpectOpResult<()> {
        match self {
            Self::ObjectNot(inner) => inner.on_object(context, received),
            _ => Err(ExpectOpError::unsupported_operation_type(
                context,
                self,
                JsonType::Object,
            )),
        }
    }

    fn on_array(&self, context: &mut Context, received: &[Value]) -> ExpectOpResult<()> {
        match self {
            Self::ArrayNot(inner) => inner.on_array(context, received),
            _ => Err(ExpectOpError::unsupported_operation_type(
                context,
                self,
                JsonType::Array,
            )),
        }
    }

    fn on_string(&self, context: &mut Context, received: &str) -> ExpectOpResult<()> {
        match self {
            Self::StringNot(inner) => inner.on_string(context, received),
            _ => Err(ExpectOpError::unsupported_operation_type(
                context,
                self,
                JsonType::String,
            )),
        }
    }
}
