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
pub struct StringContainsNot {
    content: String,
}

impl StringContainsNot {
    pub(crate) fn new(content: String) -> Self {
        Self { content }
    }
}

impl JsonExpectOp for StringContainsNot {
    fn on_string<'a>(self, context: &mut Context<'a>, received: &'a str) -> JsonValueEqResult<()> {
        if received.contains(&self.content) {
            return Err(JsonValueEqError::ContainsFound {
                context: context.to_static(),
                json_type: ValueType::String,
                expected: StringObject::from(self.content).into(),
                received: StringObject::from(received.to_owned()).into(),
            });
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [ValueType] {
        &[ValueType::String]
    }
}

impl From<StringContainsNot> for SerializeExpectOp {
    fn from(contains: StringContainsNot) -> Self {
        SerializeExpectOp::StringContainsNot(contains)
    }
}

#[cfg(test)]
mod test_from {
    use super::*;

    #[test]
    fn it_should_convert_to_correct_op() {
        let contains = StringContainsNot::new("my-string".to_string());
        let op: SerializeExpectOp = contains.clone().into();

        assert_eq!(op, SerializeExpectOp::StringContainsNot(contains));
    }
}

#[cfg(test)]
mod test_string_contains_not {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_error_for_identical_strings() {
        let left = json!("1, 2, 3");
        let right = json!(expect.not.contains("1, 2, 3"));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json string at root contains value was expecting to not be there:
    expected string to not contain "1, 2, 3", but it was found.
    received "1, 2, 3""#
        );
    }

    #[test]
    fn it_should_error_for_partial_matches_in_middle() {
        let left = json!("0, 1, 2, 3, 4");
        let right = json!(expect.not.contains("1, 2, 3"));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json string at root contains value was expecting to not be there:
    expected string to not contain "1, 2, 3", but it was found.
    received "0, 1, 2, 3, 4""#
        );
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!("0, 1, 2, 3, 4, 5");
        let right = json!(expect.not.contains(""));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json string at root contains value was expecting to not be there:
    expected string to not contain "", but it was found.
    received "0, 1, 2, 3, 4, 5""#
        );
    }

    #[test]
    fn it_should_be_ok_for_totall_different_values() {
        let left = json!("1, 2, 3");
        let right = json!(expect.not.contains("a, b, c"));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}
