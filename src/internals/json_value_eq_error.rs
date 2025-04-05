use crate::internals::objects::ArrayObject;
use crate::internals::objects::ValueObject;
use crate::internals::types::ValueType;
use crate::internals::types::ValueTypeObject;
use crate::internals::Context;
use crate::SerializeExpectOp;
use serde_json::Value;
use thiserror::Error;

pub type JsonValueEqResult<V> = Result<V, JsonValueEqError>;

#[derive(Debug, Error, PartialEq)]
pub enum JsonValueEqError {
    #[error(
        "Json {} at {context} are not equal:
    expected {expected}
    received {received}",
        value_or_number_type_name(received, expected)
    )]
    DifferentTypes {
        context: Context<'static>,
        received: ValueTypeObject,
        expected: ValueTypeObject,
    },

    #[error(
        "Json {json_type}s at {context} are not equal:
    expected {expected}
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
        "Json at {context} are not equal:
    expected object key '{expected_key}',
    but it was not found"
    )]
    ObjectKeyMissing {
        context: Context<'static>,
        expected_key: String,
    },

    #[error(
        "Json arrays at {context} are not equal:
    expected {expected}
        full array {expected_full_array}
    received {received}
        full array {received_array}"
    )]
    ArrayContainsDifferentTypes {
        context: Context<'static>,
        received: ValueTypeObject,
        received_array: ArrayObject,
        expected: ValueTypeObject,
        expected_full_array: ArrayObject,
    },

    #[error(
        "Json {json_type}s at {context} are not equal:
    expected {expected}
        full array {expected_full_array}
    received {received}
        full array {received_array}"
    )]
    ArrayContainsDifferentValues {
        context: Context<'static>,
        json_type: ValueType,
        received: ValueObject,
        received_array: ArrayObject,
        expected: ValueObject,
        expected_full_array: ArrayObject,
    },

    #[error(
        "Json arrays at {context} are not equal:
    expected {expected_array}
    received {received_array}"
    )]
    ArrayValuesAreDifferent {
        context: Context<'static>,
        received_array: ArrayObject,
        expected_array: ArrayObject,
    },

    #[error(
        "Json at {context} are not equal:
    expected array index at '{expected_index}',
    but it was not found"
    )]
    ArrayIndexMissing {
        context: Context<'static>,
        expected_index: usize,
    },

    #[error(
        "Json arrays at {context} are not equal, missing {} {} at the end:
    expected {expected_array}
    received {received_array}
     missing {missing_in_received}"
     , missing_in_received.len(), pluralise_item_word(missing_in_received.len())
    )]
    ArrayMissingAtEnd {
        context: Context<'static>,
        expected_array: ArrayObject,
        received_array: ArrayObject,
        missing_in_received: ArrayObject,
    },

    #[error(
        "Json arrays at {context} are not equal, missing {} {} from the start:
    expected {expected_array}
    received {received_array}
     missing {missing_in_received}"
     , missing_in_received.len(), pluralise_item_word(missing_in_received.len())
    )]
    ArrayMissingAtStart {
        context: Context<'static>,
        expected_array: ArrayObject,
        received_array: ArrayObject,
        missing_in_received: ArrayObject,
    },

    #[error(
        "Json arrays at {context} are not equal, received has {} extra {} at the end:
    expected {expected_array}
    received {received_array}
       extra {extra_in_received}"
     , extra_in_received.len(), pluralise_item_word(extra_in_received.len())
    )]
    ArrayExtraAtEnd {
        context: Context<'static>,
        expected_array: ArrayObject,
        received_array: ArrayObject,
        extra_in_received: ArrayObject,
    },

    #[error(
        "Json arrays at {context} are not equal, received has {} extra {} at the start:
    expected {expected_array}
    received {received_array}
       extra {extra_in_received}"
     , extra_in_received.len(), pluralise_item_word(extra_in_received.len())
    )]
    ArrayExtraAtStart {
        context: Context<'static>,
        expected_array: ArrayObject,
        received_array: ArrayObject,
        extra_in_received: ArrayObject,
    },

    #[error(
        r#"Json object at {context} has extra field "{received_extra_field}":
    expected {expected_obj}
    received {received_obj}"#
    )]
    ObjectReceivedHasExtraKey {
        context: Context<'static>,
        received_extra_field: String,
        received_obj: ValueObject,
        expected_obj: ValueObject,
    },

    #[error(
        "Json {json_type} at {context} contains value was expecting to not be there:
    expected {json_type} to not contain {expected}, but it was found.
    received {received}"
    )]
    ContainsFound {
        context: Context<'static>,
        json_type: ValueType,
        expected: ValueObject,
        received: ValueObject,
    },

    #[error(
        "Json {json_type} at {context} does not contain expected value:
    expected {json_type} to contain {expected}, but it was not found.
    received {received}"
    )]
    ContainsNotFound {
        context: Context<'static>,
        json_type: ValueType,
        expected: ValueObject,
        received: ValueObject,
    },
}

impl JsonValueEqError {
    pub fn context(&self) -> &Context<'static> {
        match self {
            Self::UnsupportedOperation { context, .. } => context,

            Self::ArrayValuesAreDifferent { context, .. } => context,
            Self::ArrayIndexMissing { context, .. } => context,
            Self::ArrayMissingAtEnd { context, .. } => context,
            Self::ArrayMissingAtStart { context, .. } => context,
            Self::ArrayExtraAtEnd { context, .. } => context,
            Self::ArrayExtraAtStart { context, .. } => context,
            Self::ArrayContainsDifferentTypes { context, .. } => context,
            Self::ArrayContainsDifferentValues { context, .. } => context,

            Self::DifferentTypes { context, .. } => context,
            Self::DifferentValues { context, .. } => context,

            Self::ObjectKeyMissing { context, .. } => context,
            Self::ObjectReceivedHasExtraKey { context, .. } => context,

            // Operations
            Self::ContainsFound { context, .. } => context,
            Self::ContainsNotFound { context, .. } => context,
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
            Self::DifferentValues {
                context,
                json_type,
                received,
                expected,
            } => Self::ArrayContainsDifferentValues {
                context,
                json_type,
                received,
                received_array: ArrayObject::from(received_array.to_owned()),
                expected,
                expected_full_array: ArrayObject::from(expected_array.to_owned()),
            },
            Self::DifferentTypes {
                context,
                received,
                expected,
            } => Self::ArrayContainsDifferentTypes {
                context,
                received,
                received_array: ArrayObject::from(received_array.to_owned()),
                expected,
                expected_full_array: ArrayObject::from(expected_array.to_owned()),
            },
            _ => source_error,
        }
    }
}

fn pluralise_item_word(len: usize) -> &'static str {
    if len == 1 {
        "item"
    } else {
        "items"
    }
}

fn value_or_number_type_name(left: &ValueTypeObject, right: &ValueTypeObject) -> &'static str {
    if left.is_number() && right.is_number() {
        "numbers"
    } else {
        "values"
    }
}
