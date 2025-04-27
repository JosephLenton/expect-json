use crate::expect_op;
use crate::internals::Context;
use crate::ops::utils::DurationFormatter;
use crate::ExpectOp;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use crate::JsonType;
use chrono::DateTime;
use chrono::Duration as ChronoDuration;
use chrono::FixedOffset;
use chrono::Offset;
use chrono::Utc;
use std::time::Duration as StdDuration;

///
/// Expects an ISO 8601 date time string.
///
/// By _default_ this expects a UTC timezone, and this can be disabled with [IsoDateTime::allow_non_utc()].
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
/// server.get(&"/latest-comment")
///     .await
///     .assert_json_contains(&json!({
///         "comment": "My example comment",
///         "created_at": expect.iso_date_time(),
///
///         // Expect it was updated in the last minute
///         "updated_at": expect.iso_date_time()
///             .within_past(Duration::from_secs(60)),
///
///         // Expect it also expires in the next minute
///         "expires_at": expect.iso_date_time()
///             .within_future(Duration::from_secs(60)),
///
///         // Users time could have any timezone
///         "users_created_at": expect.iso_date_time()
///             .allow_non_utc(),
///     }));
/// #
/// # Ok(()) }
/// ```
///
#[expect_op(internal)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IsoDateTime {
    is_utc_only: bool,
    maybe_past_duration: Option<StdDuration>,
    maybe_future_duration: Option<StdDuration>,
}

impl IsoDateTime {
    pub(crate) fn new() -> Self {
        Self {
            is_utc_only: true,
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
    pub fn allow_non_utc(self) -> Self {
        Self {
            is_utc_only: false,
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
    pub fn within_future(self, duration: StdDuration) -> Self {
        Self {
            maybe_future_duration: Some(duration),
            ..self
        }
    }
}

impl ExpectOp for IsoDateTime {
    fn on_string(&self, context: &mut Context, received: &str) -> ExpectOpResult<()> {
        let date_time = DateTime::<FixedOffset>::parse_from_rfc3339(received).map_err(|error| {
            let error_message = format!("failed to parse string '{received}' as iso date time");
            ExpectOpError::custom_error(context, self, error_message, error)
        })?;

        if self.is_utc_only {
            let is_date_time_utc = date_time.offset().fix().utc_minus_local() != 0;
            if is_date_time_utc {
                let error_message = format!(
                    "ISO datetime '{received}' is using a non-UTC timezone, expected UTC only"
                );
                return Err(ExpectOpError::custom(context, self, error_message));
            }
        }

        match (self.maybe_past_duration, self.maybe_future_duration) {
            (None, None) => {}
            (Some(past_duration), None) => {
                let is_date_time_outside_past = date_time < Utc::now() - past_duration;
                if is_date_time_outside_past {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(past_duration).unwrap());
                    let error_message = format!("ISO datetime '{received}' is too far from the past, expected between '{duration}' ago and now");
                    return Err(ExpectOpError::custom(context, self, error_message));
                }

                let is_date_time_ahead_of_now = date_time > Utc::now();
                if is_date_time_ahead_of_now {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(past_duration).unwrap());
                    let error_message = format!("ISO datetime '{received}' is in the future of now, expected between '{duration}' ago and now");
                    return Err(ExpectOpError::custom(context, self, error_message));
                }
            }
            (None, Some(future_duration)) => {
                let is_date_time_outside_future = date_time > Utc::now() + future_duration;
                if is_date_time_outside_future {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(future_duration).unwrap());
                    let error_message = format!("ISO datetime '{received}' is too far in the future, expected between now and '{duration}' in the future");
                    return Err(ExpectOpError::custom(context, self, error_message));
                }

                let is_date_time_behind_of_now = date_time < Utc::now();
                if is_date_time_behind_of_now {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(future_duration).unwrap());
                    let error_message = format!("ISO datetime '{received}' is in the past of now, expected between now and '{duration}' in the future");
                    return Err(ExpectOpError::custom(context, self, error_message));
                }
            }
            (Some(past_duration), Some(future_duration)) => {
                let is_date_time_outside_past = date_time < Utc::now() - past_duration;
                if is_date_time_outside_past {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(past_duration).unwrap());
                    let error_message = format!("ISO datetime '{received}' is too far from the past, expected between '{duration}' ago and now");
                    return Err(ExpectOpError::custom(context, self, error_message));
                }

                let is_date_time_outside_future = date_time > Utc::now() + future_duration;
                if is_date_time_outside_future {
                    let duration =
                        DurationFormatter::new(ChronoDuration::from_std(future_duration).unwrap());
                    let error_message = format!("ISO datetime '{received}' is too far in the future, expected between now and '{duration}' in the future");
                    return Err(ExpectOpError::custom(context, self, error_message));
                }
            }
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
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
        let right = json!(expect.iso_date_time());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_to_parse_iso_datetime_with_non_utc_timezone() {
        let left = json!("2024-01-15T13:45:30+01:00");
        let right = json!(expect.iso_date_time());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect.IsoDateTime() error at root:
    ISO datetime '2024-01-15T13:45:30+01:00' is using a non-UTC timezone, expected UTC only"#
        );
    }

    #[test]
    fn it_should_fail_to_parse_iso_datetime_without_timezone() {
        let left = json!("2024-01-15T13:45:30");
        let right = json!(expect.iso_date_time());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect.IsoDateTime() error at root:
    failed to parse string '2024-01-15T13:45:30' as iso date time,
    premature end of input"#
        );
    }
}

#[cfg(test)]
mod test_utc_only {
    use crate::expect;
    use crate::expect_json_eq;
    use serde_json::json;

    #[test]
    fn it_should_parse_iso_datetime_with_utc_timezone_when_set() {
        let left = json!("2024-01-15T13:45:30Z");
        let right = json!(expect.iso_date_time().allow_non_utc());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_parse_iso_datetime_with_non_utc_timezone_when_set() {
        let left = json!("2024-01-15T13:45:30+01:00");
        let right = json!(expect.iso_date_time().allow_non_utc());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
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
        let right = json!(expect
            .iso_date_time()
            .within_past(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_not_parse_iso_datetime_within_past_too_far() {
        let now = Utc::now();
        let now_str = (now - ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect
            .iso_date_time()
            .within_past(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect.IsoDateTime() error at root:
    ISO datetime '{now_str}' is too far from the past, expected between '1 minute' ago and now"#
            )
        );
    }

    #[test]
    fn it_should_not_parse_iso_datetime_ahead_of_now() {
        let now = Utc::now();
        let now_str = (now + ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect
            .iso_date_time()
            .within_past(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect.IsoDateTime() error at root:
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
        let right = json!(expect
            .iso_date_time()
            .within_future(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_not_parse_iso_datetime_within_past_too_far() {
        let now = Utc::now();
        let now_str = (now + ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect
            .iso_date_time()
            .within_future(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect.IsoDateTime() error at root:
    ISO datetime '{now_str}' is too far in the future, expected between now and '1 minute' in the future"#
            )
        );
    }

    #[test]
    fn it_should_not_parse_iso_datetime_before_now() {
        let now = Utc::now();
        let now_str = (now - ChronoDuration::seconds(90)).to_rfc3339();
        let left = json!(now_str);
        let right = json!(expect
            .iso_date_time()
            .within_future(StdDuration::from_secs(60)));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect.IsoDateTime() error at root:
    ISO datetime '{now_str}' is in the past of now, expected between now and '1 minute' in the future"#
            )
        );
    }
}
