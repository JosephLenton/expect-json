use crate::internals::pretty_formatter::PrettyDisplay;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct StringObject(pub String);

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

#[cfg(test)]
mod test_fmt {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_display_empty_string() {
        let string_object = StringObject("".to_string());
        assert_eq!(format!("{string_object}"), r#""""#);
    }

    #[test]
    fn it_should_display_string() {
        let string_object = StringObject("Hello, world!".to_string());
        assert_eq!(format!("{string_object}"), r#""Hello, world!""#);
    }
}
