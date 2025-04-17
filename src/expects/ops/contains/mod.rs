use crate::internals::Context;
use crate::internals::JsonValueEqResult;
use crate::ExpectOp;
use crate::JsonType;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

mod array_contains;
pub use self::array_contains::*;

mod array_contains_not;
pub use self::array_contains_not::*;

mod object_contains;
pub use self::object_contains::*;

mod object_contains_not;
pub use self::object_contains_not::*;

mod string_contains;
pub use self::string_contains::*;

mod string_contains_not;
pub use self::string_contains_not::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Contains {
    Array(ArrayContains),
    Object(ObjectContains),
    String(StringContains),
    ArrayNot(ArrayContainsNot),
    ObjectNot(ObjectContainsNot),
    StringNot(StringContainsNot),
}

impl Contains {
    pub(crate) fn new<V>(values: V) -> Self
    where
        V: Into<Value>,
    {
        let value = Into::<Value>::into(values);
        match value {
            Value::Array(values_array) => Self::Array(ArrayContains::new(values_array)),
            Value::String(values_string) => Self::String(StringContains::new(values_string)),
            Value::Object(values_object) => Self::Object(ObjectContains::new(values_object)),
            _ => {
                let value_type = JsonType::from(&value);
                panic!(
                    ".contains expected to take array, string, or object. Received: {value_type}"
                );
            }
        }
    }

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

#[typetag::serde]
impl ExpectOp for Contains {
    fn on_object(
        &self,
        context: &mut Context,
        received: &serde_json::Map<String, Value>,
    ) -> JsonValueEqResult<()> {
        match self {
            Self::Object(inner) => inner.on_object(context, received),
            Self::ObjectNot(inner) => inner.on_object(context, received),
            _ => Err(context.unsupported_expect_op_type(JsonType::Object, self)),
        }
    }

    fn on_array(&self, context: &mut Context, received: &[Value]) -> JsonValueEqResult<()> {
        match self {
            Self::Array(inner) => inner.on_array(context, received),
            Self::ArrayNot(inner) => inner.on_array(context, received),
            _ => Err(context.unsupported_expect_op_type(JsonType::Array, self)),
        }
    }

    fn on_string(&self, context: &mut Context, received: &str) -> JsonValueEqResult<()> {
        match self {
            Self::String(inner) => inner.on_string(context, received),
            Self::StringNot(inner) => inner.on_string(context, received),
            _ => Err(context.unsupported_expect_op_type(JsonType::String, self)),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Array(inner) => inner.name(),
            Self::ArrayNot(inner) => inner.name(),
            Self::Object(inner) => inner.name(),
            Self::ObjectNot(inner) => inner.name(),
            Self::String(inner) => inner.name(),
            Self::StringNot(inner) => inner.name(),
        }
    }

    fn supported_types(&self) -> &'static [JsonType] {
        match self {
            Self::Array(inner) => inner.supported_types(),
            Self::ArrayNot(inner) => inner.supported_types(),
            Self::Object(inner) => inner.supported_types(),
            Self::ObjectNot(inner) => inner.supported_types(),
            Self::String(inner) => inner.supported_types(),
            Self::StringNot(inner) => inner.supported_types(),
        }
    }
}

#[cfg(test)]
mod test_new {
    use crate::expect;
    use serde_json::json;

    #[test]
    #[should_panic]
    fn it_should_error_if_given_not_an_array_or_string() {
        expect.contains(json!(false));
    }
}
