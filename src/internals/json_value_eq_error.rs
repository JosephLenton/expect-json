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
        context: Context,
        expected: ValueType,
        received: ValueType,
    },

    #[error(
        "At {context},
    expected '{expected}',
    received '{received}'"
    )]
    DifferentBooleanValues {
        context: Context,
        expected: bool,
        received: bool,
    },
}
