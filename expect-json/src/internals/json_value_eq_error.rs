use crate::internals::objects::ArrayObject;
use crate::internals::objects::ValueObject;
use crate::internals::objects::ValueTypeObject;
use crate::internals::utils::is_unquotable_js_identifier;
use crate::internals::Context;
use crate::internals::ExpectOpMeta;
use crate::JsonType;
use serde_json::Value;
use std::error::Error as StdError;
use std::fmt::Write;
use thiserror::Error;

pub type JsonValueEqResult<V> = Result<V, JsonValueEqError>;

#[derive(Debug, Error)]
pub enum JsonValueEqError {
    #[error(
        "Json {} at {context} are different types:
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
        json_type: JsonType,
        received: ValueObject,
        expected: ValueObject,
    },

    #[error(
        "Json is not null at {context}, expected null:
    expected null
    received {received}"
    )]
    ReceivedIsNotNull {
        context: Context<'static>,
        received: ValueTypeObject,
    },

    #[error(
        "Json null received at {context}, expected not null:
    expected {expected}
    received null"
    )]
    ReceivedIsNull {
        context: Context<'static>,
        expected: ValueTypeObject,
    },

    // TODO, this error message should include which operations it _can_ be performed on.
    // The underlying problem might be the server returned different data to what we expected.
    #[error(
        "Json comparison on unsupported type, at {context}:
    operation {} cannot be performed against {received_type},
    {}",
    expected_operation.name,
    format_expected_operation_types(expected_operation)
    )]
    UnsupportedOperation {
        context: Context<'static>,
        received_type: JsonType,
        expected_operation: ExpectOpMeta,
    },

    #[error(
        "Json objects at {context} are not equal:
    expected field '{expected_key}',
    but it was not found"
    )]
    ObjectKeyMissing {
        context: Context<'static>,
        expected_key: String,
    },

    #[error(
        "Json object at {context} is missing key for {}:
    expected field '{expected_key}',
    but it was not found",
        expected_operation.name
    )]
    ObjectKeyMissingForExpectOp {
        context: Context<'static>,
        expected_key: String,
        expected_operation: ExpectOpMeta,
    },

    #[error(
        "Json arrays at {context} contain different types:
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
        json_type: JsonType,
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
    ArrayMissingInMiddle {
        context: Context<'static>,
        received_array: ArrayObject,
        expected_array: ArrayObject,
        // TODO, get this working.
        // It should display all of the items found in Expected, and not in Received.
        // Taking into account that some items might be missing as duplicates.
        // missing_in_received: ArrayObject,
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
        "Json arrays at {context} are not equal:
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
        "Json arrays at {context} are not equal, received {} extra {} at the end:
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
        "Json arrays at {context} are not equal, received {} extra {} at the start:
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
        r#"Json object at {context} has many extra fields over expected:
    expected {expected_obj}
    received {received_obj}

    extra fields in received:
{}"#,
        format_extra_fields(received_extra_fields)
    )]
    ObjectReceivedHasExtraKeys {
        context: Context<'static>,
        received_extra_fields: Vec<String>,
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
        json_type: JsonType,
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
        json_type: JsonType,
        expected: ValueObject,
        received: ValueObject,
    },

    #[error(
        "Json expect {} at {context} ran into an error:
    {error}",
    expected_operation.name,
    )]
    UnknownError {
        #[source]
        error: Box<dyn StdError>,
        context: Context<'static>,
        expected_operation: ExpectOpMeta,
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
            Self::ArrayMissingInMiddle { context, .. } => context,
            Self::ArrayExtraAtEnd { context, .. } => context,
            Self::ArrayExtraAtStart { context, .. } => context,
            Self::ArrayContainsDifferentTypes { context, .. } => context,
            Self::ArrayContainsDifferentValues { context, .. } => context,

            Self::DifferentTypes { context, .. } => context,
            Self::DifferentValues { context, .. } => context,

            Self::ReceivedIsNotNull { context, .. } => context,
            Self::ReceivedIsNull { context, .. } => context,

            Self::ObjectKeyMissing { context, .. } => context,
            Self::ObjectKeyMissingForExpectOp { context, .. } => context,
            Self::ObjectReceivedHasExtraKey { context, .. } => context,
            Self::ObjectReceivedHasExtraKeys { context, .. } => context,

            // Operations
            Self::ContainsFound { context, .. } => context,
            Self::ContainsNotFound { context, .. } => context,

            Self::UnknownError { context, .. } => context,
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

fn format_extra_fields(received_extra_fields: &[String]) -> String {
    let mut output = String::new();

    for field in received_extra_fields {
        if is_unquotable_js_identifier(field) {
            let _ = writeln!(output, "        {field},");
        } else {
            let _ = writeln!(output, r#"        "{field}","#);
        }
    }

    output
}

fn format_expected_operation_types(expected_operation: &ExpectOpMeta) -> String {
    let types = expected_operation.types;
    if types.is_empty() {
        "this isn't supported on any types".to_string()
    } else if types.len() == 1 {
        let supported_type = types[0];
        format!("only supported type is: {supported_type}")
    } else {
        let supported_types = types.join(", ");
        format!("supported types are: {supported_types}")
    }
}
