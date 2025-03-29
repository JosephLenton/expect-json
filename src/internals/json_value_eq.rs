use super::BooleanType;
use super::Context;
use super::JsonValueEqError;
use super::JsonValueEqResult;
use super::NumberType;
use super::StringType;
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
            expected: ValueType::type_of(e),
            received: ValueType::type_of(o),
        }),
    }
}

fn json_value_eq_boolean(
    context: Context,
    expected: bool,
    received: bool,
) -> JsonValueEqResult<()> {
    if expected != received {
        return Err(JsonValueEqError::DifferentTypes {
            context,
            expected: BooleanType::from(expected).into(),
            received: BooleanType::from(received).into(),
        });
    }

    Ok(())
}

fn json_value_eq_number(
    context: Context,
    expected_number: Number,
    received_number: Number,
) -> JsonValueEqResult<()> {
    if expected_number != received_number {
        let expected = NumberType::from(expected_number);
        let received = NumberType::from(received_number);

        return Err(JsonValueEqError::DifferentTypes {
            context,
            expected: expected.into(),
            received: received.into(),
        });
    }

    Ok(())
}

fn json_value_eq_string(
    context: Context,
    expected: String,
    received: String,
) -> JsonValueEqResult<()> {
    if expected != received {
        return Err(JsonValueEqError::DifferentTypes {
            context,
            expected: StringType::from(expected).into(),
            received: StringType::from(received).into(),
        });
    }

    Ok(())
}
