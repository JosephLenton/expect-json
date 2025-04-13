use crate::expects::SerializeExpectOp;
use crate::internals::objects::ArrayObject;
use crate::internals::types::ValueType;
use crate::internals::Context;
use crate::internals::JsonExpectOp;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ArrayContainsNot {
    values: Vec<Value>,
}

impl ArrayContainsNot {
    pub(crate) fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
}

impl JsonExpectOp for ArrayContainsNot {
    fn on_array<'a>(
        self,
        context: &mut Context<'a>,
        received_values: &'a [Value],
    ) -> JsonValueEqResult<()> {
        let received_items_in_set = received_values.iter().collect::<HashSet<&'a Value>>();

        for expected in self.values {
            if received_items_in_set.contains(&expected) {
                return Err(JsonValueEqError::ContainsFound {
                    context: context.to_static(),
                    json_type: ValueType::Array,
                    expected: expected.into(),
                    received: ArrayObject::from(received_values.to_owned()).into(),
                });
            }
        }

        Ok(())
    }
}

impl From<ArrayContainsNot> for SerializeExpectOp {
    fn from(contains: ArrayContainsNot) -> Self {
        SerializeExpectOp::ArrayContainsNot(contains)
    }
}

#[cfg(test)]
mod test_from {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_should_convert_to_correct_op() {
        let contains = ArrayContainsNot::new(vec![json!(123)]);
        let op: SerializeExpectOp = contains.clone().into();

        assert_eq!(op, SerializeExpectOp::ArrayContainsNot(contains));
    }
}

#[cfg(test)]
mod test_array_contains_not {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_error_for_identical_numeric_arrays() {
        let left = json!([1, 2, 3]);
        let right = json!(expect.not.contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json array at root contains value was expecting to not be there:
    expected array to not contain 1, but it was found.
    received [1, 2, 3]"#
        );
    }

    #[test]
    fn it_should_errorfor_reversed_identical_numeric_arrays() {
        let left = json!([1, 2, 3]);
        let right = json!(expect.not.contains([3, 2, 1]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json array at root contains value was expecting to not be there:
    expected array to not contain 3, but it was found.
    received [1, 2, 3]"#
        );
    }

    #[test]
    fn it_should_error_for_partial_contains() {
        let left = json!([0, 1, 2, 3, 4, 5]);
        let right = json!(expect.not.contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json array at root contains value was expecting to not be there:
    expected array to not contain 1, but it was found.
    received [0, 1, 2, 3, 4, 5]"#
        );
    }

    #[test]
    fn it_should_be_ok_for_totall_different_values() {
        let left = json!([0, 1, 2, 3]);
        let right = json!(expect.not.contains([4, 5, 6]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!([0, 1, 2, 3]);
        let right = json!(expect.not.contains(&[] as &'static [u32]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}
