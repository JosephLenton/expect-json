use crate::internals::objects::ArrayObject;
use crate::internals::objects::StringObject;
use crate::internals::objects::ValueObject;
use crate::internals::types::ValueType;
use crate::internals::types::ValueTypeObject;
use crate::internals::Context;
use crate::SerializeExpectOp;
use serde_json::Value;
use thiserror::Error;

pub type JsonValueEqResult<V> = Result<V, JsonValueEqError>;

#[derive(Debug, Error)]
pub enum JsonValueEqError {
    #[error(
        "Json values at {context} are not equal:
    expected {expected},
    received {received}"
    )]
    DifferentTypes {
        context: Context<'static>,
        received: ValueTypeObject,
        expected: ValueTypeObject,
    },

    #[error(
        "Json {json_type}s at {context} are not equal:
    expected {expected},
    received {received}"
    )]
    DifferentValues {
        context: Context<'static>,
        json_type: ValueType,
        received: ValueObject,
        expected: ValueObject,
    },

    // TODO, this error message should include which operations it _can_ be performed on.
    // The underlying problem might be the server returned different data to what we expected.
    #[error(
        "Json comparison on unsupported type:
    expected operation {} cannot be performed on type {received_type}",
    <&'static str>::from(expected_operation)
    )]
    UnsupportedOperation {
        context: Context<'static>,
        received_type: ValueType,
        expected_operation: SerializeExpectOp,
    },

    #[error(
        "Json arrays at {context} are not equal:
    expected {expected},
        full array {expected_full_array}
    received {received}
        full array {received_full_array}"
    )]
    DifferentArrayTypes {
        context: Context<'static>,
        received: ValueTypeObject,
        received_full_array: ArrayObject,
        expected: ValueTypeObject,
        expected_full_array: ArrayObject,
    },

    #[error(
        "Json at {context} is not equal:
    expected object key '{expected_key}',
    but it was not found"
    )]
    ObjectKeyMissing {
        context: Context<'static>,
        expected_key: String,
    },

    #[error(
        "Json at {context} is not equal:
    expected array index at '{expected_index}',
    but it was not found"
    )]
    ArrayIndexMissing {
        context: Context<'static>,
        expected_index: usize,
    },

    #[error(
        "Json array at {context} does not contain expected value:
    expected array to contain the {expected}, but it was not found.
    received {received_full_array}"
    )]
    ArrayContainsNotFound {
        context: Context<'static>,
        expected: ValueTypeObject,
        received_full_array: ArrayObject,
    },

    #[error(
        "Json string at {context} does not contain expected value:
    expected string to contain {expected}, but it was not found.
    received {received_full_string}"
    )]
    StringContainsNotFound {
        context: Context<'static>,
        expected: StringObject,
        received_full_string: StringObject,
    },

    #[error(
        r#"Json object at {context} has extra field .{received_extra_field}:
    expected {expected_obj},
    received {received_obj}"#
    )]
    ObjectReceivedHasExtraKey {
        context: Context<'static>,
        received_extra_field: String,
        received_obj: ValueTypeObject,
        expected_obj: ValueTypeObject,
    },
}

impl JsonValueEqError {
    pub fn context(&self) -> &Context<'static> {
        match self {
            Self::UnsupportedOperation { context, .. } => context,
            Self::ArrayIndexMissing { context, .. } => context,
            Self::DifferentArrayTypes { context, .. } => context,
            Self::DifferentTypes { context, .. } => context,
            Self::DifferentValues { context, .. } => context,
            Self::ObjectKeyMissing { context, .. } => context,
            Self::ArrayContainsNotFound { context, .. } => context,
            Self::StringContainsNotFound { context, .. } => context,
            Self::ObjectReceivedHasExtraKey { context, .. } => context,
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
            Self::DifferentTypes {
                context,
                received,
                expected,
            } => Self::DifferentArrayTypes {
                context,
                received,
                received_full_array: ArrayObject::from(received_array.to_owned()),
                expected,
                expected_full_array: ArrayObject::from(expected_array.to_owned()),
            },
            _ => source_error,
        }
    }
}
