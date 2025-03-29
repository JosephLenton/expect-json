use super::Context;
use super::JsonValueEqError;
use super::JsonValueEqResult;
use super::ValueType;
use serde_json::Number;
use serde_json::Value;

pub fn json_value_eq(context: Context, expected: Value, other: Value) -> JsonValueEqResult<()> {
    match (expected, other) {
        (Value::Null, Value::Null) => Ok(()),
        (Value::Number(l), Value::Number(r)) => json_value_eq_number(context, l, r),
        (Value::String(l), Value::String(r)) => json_value_eq_string(context, l, r),
        (Value::Bool(l), Value::Bool(r)) => json_value_eq_boolean(context, l, r),
        (Value::Array(l), Value::Array(r)) => {
            unimplemented!()
        }
        (Value::Object(l), Value::Object(r)) => {
            unimplemented!()
        }
        (e, o) => Err(JsonValueEqError::DifferentTypes {
            context,
            expected_type: ValueType::type_of(&e),
            received_type: ValueType::type_of(&o),
        }),
    }
}

fn json_value_eq_boolean(
    context: Context,
    expected: bool,
    received: bool,
) -> JsonValueEqResult<()> {
    if expected != received {
        return Err(JsonValueEqError::DifferentBooleanValues {
            context,
            expected,
            received,
        });
    }

    Ok(())
}

fn json_value_eq_number(
    context: Context,
    expected: Number,
    received: Number,
) -> JsonValueEqResult<()> {
    // if expected != received {
    //     return Err(JsonValueEqError::DifferentBooleanValues { context, expected, received });
    // }

    unimplemented!("todo, eq number");
    Ok(())
}

fn json_value_eq_string(
    context: Context,
    expected: String,
    received: String,
) -> JsonValueEqResult<()> {
    if expected != received {
        return Err(JsonValueEqError::DifferentStringValues {
            context,
            expected,
            received,
        });
    }

    Ok(())
}
