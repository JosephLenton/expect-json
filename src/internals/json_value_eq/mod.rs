use crate::internals::context::Context;
use crate::internals::objects::ValueObject;
use crate::internals::types::ValueTypeObject;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use serde_json::Value;

mod json_value_eq_array;
mod json_value_eq_boolean;
mod json_value_eq_float;
mod json_value_eq_integer;
mod json_value_eq_object;
mod json_value_eq_string;

pub fn json_value_eq<'a>(
    context: &mut Context<'a>,
    received: &'a Value,
    expected: &'a Value,
) -> JsonValueEqResult<()> {
    match (received, expected) {
        (Value::Null, Value::Null) => Ok(()),
        (_, Value::Null) => Err(JsonValueEqError::ReceivedIsNotNull {
            context: context.to_static(),
            received: received.clone().into(),
        }),
        (Value::Null, _) => Err(JsonValueEqError::ReceivedIsNull {
            context: context.to_static(),
            expected: expected.clone().into(),
        }),
        (Value::Number(l), Value::Number(r)) => {
            let l_value = ValueObject::from(l.clone());
            let r_value = ValueObject::from(r.clone());

            match (l_value, r_value) {
                (ValueObject::Float(l_float), ValueObject::Float(r_float)) => {
                    json_value_eq_float::json_value_eq_float(
                        context,
                        l_float.into(),
                        r_float.into(),
                    )
                }
                (ValueObject::Integer(l_int), ValueObject::Integer(r_int)) => {
                    json_value_eq_integer::json_value_eq_integer(context, l_int, r_int)
                }
                (l_value, r_value) => Err(JsonValueEqError::DifferentTypes {
                    context: context.to_static(),
                    received: ValueTypeObject::from(l_value),
                    expected: ValueTypeObject::from(r_value),
                }),
            }
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
