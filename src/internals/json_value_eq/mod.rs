use crate::internals::context::Context;
use crate::internals::types::ValueTypeObject;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use serde_json::Value;

mod json_value_eq_array;
mod json_value_eq_boolean;
mod json_value_eq_number;
mod json_value_eq_object;
mod json_value_eq_string;

pub fn json_value_eq<'a>(
    context: &mut Context<'a>,
    received: &'a Value,
    expected: &'a Value,
) -> JsonValueEqResult<()> {
    match (received, expected) {
        (Value::Null, Value::Null) => Ok(()),
        (Value::Number(l), Value::Number(r)) => {
            json_value_eq_number::json_value_eq_number(context, l, r)
        }
        (Value::String(l), Value::String(r)) => {
            json_value_eq_string::json_value_eq_string(context, l, r)
        }
        (Value::Bool(l), Value::Bool(r)) => {
            json_value_eq_boolean::json_value_eq_boolean(context, *l, *r)
        }
        (Value::Array(l), Value::Array(r)) => {
            json_value_eq_array::json_value_eq_array(context, l, r)
        }
        (Value::Object(l), Value::Object(r)) => {
            json_value_eq_object::json_value_eq_object(context, l, r)
        }
        _ => Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            received: ValueTypeObject::from(received.clone()),
            expected: ValueTypeObject::from(expected.clone()),
        }),
    }
}
