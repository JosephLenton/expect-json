use crate::expect_op;
use crate::expects::ExpectOp;
use crate::internals::objects::ArrayObject;
use crate::internals::Context;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use crate::JsonType;
use serde_json::Value;
use std::collections::HashSet;

#[expect_op(internal)]
#[derive(Clone, Debug, PartialEq)]
pub struct ArrayContainsNot {
    values: Vec<Value>,
}

impl ArrayContainsNot {
    pub(crate) fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
}

impl ExpectOp for ArrayContainsNot {
    fn on_array(
        &self,
        context: &mut Context<'_>,
        received_values: &[Value],
    ) -> JsonValueEqResult<()> {
        let received_items_in_set = received_values.iter().collect::<HashSet<&Value>>();

        for expected in &self.values {
            if received_items_in_set.contains(&expected) {
                return Err(JsonValueEqError::ContainsFound {
                    context: context.to_static(),
                    json_type: JsonType::Array,
                    expected: expected.clone().into(),
                    received: ArrayObject::from(received_values.to_owned()).into(),
                });
            }
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "ArrayContainsNot"
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::Array]
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
