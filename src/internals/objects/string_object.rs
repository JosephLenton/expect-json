use crate::internals::pretty_formatter::PrettyDisplay;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct StringObject(String);

impl<S> From<S> for StringObject
where
    S: Into<String>,
{
    fn from(inner: S) -> Self {
        Self(inner.into())
    }
}

impl Display for StringObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, r#""{}""#, self.0)
    }
}

impl PrettyDisplay for StringObject {}
