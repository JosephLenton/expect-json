use crate::expects::SerializeExpectOp;
use crate::internals::types::ValueType;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

mod array_contains;
pub use self::array_contains::*;

mod string_contains;
pub use self::string_contains::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Contains {
    Array(ArrayContains),
    String(StringContains),
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
            _ => {
                let value_type = ValueType::from(&value);
                panic!(
                    ".contains expected to take array, string, or object. Received: {value_type}"
                );
            }
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

#[cfg(test)]
mod test_from {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_should_convert_array_to_correct_op() {
        let contains = ArrayContains::new(vec![json!(123)]);
        let op: SerializeExpectOp = Contains::Array(contains.clone()).into();

        assert_eq!(op, SerializeExpectOp::ArrayContains(contains));
    }

    #[test]
    fn it_should_convert_string_to_correct_op() {
        let contains = StringContains::new("hello".to_string());
        let op: SerializeExpectOp = Contains::String(contains.clone()).into();

        assert_eq!(op, SerializeExpectOp::StringContains(contains));
    }
}
