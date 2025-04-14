use crate::expects::ExpectOp;
use crate::internals::Context;
use crate::internals::JsonValueEqResult;
use crate::JsonType;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ObjectContains {
    values: Map<String, Value>,
}

impl ObjectContains {
    pub(crate) fn new(values: Map<String, Value>) -> Self {
        Self { values }
    }
}

impl ExpectOp for ObjectContains {
    fn on_object<'a>(
        &'a self,
        context: &mut Context<'_>,
        received_values: &'a Map<String, Value>,
    ) -> JsonValueEqResult<()> {
        for (key, expected_value) in &self.values {
            let received_value = received_values
                .get(key)
                .ok_or_else(|| unimplemented!("todo, add an error type here"))
                .unwrap();

            context.json_eq(received_value, expected_value)?;
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "ObjectContains"
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
    operation ObjectContains cannot be performed against string,
    only supported type is: array"#
        );
    }
}
