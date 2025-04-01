use super::JsonValueEqError;
use super::JsonValueEqResult;
use crate::internals::context::Context;
use crate::SerializeExpect;
use serde_json::Value;

mod json_op_eq_array;
mod json_op_eq_boolean;
mod json_op_eq_null;
mod json_op_eq_number;
mod json_op_eq_object;
mod json_op_eq_string;

pub fn json_op_eq<'a>(
    context: &mut Context<'a>,
    received: &'a Value,
    expected_op: SerializeExpect,
) -> JsonValueEqResult<()> {
    match received {
        Value::Null => json_op_eq_null::json_op_eq_null(context, expected_op.inner),
        Value::Number(received_number) => {
            json_op_eq_number::json_op_eq_number(context, received_number, expected_op.inner)
        }
        Value::String(received_string) => {
            json_op_eq_string::json_op_eq_string(context, received_string, expected_op.inner)
        }
        Value::Bool(received_boolean) => {
            json_op_eq_boolean::json_op_eq_boolean(context, *received_boolean, expected_op.inner)
        }
        Value::Array(received_array) => {
            json_op_eq_array::json_op_eq_array(context, received_array, expected_op.inner)
        }
        Value::Object(received_object) => {
            json_op_eq_object::json_op_eq_object(context, received_object, expected_op.inner)
        }
    }
}
