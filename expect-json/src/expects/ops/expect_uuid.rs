use crate::expect_op;
use crate::Context;
use crate::ExpectOp;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use crate::JsonType;
use uuid::Uuid;

///
/// Expects an ISO 8601 date time string.
///
/// By _default_ this expects a UTC timezone, and this can be disabled with [Self::allow_non_utc()].
///
/// ```rust
/// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
/// #
/// # use axum::Router;
/// # use axum::extract::Json;
/// # use axum::routing::get;
/// # use axum_test::TestServer;
/// # use serde_json::json;
/// # use std::time::Instant;
/// #
/// # let server = TestServer::new(Router::new())?;
/// #
/// use std::time::Duration;
/// use axum_test::expect_json::expect;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/latest-comment")
///     .await
///     .assert_json(&json!({
///         "comment": "My example comment",
///         "created_at": expect::iso_date_time(),
///     }));
/// #
/// # Ok(()) }
/// ```
///
#[expect_op(internal, name = "uuid")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectUuid {
    expected_version: Option<u8>,
    is_not_nil_flag: bool,
}

impl ExpectUuid {
    pub(crate) fn new() -> Self {
        Self {
            expected_version: None,
            is_not_nil_flag: false,
        }
    }

    pub fn is_not_nil(mut self) -> Self {
        self.is_not_nil_flag = true;
        self
    }

    pub fn version(mut self, version: u8) -> Self {
        self.expected_version = Some(version);
        self
    }
}

impl ExpectOp for ExpectUuid {
    fn on_string(&self, context: &mut Context, received: &str) -> ExpectOpResult<()> {
        let uuid = Uuid::parse_str(received).map_err(|error| {
            let error_message = format!("failed to parse string '{received}' as uuid");
            ExpectOpError::custom_error(context, self, error_message, error)
        })?;

        if let Some(expected_version) = self.expected_version {
            let received_version = uuid.get_version_num();
            if received_version != (expected_version as usize) {
                let error_message = format!("expected uuid version '{expected_version}', received version '{received_version}', for uuid '{received}'");
                return Err(ExpectOpError::custom(context, self, error_message));
            }
        }

        if self.is_not_nil_flag && uuid.is_nil() {
            let error_message =
                format!("expected uuid to be not nil, but it is, received '{received}'");
            return Err(ExpectOpError::custom(context, self, error_message));
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::String]
    }
}

#[cfg(test)]
mod test_uuid {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_parse_a_simple_uuid() {
        let left = json!("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8");
        let right = json!(expect::uuid());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_parse_a_hyphenated_uuid() {
        let left = json!("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");
        let right = json!(expect::uuid());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_to_parse_an_invalid_uuid() {
        let left = json!("🦊");
        let right = json!(expect::uuid());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::uuid() error at root:
    failed to parse string '🦊' as uuid,
    invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `🦊` at 1"#
        );
    }
}

#[cfg(test)]
mod test_is_not_nil {
    use crate::expect;
    use crate::expect_json_eq;
    use serde_json::json;

    #[test]
    fn it_should_return_true_for_a_non_nil_uuid() {
        let left = json!("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");
        let right = json!(expect::uuid().is_not_nil());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_return_false_for_a_nil_uuid() {
        let left = json!("00000000-0000-0000-0000-000000000000");
        let right = json!(expect::uuid().is_not_nil());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::uuid() error at root:
    expected uuid to be not nil, but it is, received '00000000-0000-0000-0000-000000000000'"#
        );
    }
}

#[cfg(test)]
mod test_version {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_return_true_for_matching_version_1_uuid() {
        let left = json!("f3b4958c-52a1-11e7-802a-010203040506");
        let right = json!(expect::uuid().version(1));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_return_false_for_uuid_with_different_version() {
        let left = json!("f3b4958c-52a1-11e7-802a-010203040506");
        let right = json!(expect::uuid().version(2));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::uuid() error at root:
    expected uuid version '2', received version '1', for uuid 'f3b4958c-52a1-11e7-802a-010203040506'"#
        );
    }
}
