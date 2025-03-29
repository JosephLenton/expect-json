use super::Context;
use crate::internals::ValueType;
use thiserror::Error;

pub type JsonValueEqResult<V> = Result<V, JsonValueEqError>;

#[derive(Debug, Error)]
pub enum JsonValueEqError {
    #[error(
        "At {context},
    expected {expected},
    received {received}"
    )]
    DifferentTypes {
        context: Context<'static>,
        expected: ValueType,
        received: ValueType,
    },

    #[error(
        "At {context},
    expected object key '{expected_key}',
    but it was not found"
    )]
    ObjectKeyMissing {
        context: Context<'static>,
        expected_key: String,
    },

    #[error(
        "At {context},
    expected array index at '{expected_index}',
    but it was not found"
    )]
    ArrayIndexMissing {
        context: Context<'static>,
        expected_index: usize,
    },
}
