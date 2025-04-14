use crate::expects::SerializeExpect;
use crate::expects::SerializeExpectOp;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize)]
#[serde(into = "SerializeExpect")]
pub struct ExpectOp<V>
where
    V: Into<SerializeExpectOp> + Clone,
{
    inner: V,
}

impl<V> ExpectOp<V>
where
    V: Into<SerializeExpectOp> + Clone,
{
    pub(crate) fn new(inner: V) -> Self {
        Self { inner }
    }
}

impl<V> From<ExpectOp<V>> for SerializeExpect
where
    V: Into<SerializeExpectOp> + Clone,
{
    fn from(expect_op: ExpectOp<V>) -> Self {
        let inner = Into::<SerializeExpectOp>::into(expect_op.inner);
        SerializeExpect::from(inner)
    }
}
