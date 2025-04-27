use crate::internals::Context;
use crate::internals::ExpectOpMeta;
use crate::ops::ExpectObject;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectObjectSubOp {
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
            ExpectObjectSubOp::Contains(expected_values) => {
                ExpectObjectSubOp::on_object_contains(expected_values, parent, context, received)
            }
        }
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
