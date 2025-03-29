use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug)]
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
            Self::String(inner) => write!(formatter, "{inner}"),
            Self::Index(inner) => write!(formatter, "{inner}"),
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
