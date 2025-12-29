use crate::JsonType;
use crate::expect::ops::utils::DurationFormatter;
use crate::expect_core::Context;
use crate::expect_core::ExpectOp;
use crate::expect_core::ExpectOpError;
use crate::expect_core::ExpectOpResult;
use crate::expect_core::expect_op;
use chrono::DateTime;
use chrono::Duration as ChronoDuration;
use chrono::FixedOffset;
use chrono::Offset;
use chrono::Utc;
use std::time::Duration as StdDuration;

///
/// Expects an ISO 8601 date time string.
///
/// By _default_ this accepts any timezone.
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
/// use std::time::Duration;
/// use axum_test::expect_json;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/latest-comment")
///     .await
///     .assert_json(&json!({
///         "comment": "My example comment",
///         "created_at": expect_json::iso_date_time(),
///     }));
/// #
/// # Ok(()) }
/// ```
///
#[expect_op(internal, name = "iso_date_time")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectIsoDateTime {
    is_utc_only: bool,
    maybe_past_duration: Option<StdDuration>,
    maybe_future_duration: Option<StdDuration>,
}

impl ExpectIsoDateTime {
    pub(crate) fn new() -> Self {
        Self {
            is_utc_only: false,
            maybe_past_duration: None,
            maybe_future_duration: None,
        }
    }

    ///
    /// By default, `IsoDateTime` expects all date times to be in UTC.
    ///
    /// This method relaxes this constraint,
    /// and will accept date times in any timezone.
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
    /// use std::time::Duration;
    /// use axum_test::expect_json;
    ///
    /// let server = TestServer::new(Router::new())?;
    ///
    /// server.get(&"/latest-comment")
    ///     .await
    ///     .assert_json(&json!({
    ///         "comment": "My example comment",
    ///
    ///         // Users time may have any timezone
    ///         "users_created_at": expect_json::iso_date_time().utc(),
    ///     }));
    /// #
    /// # Ok(()) }
    /// ```
    ///
    pub fn utc(self) -> Self {
        Self {
            is_utc_only: true,
            ..self
        }
    }

    ///
    /// Expects the date time to be within a past duration,
    /// up to the current time.
    ///
    /// The constraint will fail when:
    ///  - the datetime is further in the past than the given duration,
    ///  - or ahead of the current time.
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
    /// use std::time::Duration;
    /// use axum_test::expect_json;
    ///
    /// let server = TestServer::new(Router::new())?;
    ///
    /// server.get(&"/latest-comment")
    ///     .await
    ///     .assert_json(&json!({
    ///         "comment": "My example comment",
    ///
    ///         // Expect it was updated in the last minute
    ///         "updated_at": expect_json::iso_date_time()
    ///             .within_past(Duration::from_secs(60)),
    ///     }));
    /// #
    /// # Ok(()) }
    /// ```
    ///
    pub fn within_past(self, duration: StdDuration) -> Self {
        Self {
            maybe_past_duration: Some(duration),
            ..self
        }
    }

    ///
    /// Expects the date time to be within the current time,
    /// and up to a future duration.
    ///
    /// The constraint will fail when:
    ///  - the datetime is further ahead than the given duration,
    ///  - or behind the current time.
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
    /// #
    /// use std::time::Duration;
    /// use axum_test::expect_json;
    ///
    /// let server = TestServer::new(Router::new())?;
    ///
    /// server.get(&"/latest-comment")
    ///     .await
    ///     .assert_json(&json!({
    ///         "comment": "My example comment",
    ///
    ///         // Expect it also expires in the next minute
    ///         "expires_at": expect_json::iso_date_time()
    ///             .within_future(Duration::from_secs(60)),
    ///     }));
    /// #
    /// # Ok(()) }
    /// ```
    ///
    pub fn within_future(self, duration: StdDuration) -> Self {
        Self {
            maybe_future_duration: Some(duration),
            ..self
        }
    }
}

impl ExpectOp for ExpectIsoDateTime {
    fn on_string(&self, context: &mut Context, received: &str) -> ExpectOpResult<()> {
        let date_time = DateTime::<FixedOffset>::parse_from_rfc3339(received).map_err(|error| {
            let error_message = format!("failed to parse string '{received}' as iso date time");
            ExpectOpError::custom_error(self, context, error_message, error)
        })?;

        if self.is_utc_only {
            let is_date_time_utc = date_time.offset().fix().utc_minus_local() != 0;
            if is_date_time_utc {
                let error_message = format!(
                    "ISO datetime '{received}' is using a non-UTC timezone, expected UTC only"
                );
                return Err(ExpectOpError::custom(self, context, error_message));
            }
        }

        match (self.maybe_past_duration, self.maybe_future_duration) {
            (None, None) => {}
            (Some(past_duration), None) => {
                let is_date_time_outside_past = date_time < Utc::now() - past_duration;
                if is_date_time_outside_past {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(past_duration).unwrap());
                    let error_message = format!(
                        "ISO datetime '{received}' is too far from the past, expected between '{duration}' ago and now"
                    );
                    return Err(ExpectOpError::custom(self, context, error_message));
                }

                let is_date_time_ahead_of_now = date_time > Utc::now();
                if is_date_time_ahead_of_now {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(past_duration).unwrap());
                    let error_message = format!(
                        "ISO datetime '{received}' is in the future of now, expected between '{duration}' ago and now"
                    );
                    return Err(ExpectOpError::custom(self, context, error_message));
                }
            }
            (None, Some(future_duration)) => {
                let is_date_time_outside_future = date_time > Utc::now() + future_duration;
                if is_date_time_outside_future {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(future_duration).unwrap());
                    let error_message = format!(
                        "ISO datetime '{received}' is too far in the future, expected between now and '{duration}' in the future"
                    );
                    return Err(ExpectOpError::custom(self, context, error_message));
                }

                let is_date_time_behind_of_now = date_time < Utc::now();
                if is_date_time_behind_of_now {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(future_duration).unwrap());
                    let error_message = format!(
                        "ISO datetime '{received}' is in the past of now, expected between now and '{duration}' in the future"
                    );
                    return Err(ExpectOpError::custom(self, context, error_message));
                }
            }
            (Some(past_duration), Some(future_duration)) => {
                let is_date_time_outside_past = date_time < Utc::now() - past_duration;
                if is_date_time_outside_past {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(past_duration).unwrap());
                    let error_message = format!(
                        "ISO datetime '{received}' is too far from the past, expected between '{duration}' ago and now"
                    );
                    return Err(ExpectOpError::custom(self, context, error_message));
                }

                let is_date_time_outside_future = date_time > Utc::now() + future_duration;
                if is_date_time_outside_future {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(future_duration).unwrap());
                    let error_message = format!(
                        "ISO datetime '{received}' is too far in the future, expected between now and '{duration}' in the future"
                    );
                    return Err(ExpectOpError::custom(self, context, error_message));
                }
            }
        }

        Ok(())
    }

    fn debug_supported_types(&self) -> &'static [JsonType] {
        &[JsonType::String]
    }
}

#[cfg(test)]
mod test_iso_date_time {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_parse_iso_datetime_with_utc_timezone() {
        let left = json!("2024-01-15T13:45:30Z");
        let right = json!(expect::iso_date_time());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_parse_iso_datetime_with_non_utc_timezone_by_default() {
        let left = json!("2024-01-15T13:45:30+01:00");
        let right = json!(expect::iso_date_time());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_to_parse_iso_datetime_without_timezone() {
        let left = json!("2024-01-15T13:45:30");
        let right = json!(expect::iso_date_time());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::iso_date_time() error at root:
    failed to parse string '2024-01-15T13:45:30' as iso date time,
    premature end of input"#
        );
    }
}

#[cfg(test)]
mod test_utc {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_parse_iso_datetime_with_utc_timezone_when_set() {
        let left = json!("2024-01-15T13:45:30Z");
        let right = json!(expect::iso_date_time().utc());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_not_parse_iso_datetime_with_non_utc_timezone_when_set() {
        let left = json!("2024-01-15T13:45:30+01:00");
        let right = json!(expect::iso_date_time().utc());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            "Json expect::iso_date_time() error at root:
    ISO datetime '2024-01-15T13:45:30+01:00' is using a non-UTC timezone, expected UTC only"
        );
    }
}

#[cfg(test)]
mod test_within_past {
    use super::*;
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_parse_iso_datetime_within_past_set() {
        let now = Utc::now();
        let now_str = (now - ChronoDuration::seconds(30)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect::iso_date_time().within_past(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_not_parse_iso_datetime_within_past_too_far() {
        let now = Utc::now();
        let now_str = (now - ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect::iso_date_time().within_past(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::iso_date_time() error at root:
    ISO datetime '{now_str}' is too far from the past, expected between '1 minute' ago and now"#
            )
        );
    }

    #[test]
    fn it_should_not_parse_iso_datetime_ahead_of_now() {
        let now = Utc::now();
        let now_str = (now + ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect::iso_date_time().within_past(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::iso_date_time() error at root:
    ISO datetime '{now_str}' is in the future of now, expected between '1 minute' ago and now"#
            )
        );
    }
}

#[cfg(test)]
mod test_within_future {
    use super::*;
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_parse_iso_datetime_within_future_set() {
        let now = Utc::now();
        let now_str = (now + ChronoDuration::seconds(30)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect::iso_date_time().within_future(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_not_parse_iso_datetime_within_past_too_far() {
        let now = Utc::now();
        let now_str = (now + ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect::iso_date_time().within_future(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::iso_date_time() error at root:
    ISO datetime '{now_str}' is too far in the future, expected between now and '1 minute' in the future"#
            )
        );
    }

    #[test]
    fn it_should_not_parse_iso_datetime_before_now() {
        let now = Utc::now();
        let now_str = (now - ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect::iso_date_time().within_future(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::iso_date_time() error at root:
    ISO datetime '{now_str}' is in the past of now, expected between now and '1 minute' in the future"#
            )
        );
    }

    #[test]
    fn it_should_pass_if_date_within_past_and_future() {
        let now = Utc::now();
        let now_str = now.to_rfc3339();
        let left = json!(now_str);
        let right = json!(
            expect::iso_date_time()
                .within_past(StdDuration::from_secs(60))
                .within_future(StdDuration::from_secs(60))
        );

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_if_date_behind_past_and_future() {
        let now = Utc::now();
        let now_str = (now - ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(
            expect::iso_date_time()
                .within_past(StdDuration::from_secs(60))
                .within_future(StdDuration::from_secs(60))
        );

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::iso_date_time() error at root:
    ISO datetime '{now_str}' is too far from the past, expected between '1 minute' ago and now"#
            )
        );
    }

    #[test]
    fn it_should_fail_if_date_ahead_of_past_and_future() {
        let now = Utc::now();
        let now_str = (now + ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(
            expect::iso_date_time()
                .within_past(StdDuration::from_secs(60))
                .within_future(StdDuration::from_secs(60))
        );

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::iso_date_time() error at root:
    ISO datetime '{now_str}' is too far in the future, expected between now and '1 minute' in the future"#
            )
        );
    }
}
