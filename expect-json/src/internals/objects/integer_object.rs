use crate::internals::pretty_formatter::PrettyDisplay;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IntegerObject {
    Positive(u64),
    Negative(i64),
}

impl From<u64> for IntegerObject {
    fn from(num: u64) -> Self {
        Self::Positive(num)
    }
}

impl From<i64> for IntegerObject {
    fn from(num: i64) -> Self {
        Self::Negative(num)
    }
}

impl Display for IntegerObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::Positive(n) => write!(formatter, "{}", n),
            Self::Negative(n) => write!(formatter, "{}", n),
        }
    }
}

impl PrettyDisplay for IntegerObject {}
