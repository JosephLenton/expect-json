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

#[cfg(test)]
mod test_from {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_should_convert_array_to_correct_op() {
        let contains = ArrayContainsNot::new(vec![json!(123)]);
        let op: SerializeExpectOp = ContainsNot::Array(contains.clone()).into();

        assert_eq!(op, SerializeExpectOp::ArrayContainsNot(contains));
    }

    #[test]
    fn it_should_convert_string_to_correct_op() {
        let contains = StringContainsNot::new("hello".to_string());
        let op: SerializeExpectOp = ContainsNot::String(contains.clone()).into();

        assert_eq!(op, SerializeExpectOp::StringContainsNot(contains));
    }
}
