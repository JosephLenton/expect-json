use crate::internals::utils::is_unquotable_js_identifier;
use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub enum ContextPathPart<'a> {
    String(Cow<'a, String>),
    Index(usize),
}

impl ContextPathPart<'_> {
    pub fn to_static(&self) -> ContextPathPart<'static> {
        match self {
            Self::String(inner) => {
                let cloned_inner = inner.clone().into_owned();
                let cow = Cow::<'static, String>::Owned(cloned_inner);
                ContextPathPart::String(cow)
            }
            Self::Index(index) => ContextPathPart::Index(*index),
        }
    }
}

impl Display for ContextPathPart<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::String(inner) => {
                if is_unquotable_js_identifier(inner.as_str()) {
                    write!(formatter, ".{inner}")
                } else {
                    write!(formatter, r#"."{inner}""#)
                }
            }
            Self::Index(inner) => write!(formatter, "[{inner}]"),
        }
    }
}

impl<'a> From<&'a String> for ContextPathPart<'a> {
    fn from(inner: &'a String) -> Self {
        Self::String(Cow::Borrowed(inner))
    }
}

impl From<usize> for ContextPathPart<'_> {
    fn from(inner: usize) -> Self {
        Self::Index(inner)
    }
}

#[cfg(test)]
mod test_fmt {
    use super::*;

    #[test]
    fn it_should_print_whole_paths_without_quotes_when_an_identifier() {
        let path = ContextPathPart::String(std::borrow::Cow::Owned("example".to_string()));
        let output = format!("{path}");

        assert_eq!(output, r#".example"#);
    }

    #[test]
    fn it_should_print_paths_with_quotes_when_not_an_identifier() {
        let path = ContextPathPart::String(std::borrow::Cow::Owned("my-example".to_string()));
        let output = format!("{path}");

        assert_eq!(output, r#"."my-example""#);
    }

    #[test]
    fn it_should_print_empty_paths_as_quotes() {
        let path = ContextPathPart::String(std::borrow::Cow::Owned("".to_string()));
        let output = format!("{path}");

        assert_eq!(output, r#"."""#);
    }

    #[test]
    fn it_should_print_paths_with_spaces_with_quotes() {
        let path = ContextPathPart::String(std::borrow::Cow::Owned("".to_string()));
        let output = format!("{path}");

        assert_eq!(output, r#"."""#);
    }
}

#[cfg(test)]
mod test_from {
    use super::*;

    #[test]
    fn it_should_convert_strings_to_string_paths() {
        let path_raw = format!("my_path");
        let path = ContextPathPart::from(&path_raw);

        assert_eq!(path, ContextPathPart::String(Cow::Borrowed(&path_raw)));
    }

    #[test]
    fn it_should_convert_usize_to_index_paths() {
        let path = ContextPathPart::from(123_usize);

        assert_eq!(path, ContextPathPart::Index(123));
    }
}
