use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BooleanType(bool);

impl From<bool> for BooleanType {
    fn from(inner: bool) -> Self {
        Self(inner)
    }
}

impl Display for BooleanType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "boolean {}", self.0)
    }
}
