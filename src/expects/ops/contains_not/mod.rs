use crate::expects::SerializeExpectOp;
use crate::internals::types::ValueType;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

mod array_contains_not;
pub use self::array_contains_not::*;

mod string_contains_not;
pub use self::string_contains_not::*;

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

impl From<ContainsNot> for SerializeExpectOp {
    fn from(contains: ContainsNot) -> Self {
        match contains {
            ContainsNot::Array(contains) => Self::ArrayContainsNot(contains),
            ContainsNot::String(contains) => Self::StringContainsNot(contains),
        }
    }
}
