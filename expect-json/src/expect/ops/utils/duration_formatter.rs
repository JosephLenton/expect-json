use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct DurationFormatter(Duration);

impl DurationFormatter {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl From<Duration> for DurationFormatter {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl Display for DurationFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let total_seconds = self.0.as_secs();
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = (total_seconds / 3600) % 24;
        let days = total_seconds / 86_400;
        let mut has_written = false;

        if days > 0 {
            if days == 1 {
                write!(f, "1 day")?;
            } else {
                write!(f, "{days} days")?;
            }
            has_written = true;
        }

        if hours > 0 {
            if has_written {
                write!(f, ", ")?;
            }
            if hours == 1 {
                write!(f, "1 hour")?;
            } else {
                write!(f, "{hours} hours")?;
            }
            has_written = true;
        }

        if minutes > 0 {
            if has_written {
                write!(f, ", ")?;
            }
            if minutes == 1 {
                write!(f, "1 minute")?;
            } else {
                write!(f, "{minutes} minutes")?;
            }
            has_written = true;
        }

        if seconds > 0 {
            if has_written {
                write!(f, ", ")?;
            }
            if seconds == 1 {
                write!(f, "1 second")?;
            } else {
                write!(f, "{seconds} seconds")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_from {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_convert_duration_to_duration_formatter() {
        let duration = DurationFormatter::from(Duration::from_secs(60) + Duration::from_secs(19));
        let other_duration = DurationFormatter::new(Duration::from_secs(79));

        assert_eq!(duration, other_duration);
    }
}

#[cfg(test)]
mod test_fmt {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_format_duration_with_days() {
        let duration = Duration::from_secs(86_400);
        let formatter = DurationFormatter::new(duration);
        assert_eq!(formatter.to_string(), "1 day");
    }

    #[test]
    fn it_should_format_big_mix_of_values() {
        let duration = Duration::from_secs(456 * 86_400)
            + Duration::from_secs(2 * 3600)
            + Duration::from_secs(3 * 60)
            + Duration::from_secs(4);
        let formatter = DurationFormatter::new(duration);
        assert_eq!(
            formatter.to_string(),
            "456 days, 2 hours, 3 minutes, 4 seconds"
        );
    }

    #[test]
    fn it_should_format_big_mix_of_singular_values() {
        let duration = Duration::from_secs(86_400)
            + Duration::from_secs(3600)
            + Duration::from_secs(60)
            + Duration::from_secs(1);
        let formatter = DurationFormatter::new(duration);
        assert_eq!(formatter.to_string(), "1 day, 1 hour, 1 minute, 1 second");
    }

    #[test]
    fn it_should_format_minutes_and_seconds() {
        let duration = Duration::from_secs(13 * 60) + Duration::from_secs(4);
        let formatter = DurationFormatter::new(duration);
        assert_eq!(formatter.to_string(), "13 minutes, 4 seconds");
    }

    #[test]
    fn it_should_format_seconds() {
        let duration = Duration::from_secs(4);
        let formatter = DurationFormatter::new(duration);
        assert_eq!(formatter.to_string(), "4 seconds");
    }

    #[test]
    fn it_should_format_hours() {
        let duration = Duration::from_secs(60 * 2 * 60);
        let formatter = DurationFormatter::new(duration);
        assert_eq!(formatter.to_string(), "2 hours");
    }
}
