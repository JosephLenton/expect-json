use crate::expect_op;
use crate::expects::ExpectOp;
use crate::internals::objects::ArrayObject;
use crate::internals::Context;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use crate::JsonType;
use serde_json::Value;

#[expect_op(internal)]
#[derive(Clone, Debug, PartialEq)]
pub struct ArrayContains {
    values: Vec<Value>,
}

impl ArrayContains {
    pub(crate) fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
}

impl ExpectOp for ArrayContains {
    fn on_array(&self, context: &mut Context<'_>, received_values: &[Value]) -> ExpectOpResult<()> {
        // TODO: This is brute force as we don't know if we are containing an inner ExpectOp.
        // Can this be done without a brute force approach?
        for expected in &self.values {
            let is_found = received_values
                .iter()
                .any(|received| context.json_eq(received, expected).is_ok());

            if !is_found {
                return Err(ExpectOpError::ContainsNotFound {
                    context: context.to_static(),
                    json_type: JsonType::Array,
                    expected: expected.clone().into(),
                    received: ArrayObject::from(received_values.to_owned()).into(),
                });
            }
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::Array]
    }
}

#[cfg(test)]
mod test_array_contains {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_numeric_arrays() {
        let left = json!([1, 2, 3]);
        let right = json!(expect.contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_reversed_identical_numeric_arrays() {
        let left = json!([1, 2, 3]);
        let right = json!(expect.contains([3, 2, 1]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_partial_contains() {
        let left = json!([0, 1, 2, 3, 4, 5]);
        let right = json!(expect.contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_for_totall_different_values() {
        let left = json!([0, 1, 2, 3]);
        let right = json!(expect.contains([4, 5, 6]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json array at root does not contain expected value:
    expected array to contain 4, but it was not found.
    received [0, 1, 2, 3]"#
        );
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!([0, 1, 2, 3]);
        let right = json!(expect.contains(&[] as &'static [u32]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_if_used_against_the_wrong_type() {
        let left = json!("🦊");
        let right = json!(expect.contains([4, 5, 6]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json comparison on unsupported type, at root:
    expect.Contains() cannot be performed against string,
    only supported type is: array"#
        );
    }

    #[test]
    fn it_should_handle_nested_contains() {
        let left = json!([
            {
                "text": "Hello",
                "author": "Jane Candle"
            },
            {
                "text": "Goodbye",
                "author": "John Lighthouse"
            }
        ]);

        let right = json!(expect.contains([json!({
            "text": "Hello",
            "author": expect.contains("Jane"),
        }),]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{}", output.unwrap_err().to_string());
    }
}
