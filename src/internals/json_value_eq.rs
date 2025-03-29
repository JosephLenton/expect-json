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
use serde_json::Number;
use serde_json::Value;

pub fn json_value_eq<'a>(
    context: &mut Context<'a>,
    expected: &'a Value,
    received: &'a Value,
) -> JsonValueEqResult<()> {
    match (expected, received) {
        (Value::Null, Value::Null) => Ok(()),
        (Value::Number(l), Value::Number(r)) => json_value_eq_number(context, l, r),
        (Value::String(l), Value::String(r)) => json_value_eq_string(context, l, r),
        (Value::Bool(l), Value::Bool(r)) => json_value_eq_boolean(context, *l, *r),
        (Value::Array(l), Value::Array(r)) => json_value_eq_array(context, l, r),
        (Value::Object(l), Value::Object(r)) => json_value_eq_object(context, l, r),
        (e, o) => Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            expected: ValueType::type_of(e.clone()),
            received: ValueType::type_of(o.clone()),
        }),
    }
}

fn json_value_eq_boolean(
    context: &mut Context,
    expected: bool,
    received: bool,
) -> JsonValueEqResult<()> {
    if expected != received {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            expected: BooleanObject::from(expected).into(),
            received: BooleanObject::from(received).into(),
        });
    }

    Ok(())
}

fn json_value_eq_number(
    context: &mut Context,
    expected_number: &Number,
    received_number: &Number,
) -> JsonValueEqResult<()> {
    if expected_number != received_number {
        let expected = NumberObject::from(expected_number);
        let received = NumberObject::from(received_number);

        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            expected: expected.into(),
            received: received.into(),
        });
    }

    Ok(())
}

fn json_value_eq_string(
    context: &mut Context,
    expected: &str,
    received: &str,
) -> JsonValueEqResult<()> {
    if expected != received {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            expected: StringObject::from(expected.to_string()).into(),
            received: StringObject::from(received.to_string()).into(),
        });
    }

    Ok(())
}

fn json_value_eq_object<'a>(
    context: &mut Context<'a>,
    expected: &'a JsonObject,
    received: &'a JsonObject,
) -> JsonValueEqResult<()> {
    if expected.len() != received.len() {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            expected: ObjectObject::from(expected.clone()).into(),
            received: ObjectObject::from(received.clone()).into(),
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
        json_value_eq(context, expected_value, received_value)?;
        context.pop();
    }

    Ok(())
}

fn json_value_eq_array<'a>(
    context: &mut Context<'a>,
    expected_array: &'a [Value],
    received_array: &'a [Value],
) -> JsonValueEqResult<()> {
    if expected_array.len() != received_array.len() {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            expected: ArrayObject::from(expected_array.to_owned()).into(),
            received: ArrayObject::from(received_array.to_owned()).into(),
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
        json_value_eq(context, expected_value, received_value).map_err(|source_error| {
            JsonValueEqError::array_index_missing(
                context,
                source_error,
                expected_array,
                received_array,
            )
        })?;
        context.pop();
    }

    Ok(())
}
