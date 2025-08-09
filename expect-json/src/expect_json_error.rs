use crate::expect_core::Context;
use crate::expect_core::ExpectOpError;
use crate::internals::objects::ArrayObject;
use crate::internals::objects::ValueObject;
use crate::internals::objects::ValueTypeObject;
use crate::internals::utils::is_unquotable_js_identifier;
use crate::JsonType;
use serde_json::Error as SerdeJsonError;
use serde_json::Value;
use std::fmt::Write;
use thiserror::Error;

pub type ExpectJsonResult<V> = Result<V, ExpectJsonError>;

#[derive(Debug, Error)]
pub enum ExpectJsonError {
    #[error("Failed to serialise expected value to Json")]
    FailedToSerialiseExpected(#[source] SerdeJsonError),

    #[error("Failed to serialise other value to Json")]
    FailedToSerialiseReceived(#[source] SerdeJsonError),

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
        "Json arrays at {context} contain different types:
    expected {expected}
        full array {expected_full_array}
    received {received}
        full array {received_array}"
    )]
    ArrayContainsDifferentTypes {
        context: Context<'static>,
        received: Box<ValueTypeObject>,
        received_array: Box<ArrayObject>,
        expected: Box<ValueTypeObject>,
        expected_full_array: Box<ArrayObject>,
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
        received: Box<ValueObject>,
        received_array: Box<ArrayObject>,
        expected: Box<ValueObject>,
        expected_full_array: Box<ArrayObject>,
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

    #[error("{source}")]
    ExpectOpError {
        #[from]
        source: ExpectOpError,
    },
}

impl ExpectJsonError {
    pub(crate) fn array_index_missing<'a>(
        context: &mut Context<'a>,
        source_error: Self,
        received_array: &'a [Value],
        expected_array: &'a [Value],
    ) -> Self {
        match source_error {
            Self::DifferentValues {
                context: source_error_context,
                json_type,
                received,
                expected,
            } => {
                // If the source is deeper, then it takes precedence.
                if source_error_context != *context {
                    return Self::DifferentValues {
                        context: source_error_context,
                        json_type,
                        received,
                        expected,
                    };
                }

                Self::ArrayContainsDifferentValues {
                    context: source_error_context,
                    json_type,
                    received: Box::new(received),
                    received_array: Box::new(ArrayObject::from(received_array.to_owned())),
                    expected: Box::new(expected),
                    expected_full_array: Box::new(ArrayObject::from(expected_array.to_owned())),
                }
            }
            Self::DifferentTypes {
                context: source_error_context,
                received,
                expected,
            } => {
                // If the source is deeper, then it takes precedence.
                if source_error_context != *context {
                    return Self::DifferentTypes {
                        context: source_error_context,
                        received,
                        expected,
                    };
                }

                Self::ArrayContainsDifferentTypes {
                    context: source_error_context,
                    received: Box::new(received),
                    received_array: Box::new(ArrayObject::from(received_array.to_owned())),
                    expected: Box::new(expected),
                    expected_full_array: Box::new(ArrayObject::from(expected_array.to_owned())),
                }
            }

            // All other errors are left as is.
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
