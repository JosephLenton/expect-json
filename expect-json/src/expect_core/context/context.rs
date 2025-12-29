use crate::ExpectJsonResult;
use crate::expect_core::ContextWith;
use crate::expect_core::context::ContextPathPart;
use crate::internals::json_eq;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Context<'c> {
    stack: Vec<ContextPathPart<'c>>,
    is_propagated_contains: bool,
}

impl<'c> Context<'c> {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn json_eq(&self, received: &'c Value, expected: &'c Value) -> ExpectJsonResult<()> {
        json_eq(&mut self.clone(), received, expected)
    }

    pub(crate) fn with_path<'a, P>(&'a mut self, path: P) -> ContextWith<'a, 'c>
    where
        P: Into<ContextPathPart<'c>>,
    {
        ContextWith::new(self).with_path(path)
    }

    pub(crate) fn with_propagated_contains<'a>(&'a mut self) -> ContextWith<'a, 'c> {
        ContextWith::new(self).with_propagated_contains()
    }

    pub(crate) fn without_propagated_contains<'a>(&'a mut self) -> ContextWith<'a, 'c> {
        ContextWith::new(self).without_propagated_contains()
    }

    pub(crate) fn enable_propagated_contains(&mut self) {
        self.is_propagated_contains = true;
    }

    pub(crate) fn disable_propagated_contains(&mut self) {
        self.is_propagated_contains = false;
    }

    pub(crate) fn is_propagated_contains(&self) -> bool {
        self.is_propagated_contains
    }

    pub(crate) fn push<P>(&mut self, path: P)
    where
        P: Into<ContextPathPart<'c>>,
    {
        self.stack.push(path.into());
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn to_static(&self) -> Context<'static> {
        let stack = self.stack.iter().map(ContextPathPart::to_static).collect();

        Context { stack, ..*self }
    }
}

impl Display for Context<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "root")?;

        for path in &self.stack {
            write!(formatter, "{path}")?;
        }

        Ok(())
    }
}
