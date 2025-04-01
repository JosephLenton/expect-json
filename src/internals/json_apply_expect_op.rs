use super::JsonValueEqError;
use super::JsonValueEqResult;
use crate::internals::context::Context;
use crate::SerializeExpect;
use crate::SerializeExpectOp;
use serde_json::Value;
use std::collections::HashSet;

pub fn json_apply_expect_op<'a>(
    context: &mut Context<'a>,
    received: &'a Value,
    expected_op: SerializeExpect<'static>,
) -> JsonValueEqResult<()> {
    match received {
        Value::Null => unimplemented!("todo, null comparisons"),
        Value::Number(l) => unimplemented!("todo, number comparisons"),
        Value::String(l) => unimplemented!("todo, string comparisons"),
        Value::Bool(l) => unimplemented!("todo, bool comparisons"),
        Value::Array(arr_values) => json_apply_expect_op_array(context, arr_values, expected_op),
        Value::Object(l) => unimplemented!("todo, obj comparisons"),
    }
}

fn json_apply_expect_op_array<'a>(
    context: &mut Context<'a>,
    received: &'a Vec<Value>,
    expected_op: SerializeExpect<'static>,
) -> JsonValueEqResult<()> {
    match expected_op.inner {
        SerializeExpectOp::Contains { values } => {
            json_expect_array_contains(context, received, values.into_owned())
        }
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
