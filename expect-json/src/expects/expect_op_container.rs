use crate::expects::ExpectOp;
use crate::expects::SerializeExpectOp;
use crate::ExpectOpSerialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ExpectOpContainer<V>
where
    V: ExpectOp + Clone,
{
    inner: V,
}

impl<V> ExpectOpContainer<V>
where
    V: ExpectOp + Clone,
{
    pub fn new(inner: V) -> Self {
        Self { inner }
    }
}

impl<V> Serialize for ExpectOpContainer<V>
where
    V: ExpectOpSerialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializeExpectOp::new(Box::new(self.inner.clone())).serialize(serializer)
    }
}
