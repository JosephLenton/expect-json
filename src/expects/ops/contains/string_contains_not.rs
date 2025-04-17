use crate::expects::ExpectOp;
use crate::internals::objects::StringObject;
use crate::internals::Context;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use crate::JsonType;
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

#[typetag::serde]
impl ExpectOp for StringContainsNot {
    fn on_string(&self, context: &mut Context<'_>, received: &str) -> JsonValueEqResult<()> {
        if received.contains(&self.content) {
            return Err(JsonValueEqError::ContainsFound {
                context: context.to_static(),
                json_type: JsonType::String,
                expected: StringObject::from(self.content.clone()).into(),
                received: StringObject::from(received.to_owned()).into(),
            });
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "StringContainsNot"
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::String]
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
