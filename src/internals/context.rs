use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug)]
pub enum Context {
    Root,
}

impl Display for Context {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Root => write!(formatter, "root"),
        }
    }
}
