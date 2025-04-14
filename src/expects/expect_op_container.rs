use crate::expects::ExpectOp;
use crate::expects::SerializeExpectOp;
use crate::internals;
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
    pub(crate) fn new(inner: V) -> Self {
        Self { inner }
    }
}

impl<V> Serialize for ExpectOpContainer<V>
where
    V: ExpectOp + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (store_key, op_key) = internals::expect_store::store(self.inner.clone());
        let serialized = SerializeExpectOp::new(store_key, op_key);

        serialized.serialize(serializer)
    }
}
