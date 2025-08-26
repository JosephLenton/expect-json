use crate::expect::ops::ExpectObject;
use crate::expect_core::Context;
use crate::expect_core::ExpectOpError;
use crate::expect_core::ExpectOpResult;
use crate::internals::json_value_eq::json_value_eq_object_contains;
use crate::internals::objects::ObjectObject;
use crate::internals::ExpectOpMeta;
use crate::ExpectJsonError;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectObjectSubOp {
    Empty,
    NotEmpty,
    Contains(Map<String, Value>),
    PartialContains(Map<String, Value>),
}

impl ExpectObjectSubOp {
    pub(crate) fn on_object(
        &self,
        parent: &ExpectObject,
        context: &mut Context,
        received: &Map<String, Value>,
    ) -> ExpectOpResult<()> {
        match self {
            Self::Empty => on_object_empty(parent, context, received),
            Self::NotEmpty => on_object_not_empty(parent, context, received),
            Self::Contains(expected_values) => {
                on_object_contains(parent, context, expected_values, received)
            }
            Self::PartialContains(expected_values) => {
                on_object_propagated_contains(parent, context, expected_values, received)
            }
        }
    }
}

fn on_object_empty(
    parent: &ExpectObject,
    context: &mut Context<'_>,
    received: &Map<String, Value>,
) -> ExpectOpResult<()> {
    if !received.is_empty() {
        let error_message = format!(
            r#"expected empty object
    received {}"#,
            ObjectObject::from(received.clone())
        );
        return Err(ExpectOpError::custom(parent, context, error_message));
    }

    Ok(())
}

fn on_object_not_empty(
    parent: &ExpectObject,
    context: &mut Context<'_>,
    received: &Map<String, Value>,
) -> ExpectOpResult<()> {
    if received.is_empty() {
        let error_message = format!(
            r#"expected non-empty object
    received {}"#,
            ObjectObject::from(received.clone())
        );
        return Err(ExpectOpError::custom(parent, context, error_message));
    }

    Ok(())
}

fn on_object_contains(
    parent: &ExpectObject,
    context: &mut Context<'_>,
    expected: &Map<String, Value>,
    received: &Map<String, Value>,
) -> ExpectOpResult<()> {
    json_value_eq_object_contains(&mut context.clone(), received, expected).map_err(|err| match err
    {
        ExpectJsonError::ObjectKeyMissing {
            context,
            expected_key,
        } => ExpectOpError::ObjectKeyMissingForExpectOp {
            context,
            expected_key,
            expected_operation: ExpectOpMeta::new(parent),
        },
        err => err.into(),
    })
}

fn on_object_propagated_contains(
    parent: &ExpectObject,
    context: &mut Context<'_>,
    expected_values: &Map<String, Value>,
    received: &Map<String, Value>,
) -> ExpectOpResult<()> {
    context
        .with_propagated_contains()
        .map(|context| on_object_contains(parent, context, expected_values, received))
}
