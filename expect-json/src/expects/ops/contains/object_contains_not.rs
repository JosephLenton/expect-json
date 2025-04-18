use crate::expect_op;
use crate::expects::ExpectOp;
use crate::internals::Context;
use crate::internals::JsonValueEqResult;
use crate::JsonType;
use serde_json::Map;
use serde_json::Value;

#[expect_op(internal)]
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectContainsNot {
    values: Map<String, Value>,
}

impl ObjectContainsNot {
    pub(crate) fn new(values: Map<String, Value>) -> Self {
        Self { values }
    }
}

impl ExpectOp for ObjectContainsNot {
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
        "ObjectContainsNot"
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
