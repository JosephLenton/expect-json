use super::Context;
use crate::internals::ValueType;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use thiserror::Error;

const MAX_STRING_ERROR_LEN: usize = 80;

pub type JsonValueEqResult<V> = Result<V, JsonValueEqError>;

#[derive(Debug, Error)]
pub enum JsonValueEqError {
    #[error("At {context}, expected type '{expected_type}', received '{received_type}'")]
    DifferentTypes {
        context: Context,
        expected_type: ValueType,
        received_type: ValueType,
    },

    #[error("At {context}, expected '{expected}', received '{received}'")]
    DifferentBooleanValues {
        context: Context,
        expected: bool,
        received: bool,
    },

    #[error("At {context},{}", FormatErrorStrings(expected, received))]
    DifferentStringValues {
        context: Context,
        expected: String,
        received: String,
    },
}

struct FormatErrorStrings<'a>(&'a str, &'a str);
impl Display for FormatErrorStrings<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let expected = self.0;
        let received = self.1;

        let total_len = expected.len() + received.len();
        if total_len > MAX_STRING_ERROR_LEN {
            write!(
                formatter,
                r#"
    expected "{expected}",
    received "{received}""#
            )
        } else {
            write!(
                formatter,
                r#" expected "{expected}", received "{received}""#
            )
        }
    }
}
