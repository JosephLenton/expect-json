use crate::expect::ops::expect_string::ExpectStringSubOp;
use crate::expect_op;
use crate::expect_op::Context;
use crate::expect_op::ExpectOp;
use crate::expect_op::ExpectOpResult;
use crate::JsonType;

#[expect_op(internal, name = "string")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectString {
    sub_ops: Vec<ExpectStringSubOp>,
}

impl ExpectString {
    pub(crate) fn new() -> Self {
        Self { sub_ops: vec![] }
    }

    pub fn is_empty(mut self) -> Self {
        self.sub_ops.push(ExpectStringSubOp::IsEmpty);
        self
    }

    pub fn is_not_empty(mut self) -> Self {
        self.sub_ops.push(ExpectStringSubOp::IsNotEmpty);
        self
    }

    pub fn min_len(mut self, min_len: usize) -> Self {
        self.sub_ops.push(ExpectStringSubOp::MinLen(min_len));
        self
    }

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.sub_ops.push(ExpectStringSubOp::MaxLen(max_len));
        self
    }

    pub fn contains<S>(mut self, expected_sub_string: S) -> Self
    where
        S: Into<String>,
    {
        self.sub_ops
            .push(ExpectStringSubOp::Contains(expected_sub_string.into()));
        self
    }
}

impl ExpectOp for ExpectString {
    fn on_string(&self, context: &mut Context, received: &str) -> ExpectOpResult<()> {
        for sub_op in &self.sub_ops {
            sub_op.on_string(self, context, received)?;
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::String]
    }
}

#[cfg(test)]
mod test_contains {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_strings() {
        let left = json!("1, 2, 3");
        let right = json!(expect::string().contains("1, 2, 3"));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_partial_matches_in_middle() {
        let left = json!("0, 1, 2, 3, 4");
        let right = json!(expect::string().contains("1, 2, 3"));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!("0, 1, 2, 3, 4, 5");
        let right = json!(expect::string().contains(""));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_for_totall_different_values() {
        let left = json!("1, 2, 3");
        let right = json!(expect::string().contains("a, b, c"));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json string at root does not contain expected value:
    expected string to contain "a, b, c", but it was not found.
    received "1, 2, 3""#
        );
    }
}

#[cfg(test)]
mod test_is_empty {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_is_empty() {
        let left = json!("");
        let right = json!(expect::string().is_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_is_not_empty() {
        let left = json!("🦊");
        let right = json!(expect::string().is_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::string() error at root:
    expected empty string
    received "🦊""#
            )
        );
    }
}

#[cfg(test)]
mod test_is_not_empty {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_is_not_empty() {
        let left = json!("🦊");
        let right = json!(expect::string().is_not_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_is_empty() {
        let left = json!("");
        let right = json!(expect::string().is_not_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::string() error at root:
    expected non-empty string
    received """#
            )
        );
    }
}

#[cfg(test)]
mod test_min_len {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_has_exactly_enough_characters() {
        let left = json!("123");
        let right = json!(expect::string().min_len(3));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_pass_when_string_has_more_than_enough_characters() {
        let left = json!("12345");
        let right = json!(expect::string().min_len(3));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_is_too_short() {
        let left = json!("12");
        let right = json!(expect::string().min_len(3));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::string() error at root:
    expected string to have at least 3 characters, but it has 2,
    received "12""#
            )
        );
    }
}

#[cfg(test)]
mod test_max_len {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_has_exactly_enough_characters() {
        let left = json!("123");
        let right = json!(expect::string().max_len(3));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_pass_when_string_has_less_than_enough_characters() {
        let left = json!("12");
        let right = json!(expect::string().max_len(5));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_is_too_long() {
        let left = json!("🦊🦊🦊🦊🦊🦊");
        let right = json!(expect::string().max_len(3));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::string() error at root:
    expected string to have at most 3 characters, but it has 24,
    received "🦊🦊🦊🦊🦊🦊""#
            )
        );
    }
}
