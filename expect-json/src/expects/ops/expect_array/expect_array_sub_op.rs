use crate::internals::objects::ArrayObject;
use crate::internals::Context;
use crate::ops::ExpectArray;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use crate::JsonType;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectArraySubOp {
    MinLen(usize),
    MaxLen(usize),
    Contains(Vec<Value>),
}

impl ExpectArraySubOp {
    pub(crate) fn on_array(
        &self,
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received: &[Value],
    ) -> ExpectOpResult<()> {
        match self {
            ExpectArraySubOp::MinLen(min_len) => {
                ExpectArraySubOp::on_array_min_len(*min_len, parent, context, received)
            }
            ExpectArraySubOp::MaxLen(max_len) => {
                ExpectArraySubOp::on_array_max_len(*max_len, parent, context, received)
            }
            ExpectArraySubOp::Contains(expected_values) => {
                ExpectArraySubOp::on_array_contains(expected_values, parent, context, received)
            }
        }
    }

    fn on_array_min_len(
        min_len: usize,
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received: &[Value],
    ) -> ExpectOpResult<()> {
        if received.len() < min_len {
            let error_message = format!(
                "Array is too short, expected length at least {}, received length {}",
                min_len,
                received.len()
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_array_max_len(
        max_len: usize,
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received: &[Value],
    ) -> ExpectOpResult<()> {
        if received.len() > max_len {
            let error_message = format!(
                "Array is too long, expected length at most {}, received length {}",
                max_len,
                received.len()
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_array_contains(
        expected_values: &[Value],
        _parent: &ExpectArray,
        context: &mut Context<'_>,
        received_values: &[Value],
    ) -> ExpectOpResult<()> {
        // TODO: This is brute force as we don't know if we are containing an inner ExpectOp.
        // Can this be done without a brute force approach?
        for expected in expected_values {
            let is_found = received_values
                .iter()
                .any(|received| context.json_eq(received, expected).is_ok());

            if !is_found {
                return Err(ExpectOpError::ContainsNotFound {
                    context: context.to_static(),
                    json_type: JsonType::Array,
                    expected: expected.clone().into(),
                    received: ArrayObject::from(received_values.to_owned()).into(),
                });
            }
        }

        Ok(())
    }
}
