use crate::expect_op;
use crate::internals::Context;
use crate::ops::utils::DurationFormatter;
use crate::ExpectOp;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use chrono::DateTime;
use chrono::Duration as ChronoDuration;
use chrono::FixedOffset;
use chrono::Offset;
use chrono::Utc;
use std::time::Duration as StdDuration;

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

    pub fn allow_non_utc(self) -> Self {
        Self {
            is_utc_only: false,
            ..self
        }
    }

    pub fn within_past(self, duration: StdDuration) -> Self {
        Self {
            maybe_past_duration: Some(duration),
            ..self
        }
    }

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

        if let Some(past_duration) = self.maybe_past_duration {
            let is_date_time_outside_past = date_time < Utc::now() - past_duration;
            if is_date_time_outside_past {
                let duration =
                    DurationFormatter::new(ChronoDuration::from_std(past_duration).unwrap());
                let error_message = format!("ISO datetime '{received}' is too far from the past, expected within '{duration}' ago of now");
                return Err(ExpectOpError::custom(context, self, error_message));
            }
        }

        if let Some(future_duration) = self.maybe_future_duration {
            let is_date_time_outside_future = date_time > Utc::now() + future_duration;
            if is_date_time_outside_future {
                let duration =
                    DurationFormatter::new(ChronoDuration::from_std(future_duration).unwrap());
                let error_message = format!("ISO datetime '{received}' is too far in the future, expected within '{duration}' from now");
                return Err(ExpectOpError::custom(context, self, error_message));
            }
        }

        Ok(())
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
    ISO datetime '{now_str}' is too far from the past, expected within '1 minute' ago of now"#
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
    ISO datetime '{now_str}' is too far in the future, expected within '1 minute' from now"#
            )
        );
    }
}
