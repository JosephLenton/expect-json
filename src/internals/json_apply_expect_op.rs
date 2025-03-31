use super::objects::ArrayObject;
use super::objects::BooleanObject;
use super::objects::NumberObject;
use super::objects::ObjectObject;
use super::objects::StringObject;
use super::JsonObject;
use super::JsonValueEqError;
use super::JsonValueEqResult;
use crate::internals::context::Context;
use crate::internals::types::ValueType;
use crate::SerializeExpect;
use crate::SerializeExpectOp;
use serde_json::Number;
use serde_json::Value;

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
    received: &'a Vec<Value>,
    expected: Vec<Value>,
) -> JsonValueEqResult<()> {
    for value in expected {
        if !received.contains(&value) {
            panic!("dkdkdk")
        }
    }
    Ok(())
}
