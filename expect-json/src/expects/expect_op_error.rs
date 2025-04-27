use crate::internals::objects::ArrayObject;
use crate::internals::objects::ValueObject;
use crate::internals::objects::ValueTypeObject;
use crate::internals::ExpectOpMeta;
use crate::Context;
use crate::ExpectJsonError;
use crate::ExpectOp;
use crate::JsonType;
use std::error::Error as StdError;
use thiserror::Error;

pub type ExpectOpResult<V> = Result<V, ExpectOpError>;

#[derive(Debug, Error)]
pub enum ExpectOpError {
    // TODO, this error message should include which operations it _can_ be performed on.
    // The underlying problem might be the server returned different data to what we expected.
    #[error(
        "Json expect::{}() at {context}, received wrong type:
    expected {}
    received {received}",
        expected_operation.name,
        format_expected_operation_types(expected_operation),
    )]
    UnsupportedOperation {
        context: Context<'static>,
        received: ValueTypeObject,
        expected_operation: ExpectOpMeta,
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
        "Json at {context} has key with value, expecting either key not present or different value.
    received {received}"
    )]
    ObjectKeyValueIsEqual {
        context: Context<'static>,
        received: ValueObject,
        expected_operation: ExpectOpMeta,
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
        "Json array at {context} contains duplicate value, expected array to contain all unique values.
    duplicate value {duplicate}.
    received full array {received_array}"
    )]
    ArrayContainsDuplicate {
        context: Context<'static>,
        duplicate: ValueObject,
        received_array: ArrayObject,
    },

    #[error(
        "Json expect::{}() error at {context}:
    {message},
    {error}",
    expected_operation.name,
    )]
    UnknownError {
        #[source]
        error: Box<dyn StdError>,
        context: Context<'static>,
        message: String,
        expected_operation: ExpectOpMeta,
    },

    #[error(
        "Json expect::{}() error at {context}:
    {message}",
    expected_operation.name,
    )]
    UnknownErrorMessage {
        context: Context<'static>,
        message: String,
        expected_operation: ExpectOpMeta,
    },

    #[error("{error}")]
    ExpectJsonError {
        #[source]
        error: Box<ExpectJsonError>,
    },
}

impl ExpectOpError {
    pub fn custom<O, S>(context: &Context<'_>, expect_op: &O, message: S) -> Self
    where
        O: ExpectOp + ?Sized,
        S: Into<String>,
    {
        Self::UnknownErrorMessage {
            context: context.to_static(),
            message: message.into(),
            expected_operation: ExpectOpMeta::new(expect_op),
        }
    }

    pub fn custom_error<O, S, E>(context: &Context<'_>, expect_op: &O, message: S, error: E) -> Self
    where
        O: ExpectOp + ?Sized,
        S: Into<String>,
        E: StdError + 'static,
    {
        Self::UnknownError {
            context: context.to_static(),
            message: message.into(),
            error: Box::new(error),
            expected_operation: ExpectOpMeta::new(expect_op),
        }
    }

    pub fn unsupported_operation_type<O, V>(
        context: &Context<'_>,
        expect_op: &O,
        received: V,
    ) -> Self
    where
        O: ExpectOp + ?Sized,
        V: Into<ValueTypeObject>,
    {
        Self::UnsupportedOperation {
            context: context.to_static(),
            received: received.into(),
            expected_operation: ExpectOpMeta::new(expect_op),
        }
    }
}

impl From<ExpectJsonError> for ExpectOpError {
    fn from(error: ExpectJsonError) -> Self {
        Self::ExpectJsonError {
            error: Box::new(error),
        }
    }
}

fn format_expected_operation_types(expected_operation: &ExpectOpMeta) -> String {
    let types = expected_operation.types;
    if types.is_empty() {
        return "no supported types listed (need to implement ExpectOp::supported_types)"
            .to_string();
    }

    types.join(", ")
}
