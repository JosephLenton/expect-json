use crate::expects::SerializeExpectOp;
use crate::internals::objects::StringObject;
use crate::internals::types::ValueType;
use crate::internals::Context;
use crate::internals::JsonExpectOp;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
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

impl JsonExpectOp for StringContains {
    fn on_string<'a>(self, context: &mut Context<'a>, received: &'a str) -> JsonValueEqResult<()> {
        if !received.contains(&self.content) {
            return Err(JsonValueEqError::ContainsNotFound {
                context: context.to_static(),
                json_type: ValueType::String,
                expected: StringObject::from(self.content).into(),
                received: StringObject::from(received.to_owned()).into(),
            });
        }

        Ok(())
    }
}

impl From<StringContains> for SerializeExpectOp {
    fn from(contains: StringContains) -> Self {
        SerializeExpectOp::StringContains(contains)
    }
}

#[cfg(test)]
mod test_from {
    use super::*;

    #[test]
    fn it_should_convert_to_correct_op() {
        let contains = StringContains::new("my-string".to_string());
        let op: SerializeExpectOp = contains.clone().into();

        assert_eq!(op, SerializeExpectOp::StringContains(contains));
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
