use super::json_apply_expect_op;
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
use serde_json::Number;
use serde_json::Value;

pub fn json_value_eq<'a>(
    context: &mut Context<'a>,
    received: &'a Value,
    expected: &'a Value,
) -> JsonValueEqResult<()> {
    if let Some(expect_op) = SerializeExpect::maybe_parse(expected) {
        return json_apply_expect_op(context, received, expect_op);
    }

    match (received, expected) {
        (Value::Null, Value::Null) => Ok(()),
        (Value::Number(l), Value::Number(r)) => json_value_eq_number(context, l, r),
        (Value::String(l), Value::String(r)) => json_value_eq_string(context, l, r),
        (Value::Bool(l), Value::Bool(r)) => json_value_eq_boolean(context, *l, *r),
        (Value::Array(l), Value::Array(r)) => json_value_eq_array(context, l, r),
        (Value::Object(l), Value::Object(r)) => json_value_eq_object(context, l, r),
        (e, o) => Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            received: ValueType::type_of(o.clone()),
            expected: ValueType::type_of(e.clone()),
        }),
    }
}

fn json_value_eq_boolean(
    context: &mut Context,
    received: bool,
    expected: bool,
) -> JsonValueEqResult<()> {
    if expected != received {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            received: BooleanObject::from(received).into(),
            expected: BooleanObject::from(expected).into(),
        });
    }

    Ok(())
}

fn json_value_eq_number(
    context: &mut Context,
    received_number: &Number,
    expected_number: &Number,
) -> JsonValueEqResult<()> {
    if received_number != expected_number {
        let received = NumberObject::from(received_number);
        let expected = NumberObject::from(expected_number);

        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            received: received.into(),
            expected: expected.into(),
        });
    }

    Ok(())
}

fn json_value_eq_string(
    context: &mut Context,
    received: &str,
    expected: &str,
) -> JsonValueEqResult<()> {
    if received != expected {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            received: StringObject::from(received.to_string()).into(),
            expected: StringObject::from(expected.to_string()).into(),
        });
    }

    Ok(())
}

fn json_value_eq_object<'a>(
    context: &mut Context<'a>,
    received: &'a JsonObject,
    expected: &'a JsonObject,
) -> JsonValueEqResult<()> {
    if received.len() != expected.len() {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            received: ObjectObject::from(received.clone()).into(),
            expected: ObjectObject::from(expected.clone()).into(),
        });
    }

    for (expected_key, expected_value) in expected.iter() {
        let received_value =
            received
                .get(expected_key)
                .ok_or_else(|| JsonValueEqError::ObjectKeyMissing {
                    context: context.to_static(),
                    expected_key: expected_key.to_string(),
                })?;

        context.push(expected_key);
        json_value_eq(context, received_value, expected_value)?;
        context.pop();
    }

    Ok(())
}

fn json_value_eq_array<'a>(
    context: &mut Context<'a>,
    received_array: &'a [Value],
    expected_array: &'a [Value],
) -> JsonValueEqResult<()> {
    if expected_array.len() != received_array.len() {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            received: ArrayObject::from(received_array.to_owned()).into(),
            expected: ArrayObject::from(expected_array.to_owned()).into(),
        });
    }

    for (expected_index, expected_value) in expected_array.iter().enumerate() {
        let received_value = received_array.get(expected_index).ok_or_else(|| {
            JsonValueEqError::ArrayIndexMissing {
                context: context.to_static(),
                expected_index,
            }
        })?;

        context.push(expected_index);
        json_value_eq(context, received_value, expected_value).map_err(|source_error| {
            JsonValueEqError::array_index_missing(
                context,
                source_error,
                received_array,
                expected_array,
            )
        })?;
        context.pop();
    }

    Ok(())
}
