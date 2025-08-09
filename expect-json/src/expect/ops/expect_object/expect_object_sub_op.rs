use crate::expect::ops::ExpectObject;
use crate::expect_core::Context;
use crate::expect_core::ExpectOpError;
use crate::expect_core::ExpectOpResult;
use crate::internals::objects::ObjectObject;
use crate::internals::ExpectOpMeta;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectObjectSubOp {
    IsEmpty,
    IsNotEmpty,
    Contains(Map<String, Value>),
}

impl ExpectObjectSubOp {
    pub(crate) fn on_object(
        &self,
        parent: &ExpectObject,
        context: &mut Context,
        received: &Map<String, Value>,
    ) -> ExpectOpResult<()> {
        match self {
            Self::IsEmpty => Self::on_object_is_empty(parent, context, received),
            Self::IsNotEmpty => Self::on_object_is_not_empty(parent, context, received),
            Self::Contains(expected_values) => {
                Self::on_object_contains(expected_values, parent, context, received)
            }
        }
    }

    fn on_object_is_empty(
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
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_object_is_not_empty(
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
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_object_contains(
        expected_values: &Map<String, Value>,
        parent: &ExpectObject,
        context: &mut Context<'_>,
        received: &Map<String, Value>,
    ) -> ExpectOpResult<()> {
        for (key, expected_value) in expected_values {
            let received_value =
                received
                    .get(key)
                    .ok_or_else(|| ExpectOpError::ObjectKeyMissingForExpectOp {
                        context: context.to_static(),
                        expected_key: key.to_owned(),
                        expected_operation: ExpectOpMeta::new(parent),
                    })?;

            context.push(key.to_owned());
            context.json_eq(received_value, expected_value)?;
            context.pop();
        }

        Ok(())
    }
}
