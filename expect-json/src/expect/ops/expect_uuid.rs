use crate::expect_core::expect_op;
use crate::expect_core::Context;
use crate::expect_core::ExpectOp;
use crate::expect_core::ExpectOpError;
use crate::expect_core::ExpectOpResult;
use crate::JsonType;
use uuid::Uuid;

///
/// Expects a UUID string.
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
///         "id": expect_json::uuid(),
///         "name": "Alice",
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

    /// Expects this is not the 'nil' UUID, which is "00000000-0000-0000-0000-000000000000".
    pub fn not_nil(mut self) -> Self {
        self.is_not_nil_flag = true;
        self
    }

    /// Expects this meets the given UUID version.
    ///
    /// Details on the different versions can be found on Wikipedia: <https://en.wikipedia.org/wiki/Universally_unique_identifier#Versions_of_the_OSF_DCE_variant>
    pub fn version(mut self, version: u8) -> Self {
        self.expected_version = Some(version);
        self
    }
}

impl ExpectOp for ExpectUuid {
    fn on_string(&self, context: &mut Context, received: &str) -> ExpectOpResult<()> {
        let uuid = Uuid::parse_str(received).map_err(|error| {
            let error_message = format!("failed to parse string '{received}' as uuid");
            ExpectOpError::custom_error(self, context, error_message, error)
        })?;

        if let Some(expected_version) = self.expected_version {
            let received_version = uuid.get_version_num();
            if received_version != (expected_version as usize) {
                let error_message = format!("expected uuid version '{expected_version}', received version '{received_version}', for uuid '{received}'");
                return Err(ExpectOpError::custom(self, context, error_message));
            }
        }

        if self.is_not_nil_flag && uuid.is_nil() {
            let error_message =
                format!("expected uuid to be not nil, but it is, received '{received}'");
            return Err(ExpectOpError::custom(self, context, error_message));
        }

        Ok(())
    }

    fn debug_supported_types(&self) -> &'static [JsonType] {
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
mod test_not_nil {
    use crate::expect;
    use crate::expect_json_eq;
    use serde_json::json;

    #[test]
    fn it_should_return_true_for_a_non_nil_uuid() {
        let left = json!("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");
        let right = json!(expect::uuid().not_nil());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_return_false_for_a_nil_uuid() {
        let left = json!("00000000-0000-0000-0000-000000000000");
        let right = json!(expect::uuid().not_nil());

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
