use crate::internals::JsonValueEqError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

pub type ExpectJsonEqResult<V> = Result<V, ExpectJsonEqError>;

#[derive(Debug, Error)]
pub enum ExpectJsonEqError {
    #[error("Failed to serialise expected value to Json")]
    FailedToSerialiseExpected(#[source] SerdeJsonError),

    #[error("Failed to serialise other value to Json")]
    FailedToSerialiseOther(#[source] SerdeJsonError),

    #[error("{0}")]
    JsonValueError(#[from] JsonValueEqError),
}
