use crate::internals::Context;
use crate::internals::JsonValueEqResult;
use crate::ExpectOp;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Offset;

#[crate::expect_op(internal)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IsoDateTime {
    is_utc_only: bool,
}

impl IsoDateTime {
    pub(crate) fn new() -> Self {
        Self { is_utc_only: false }
    }

    pub fn utc_only(self) -> Self {
        Self { is_utc_only: true }
    }
}

impl ExpectOp for IsoDateTime {
    fn on_string(&self, context: &mut Context, received: &str) -> JsonValueEqResult<()> {
        let date_time = DateTime::<FixedOffset>::parse_from_rfc3339(received).map_err(|error| {
            let error_message = format!("failed to parse string '{received}' as iso date time");
            context.custom_err(self, error_message, error)
        })?;

        if self.is_utc_only {
            let is_date_time_utc = date_time.offset().fix().utc_minus_local() != 0;
            if is_date_time_utc {
                let error_message = format!(
                    "ISO datetime '{received}' is using a non-UTC timezone, expected UTC only"
                );
                return Err(context.custom_err_message(self, error_message));
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
    fn it_should_parse_iso_datetime_with_non_utc_timezone() {
        let left = json!("2024-01-15T13:45:30+01:00");
        let right = json!(expect.iso_date_time());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
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
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_parse_iso_datetime_with_utc_timezone_when_set() {
        let left = json!("2024-01-15T13:45:30Z");
        let right = json!(expect.iso_date_time().utc_only());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_parse_iso_datetime_with_non_utc_timezone_when_set() {
        let left = json!("2024-01-15T13:45:30+01:00");
        let right = json!(expect.iso_date_time().utc_only());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect.IsoDateTime() error at root:
    ISO datetime '2024-01-15T13:45:30+01:00' is using a non-UTC timezone, expected UTC only"#
        );
    }
}
