use crate::expects::ExpectOp;
use crate::ExpectMagicId;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

thread_local! {
    static SERIALIZATION_DEPTH: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpectOpContainer2<V>
where
    V: ExpectOp + Clone,
{
    Root { magic_id: ExpectMagicId, inner: V },

    Lead(V),
}

impl<V> From<V> for ExpectOpContainer2<V>
where
    V: ExpectOp + Clone,
{
    fn from(value: V) -> Self {
        let current_depth =
            SERIALIZATION_DEPTH.with(|call_depth| call_depth.fetch_add(1, Ordering::Acquire));

        let result = if current_depth == 0 {
            Self::Root {
                magic_id: ExpectMagicId::default(),
                inner: value,
            }
        } else {
            Self::Lead(value)
        };

        SERIALIZATION_DEPTH.with(|call_depth| {
            call_depth.fetch_sub(1, Ordering::Release);
        });

        result
    }
}
