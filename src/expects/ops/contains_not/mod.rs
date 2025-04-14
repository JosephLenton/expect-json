use crate::internals::types::ValueType;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

mod array_contains_not;
pub use self::array_contains_not::*;

mod string_contains_not;
pub use self::string_contains_not::*;
use crate::ExpectOp;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ContainsNot {
    Array(ArrayContainsNot),
    String(StringContainsNot),
}

impl ContainsNot {
    pub(crate) fn new<V>(values: V) -> Self
    where
        V: Into<Value>,
    {
        let value = Into::<Value>::into(values);
        match value {
            Value::Array(values_array) => Self::Array(ArrayContainsNot::new(values_array)),
            Value::String(values_string) => Self::String(StringContainsNot::new(values_string)),
            _ => {
                let value_type = ValueType::from(&value);
                panic!(
                    ".contains expected to take array, string, or object. Received: {value_type}"
                );
            }
        }
    }
}

impl ExpectOp for ContainsNot {
    fn on_array<'a>(
        &self,
        context: &mut crate::internals::Context<'a>,
        received: &'a [Value],
    ) -> crate::internals::JsonValueEqResult<()> {
        match self {
            Self::Array(inner) => inner.on_array(context, received),
            _ => Err(context.unsupported_expect_op_type(ValueType::Array, self)),
        }
    }

    fn on_string<'a>(
        &self,
        context: &mut crate::internals::Context<'a>,
        received: &'a str,
    ) -> crate::internals::JsonValueEqResult<()> {
        match self {
            Self::String(inner) => inner.on_string(context, received),
            _ => Err(context.unsupported_expect_op_type(ValueType::String, self)),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Array(inner) => inner.name(),
            Self::String(inner) => inner.name(),
        }
    }

    fn supported_types(&self) -> &'static [crate::internals::types::ValueType] {
        match self {
            Self::Array(inner) => inner.supported_types(),
            Self::String(inner) => inner.supported_types(),
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
        expect.not.contains(json!(false));
    }
}
