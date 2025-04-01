use super::JsonValueEqError;
use super::JsonValueEqResult;
use crate::internals::context::Context;
use crate::internals::types::ValueType;
use crate::SerializeExpectOp;
use serde_json::Value;
use std::collections::HashSet;

pub fn json_op_eq_array<'a>(
    context: &mut Context<'a>,
    received: &'a Vec<Value>,
    expected_operation: SerializeExpectOp,
) -> JsonValueEqResult<()> {
    match expected_operation {
        SerializeExpectOp::Contains(contains) => {
            json_expect_array_contains(context, received, contains.values)
        }

        #[allow(unreachable_patterns)]
        _ => Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Array,
            expected_operation,
        }),
    }
}

fn json_expect_array_contains<'a>(
    context: &mut Context<'a>,
    received_values: &'a Vec<Value>,
    expected_values: Vec<Value>,
) -> JsonValueEqResult<()> {
    let received_items_in_set = received_values.into_iter().collect::<HashSet<&'a Value>>();

    for expected in expected_values {
        if !received_items_in_set.contains(&expected) {
            return Err(JsonValueEqError::ArrayContainsNotFound {
                context: context.to_static(),
                expected: expected.into(),
                received_full_array: received_values.clone().into(),
            });
        }
    }

    Ok(())
}
