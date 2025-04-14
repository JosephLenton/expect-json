use crate::expects::ExpectOp;
use crate::internals::objects::StringObject;
use crate::internals::Context;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use crate::JsonType;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StringContains {
    content: String,
}

impl StringContains {
    pub(crate) fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            content: content.into(),
        }
    }
}

impl ExpectOp for StringContains {
    fn on_string(&self, context: &mut Context<'_>, received: &str) -> JsonValueEqResult<()> {
        if !received.contains(&self.content) {
            return Err(JsonValueEqError::ContainsNotFound {
                context: context.to_static(),
                json_type: JsonType::String,
                expected: StringObject::from(self.content.clone()).into(),
                received: StringObject::from(received.to_owned()).into(),
            });
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "StringContains"
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::String]
    }
}

#[cfg(test)]
mod test_string_contains {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_strings() {
        let left = json!("1, 2, 3");
        let right = json!(expect.contains("1, 2, 3"));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_partial_matches_in_middle() {
        let left = json!("0, 1, 2, 3, 4");
        let right = json!(expect.contains("1, 2, 3"));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!("0, 1, 2, 3, 4, 5");
        let right = json!(expect.contains(""));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_for_totall_different_values() {
        let left = json!("1, 2, 3");
        let right = json!(expect.contains("a, b, c"));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json string at root does not contain expected value:
    expected string to contain "a, b, c", but it was not found.
    received "1, 2, 3""#
        );
    }
}
