use crate::JsonType;
use crate::expect::ops::expect_string::ExpectStringSubOp;
use crate::expect_core::Context;
use crate::expect_core::ExpectOp;
use crate::expect_core::ExpectOpResult;
use crate::expect_core::expect_op;

#[expect_op(internal, name = "string")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectString {
    sub_ops: Vec<ExpectStringSubOp>,
}

impl ExpectString {
    pub(crate) fn new() -> Self {
        Self { sub_ops: vec![] }
    }

    pub fn empty(mut self) -> Self {
        self.sub_ops.push(ExpectStringSubOp::Empty);
        self
    }

    pub fn not_empty(mut self) -> Self {
        self.sub_ops.push(ExpectStringSubOp::NotEmpty);
        self
    }

    pub fn len(mut self, len: usize) -> Self {
        self.sub_ops.push(ExpectStringSubOp::Len(len));
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

    ///
    /// Expect a string containing a subset of the string given.
    ///
    /// ```rust
    /// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
    /// #
    /// # use axum::Router;
    /// # use axum::extract::Json;
    /// # use axum::routing::get;
    /// # use axum_test::TestServer;
    /// # use serde_json::json;
    /// #
    /// # let server = TestServer::new(Router::new())?;
    /// #
    /// use axum_test::expect_json;
    ///
    /// let server = TestServer::new(Router::new())?;
    ///
    /// server.get(&"/user")
    ///     .await
    ///     .assert_json(&json!({
    ///         "name": expect_json::string().contains("apples"),
    ///     }));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn contains<S>(mut self, expected_sub_string: S) -> Self
    where
        S: Into<String>,
    {
        self.sub_ops
            .push(ExpectStringSubOp::Contains(expected_sub_string.into()));
        self
    }

    ///
    /// Expect a string matching the regex given.
    ///
    /// ```rust
    /// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
    /// #
    /// # use axum::Router;
    /// # use axum::extract::Json;
    /// # use axum::routing::get;
    /// # use axum_test::TestServer;
    /// # use serde_json::json;
    /// #
    /// # let server = TestServer::new(Router::new())?;
    /// #
    /// use axum_test::expect_json;
    ///
    /// let server = TestServer::new(Router::new())?;
    ///
    /// server.get(&"/user")
    ///     .await
    ///     .assert_json(&json!({
    ///         "email": expect_json::string().matches_regex(r#"\w+@(?:\w+\.)+\w+"#),
    ///     }));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn matches_regex<S>(mut self, pattern: S) -> Self
    where
        S: Into<String>,
    {
        self.sub_ops
            .push(ExpectStringSubOp::MatchesRegex(pattern.into()));
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

    fn debug_supported_types(&self) -> &'static [JsonType] {
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
mod test_empty {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_is_empty() {
        let left = json!("");
        let right = json!(expect::string().empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_is_not_empty() {
        let left = json!("🦊");
        let right = json!(expect::string().empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::string() error at root:
    expected empty string
    received "🦊""#
        );
    }
}

#[cfg(test)]
mod test_not_empty {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_is_not_empty() {
        let left = json!("🦊");
        let right = json!(expect::string().not_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_is_empty() {
        let left = json!("");
        let right = json!(expect::string().not_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::string() error at root:
    expected non-empty string
    received """#
        );
    }
}

#[cfg(test)]
mod test_len {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_has_same_number_of_characters() {
        let left = json!("123");
        let right = json!(expect::string().len(3));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_is_too_short() {
        let left = json!("12");
        let right = json!(expect::string().len(3));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::string() error at root:
    expected string to have 3 characters, but it has 2,
    received "12""#
        );
    }

    #[test]
    fn it_should_fail_when_string_is_too_long() {
        let left = json!("1234");
        let right = json!(expect::string().len(3));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::string() error at root:
    expected string to have 3 characters, but it has 4,
    received "1234""#
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
            r#"Json expect::string() error at root:
    expected string to have at least 3 characters, but it has 2,
    received "12""#
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
            r#"Json expect::string() error at root:
    expected string to have at most 3 characters, but it has 24,
    received "🦊🦊🦊🦊🦊🦊""#
        );
    }
}

#[cfg(test)]
mod test_matches_regex {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_matches_regex() {
        let left = json!("abc123xyz");
        let right = json!(expect::string().matches_regex(r"^[a-z]+[0-9]+[a-z]+$"));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_does_not_match_regex() {
        let left = json!("abcxyz");
        let right = json!(expect::string().matches_regex(r"^[a-z]+[0-9]+[a-z]+$"));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json string error at root, regex did not match:
    expected string to match regex pattern '^[a-z]+[0-9]+[a-z]+$',
    received "abcxyz""#
        );
    }

    #[test]
    fn it_should_fail_when_regex_is_invalid() {
        let left = json!("abc123xyz");
        let right = json!(expect::string().matches_regex(r"([a-z]+"));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        // For robustness, we don't specify the error message coming from the regex crate.
        assert!(
            output.starts_with(r#"Json expect::string() error at root:"#),
            "Unexpected error output: {output:#?}"
        );
    }
}
