use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct StringObject(String);

impl From<String> for StringObject {
    fn from(inner: String) -> Self {
        Self(inner)
    }
}

impl Display for StringObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, r#""{}""#, self.0)
    }
}
