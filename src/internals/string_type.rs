use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct StringType(String);

impl StringType {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<String> for StringType {
    fn from(inner: String) -> Self {
        Self(inner)
    }
}

impl Display for StringType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, r#"string "{}""#, self.0)
    }
}
