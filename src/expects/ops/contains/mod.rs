mod array_contains;
pub use self::array_contains::*;

mod string_contains;
pub use self::string_contains::*;

use crate::expects::SerializeExpectOp;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Contains {
    Array(ArrayContains),
    String(StringContains),
}

impl Contains {
    pub(crate) fn new<V>(values: V) -> Self
    where
        V: Into<Value>,
    {
        match Into::<Value>::into(values) {
            Value::Array(values_array) => Self::Array(ArrayContains::new(values_array)),
            Value::String(values_string) => Self::String(StringContains::new(values_string)),
            _ => unimplemented!("todo"),
        }
    }
}

impl From<Contains> for SerializeExpectOp {
    fn from(contains: Contains) -> Self {
        match contains {
            Contains::Array(contains) => Self::ArrayContains(contains),
            Contains::String(contains) => Self::StringContains(contains),
        }
    }
}
