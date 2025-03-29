use crate::internals::ContextPathPart;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Default, Debug)]
pub struct Context<'a> {
    stack: Vec<ContextPathPart<'a>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<P>(&mut self, path: P)
    where
        P: Into<ContextPathPart<'a>>,
    {
        self.stack.push(path.into());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn to_static(&self) -> Context<'static> {
        let stack = self.stack.iter().map(|path| path.to_static()).collect();

        Context { stack }
    }
}

impl Display for Context<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        if self.stack.is_empty() {
            return write!(formatter, "root");
        }

        for path in &self.stack {
            write!(formatter, ".{path}")?;
        }

        Ok(())
    }
}
