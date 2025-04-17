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

#[typetag::serde]
impl ExpectOp for ObjectContains {
    fn on_object(
        &self,
        context: &mut Context,
        received_values: &Map<String, Value>,
    ) -> JsonValueEqResult<()> {
        for (key, expected_value) in &self.values {
            let received_value = received_values
                .get(key)
                .ok_or_else(|| unimplemented!("todo, add an error type here"))
                .unwrap();

            context.push(key.to_owned());
            context.json_eq(received_value, expected_value)?;
            context.pop();
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
mod test_object_contains {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_objects() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right =
            json!(expect.contains(json!({ "name": "John", "age": 30, "scores": [1, 2, 3] })));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_reversed_identical_objects() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right =
            json!(expect.contains(json!({ "scores": [1, 2, 3], "age": 30, "name": "John" })));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_partial_contains() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right = json!(expect.contains(json!({ "name": "John", "age": 30 })));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_for_same_fields_but_different_values() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right =
            json!(expect.contains(json!({ "name": "Joe", "age": 31, "scores": [4, 5, 6] })));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json integers at root.age are not equal:
    expected 31
    received 30"#
        );
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right = json!(expect.contains(json!({})));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_ok_for_empty_on_empty_object() {
        let left = json!({});
        let right = json!(expect.contains(json!({})));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_if_used_against_the_wrong_type() {
        let left = json!("🦊");
        let right = json!(expect.contains(json!({})));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json comparison on unsupported type, at root:
    operation ObjectContains cannot be performed against string,
    only supported type is: array"#
        );
    }
}
