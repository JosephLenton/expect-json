use crate::internals::objects::ValueObject;
use crate::internals::Context;
use crate::internals::ExpectOpMeta;
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
        "Json comparison on unsupported type, at {context}:
    expect.{}() cannot be performed against {received_type},
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
        "Json expect.{}() error at {context}:
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
        "Json expect.{}() error at {context}:
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
        context: Context<'static>,
        expected_operation: ExpectOpMeta,
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

    pub fn expect_json_error<O>(
        context: &Context<'_>,
        expect_op: &O,
        error: ExpectJsonError,
    ) -> Self
    where
        O: ExpectOp + ?Sized,
    {
        Self::ExpectJsonError {
            context: context.to_static(),
            error: Box::new(error),
            expected_operation: ExpectOpMeta::new(expect_op),
        }
    }

    pub fn unsupported_operation_type<O>(
        context: &Context<'_>,
        expect_op: &O,
        received_type: JsonType,
    ) -> Self
    where
        O: ExpectOp + ?Sized,
    {
        Self::UnsupportedOperation {
            context: context.to_static(),
            received_type,
            expected_operation: ExpectOpMeta::new(expect_op),
        }
    }
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
