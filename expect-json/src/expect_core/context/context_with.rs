use crate::ExpectJsonResult;
use crate::expect_core::Context;
use crate::expect_core::context::ContextPathPart;
use serde_json::Value;
use std::error::Error;

/// A wrapper around the context for temporarily adding values onto it,
/// before calling `json_eq`.
///
/// It will automatically undo everything applied when `json_eq` is called.
#[derive(Debug)]
pub(crate) struct ContextWith<'a, 'c> {
    context: &'a mut Context<'c>,
    previous_is_propagated_contains: bool,
    pushed_paths: usize,
}

impl<'a, 'c> ContextWith<'a, 'c> {
    pub(crate) fn new(context: &'a mut Context<'c>) -> Self {
        let previous_is_propagated_contains = context.is_propagated_contains();

        Self {
            context,
            previous_is_propagated_contains,
            pushed_paths: 0,
        }
    }

    pub fn with_propagated_contains(self) -> Self {
        self.context.enable_propagated_contains();
        self
    }

    pub fn without_propagated_contains(self) -> Self {
        self.context.disable_propagated_contains();
        self
    }

    pub fn with_path<P>(mut self, path: P) -> Self
    where
        P: Into<ContextPathPart<'c>>,
    {
        self.context.push(path);
        self.pushed_paths += 1;
        self
    }

    pub fn json_eq(self, received: &'a Value, expected: &'a Value) -> ExpectJsonResult<()> {
        self.map(|context| context.json_eq(received, expected))
    }

    pub(crate) fn map<F, E>(self, fun: F) -> Result<(), E>
    where
        F: FnOnce(&mut Context) -> Result<(), E>,
        E: Error,
    {
        let context = self.context;
        let previous_is_partial = self.previous_is_propagated_contains;
        let path_count = self.pushed_paths;

        fun(context)?;

        if previous_is_partial {
            context.enable_propagated_contains();
        } else {
            context.disable_propagated_contains();
        }

        for _ in 0..path_count {
            context.pop();
        }

        Ok(())
    }
}
