use crate::expect_op;
use crate::Context;
use crate::ExpectOp;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use crate::JsonType;
use core::str::FromStr;
use email_address::EmailAddress;

///
/// Expects a valid email address string.
///
/// You can build these using the [`crate::expect::email`] function.
///
#[expect_op(internal, name = "email")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectEmail {
    expected_domain: Option<String>,
    expected_local_part: Option<String>,
}

impl ExpectEmail {
    pub(crate) fn new() -> Self {
        Self {
            expected_domain: None,
            expected_local_part: None,
        }
    }

    ///
    /// Expects the local part of the email address.
    /// The 'local part' is the part before the '@' symbol.
    /// i.e. the 'joe' in 'joe@example.com'.
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
    /// use axum_test::expect_json::expect;
    ///
    /// let server = TestServer::new(Router::new())?;
    ///
    /// server.get(&"/user")
    ///     .await
    ///     .assert_json(&json!({
    ///         "name": "Joe",
    ///         "email": expect::email().with_local_part("joe"),
    ///     }));
    /// #
    /// # Ok(()) }
    /// ```
    ///
    pub fn with_local_part<S>(mut self, local_part: S) -> Self
    where
        S: Into<String>,
    {
        self.expected_local_part = Some(local_part.into());
        self
    }

    ///
    /// Expects the domain part of the email address.
    /// i.e. 'example.com' in 'joe@example.com'.
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
    /// use axum_test::expect_json::expect;
    ///
    /// let server = TestServer::new(Router::new())?;
    ///
    /// server.get(&"/user")
    ///     .await
    ///     .assert_json(&json!({
    ///         "name": "Joe",
    ///         "email": expect::email().with_domain("example.com"),
    ///     }));
    /// #
    /// # Ok(()) }
    /// ```
    ///
    pub fn with_domain<S>(mut self, domain: S) -> Self
    where
        S: Into<String>,
    {
        self.expected_domain = Some(domain.into());
        self
    }
}

impl ExpectOp for ExpectEmail {
    fn on_string(&self, context: &mut Context, received: &str) -> ExpectOpResult<()> {
        let email = EmailAddress::from_str(received).map_err(|e| {
            let error_message = format!("Invalid email address, received '{received}'");
            ExpectOpError::custom_error(context, self, error_message, e)
        })?;

        if let Some(expected_local_part) = &self.expected_local_part {
            if email.local_part() != expected_local_part {
                return Err(ExpectOpError::custom(
                    context,
                    self,
                    format!(
                        "Local part mismatch, expected '{}', received '{received}'",
                        expected_local_part
                    ),
                ));
            }
        }

        if let Some(expected_domain) = &self.expected_domain {
            if email.domain() != expected_domain {
                return Err(ExpectOpError::custom(
                    context,
                    self,
                    format!(
                        "Domain mismatch, expected '{}', received '{received}'",
                        expected_domain
                    ),
                ));
            }
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::String]
    }
}

#[cfg(test)]
mod test_email {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_accept_valid_email() {
        let left = json!("test@example.com");
        let right = json!(expect::email());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_accept_valid_email_with_plus_sign() {
        let left = json!("test+test@example.com");
        let right = json!(expect::email());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_accept_email_inside_object() {
        let left = json!({ "name": "Joe", "email": "test@example.com" });
        let right = json!({
            "name": "Joe",
            "email": expect::email(),
        });

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_reject_invalid_email() {
        let left = json!("🦊");
        let right = json!(expect::email());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::email() error at root:
    Invalid email address, received '🦊',
    Missing separator character '@'."#
            )
        );
    }
}

#[cfg(test)]
mod test_local_part {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_accept_valid_local_part() {
        let left = json!("test@example.com");
        let right = json!(expect::email().with_local_part("test"));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_reject_invalid_local_part() {
        let left = json!("test@example.com");
        let right = json!(expect::email().with_local_part("🦊"));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::email() error at root:
    Local part mismatch, expected '🦊', received 'test@example.com'"#
            )
        );
    }
}

#[cfg(test)]
mod test_domain {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_accept_valid_domain() {
        let left = json!("test@example.com");
        let right = json!(expect::email().with_domain("example.com"));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_reject_invalid_domain() {
        let left = json!("test@example.com");
        let right = json!(expect::email().with_domain("🦊.fox"));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::email() error at root:
    Domain mismatch, expected '🦊.fox', received 'test@example.com'"#
            )
        );
    }
}
