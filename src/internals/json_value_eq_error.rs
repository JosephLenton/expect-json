use super::objects::ArrayObject;
use crate::internals::context::Context;
use crate::internals::types::ValueType;
use serde_json::Value;
use thiserror::Error;

pub type JsonValueEqResult<V> = Result<V, JsonValueEqError>;

#[derive(Debug, Error)]
pub enum JsonValueEqError {
    #[error(
        "Json at {context} is not equal,
    expected {expected},
    received {received}"
    )]
    DifferentTypes {
        context: Context<'static>,
        received: ValueType,
        expected: ValueType,
    },

    #[error(
        "Json at {context} is not equal,
    expected {expected},
        full array {expected_full_array}
    received {received}
        full array {received_full_array}"
    )]
    DifferentArrayTypes {
        context: Context<'static>,
        received: ValueType,
        received_full_array: ArrayObject,
        expected: ValueType,
        expected_full_array: ArrayObject,
    },

    #[error(
        "Json at {context} is not equal,
    expected object key '{expected_key}',
    but it was not found"
    )]
    ObjectKeyMissing {
        context: Context<'static>,
        expected_key: String,
    },

    #[error(
        "Json at {context} is not equal,
    expected array index at '{expected_index}',
    but it was not found"
    )]
    ArrayIndexMissing {
        context: Context<'static>,
        expected_index: usize,
    },
}

impl JsonValueEqError {
    pub fn context(&self) -> &Context<'static> {
        match self {
            Self::ArrayIndexMissing { context, .. } => context,
            Self::DifferentArrayTypes { context, .. } => context,
            Self::DifferentTypes { context, .. } => context,
            Self::ObjectKeyMissing { context, .. } => context,
        }
    }

    pub fn array_index_missing<'a>(
        context: &mut Context<'a>,
        source_error: Self,
        received_array: &'a [Value],
        expected_array: &'a [Value],
    ) -> Self {
        // If the source is deeper, then it takes precedence
        if context != source_error.context() {
            return source_error;
        }

        match source_error {
            Self::ArrayIndexMissing { .. } => panic!("Logic error, an array index missing within an array index missing should not be possible, with the same context"),
            Self::DifferentArrayTypes { .. } => panic!("Logic error, different array types within an array index missing should not be possible, with the same context"),
            Self::DifferentTypes { context, received, expected } => {
                Self::DifferentArrayTypes {
                    context,
                    received,
                    received_full_array: ArrayObject::from(received_array.to_owned()),
                    expected,
                    expected_full_array: ArrayObject::from(expected_array.to_owned()),
                }
            },
            Self::ObjectKeyMissing { .. } => {
                panic!("Logic error, object key missing within an array index missing should not be possible, with the same context")
            },
        }
    }
}
