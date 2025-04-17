use crate::internals::context::context_path_part::ContextPathPart;
use crate::internals::json_eq;
use crate::internals::ExpectOpMeta;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use crate::ExpectOp;
use crate::JsonType;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Context<'a> {
    stack: Vec<ContextPathPart<'a>>,
}

impl<'a> Context<'a> {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn unsupported_expect_op_type<E>(
        &self,
        received_type: JsonType,
        expect_op: &E,
    ) -> JsonValueEqError
    where
        E: ExpectOp + ?Sized,
    {
        JsonValueEqError::UnsupportedOperation {
            context: self.to_static(),
            received_type,
            expected_operation: ExpectOpMeta::new(expect_op),
        }
    }

    pub fn json_eq(&self, received: &'a Value, expected: &'a Value) -> JsonValueEqResult<()> {
        json_eq(&mut self.clone(), received, expected)
    }

    pub(crate) fn push<P>(&mut self, path: P)
    where
        P: Into<ContextPathPart<'a>>,
    {
        self.stack.push(path.into());
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn with_path<P, F>(&mut self, path: P, inner: F) -> JsonValueEqResult<()>
    where
        P: Into<ContextPathPart<'a>>,
        F: FnOnce(&mut Context) -> JsonValueEqResult<()> + 'a,
    {
        self.push(path);
        let result = inner(self);
        self.pop();

        result
    }

    pub(crate) fn to_static(&self) -> Context<'static> {
        let stack = self.stack.iter().map(ContextPathPart::to_static).collect();

        Context { stack }
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
