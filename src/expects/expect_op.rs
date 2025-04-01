use super::SerializeExpect;
use super::SerializeExpectOp;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

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
    pub fn new(inner: V) -> Self {
        Self { inner }
    }
}

impl<V> Deref for ExpectOp<V>
where
    V: Into<SerializeExpectOp> + Clone,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<V> DerefMut for ExpectOp<V>
where
    V: Into<SerializeExpectOp> + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
