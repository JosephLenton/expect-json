use crate::internals::pretty_formatter::PrettyDisplay;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BooleanObject(pub bool);

impl From<bool> for BooleanObject {
    fn from(inner: bool) -> Self {
        Self(inner)
    }
}

impl Display for BooleanObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.0)
    }
}

impl PrettyDisplay for BooleanObject {}
