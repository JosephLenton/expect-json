use crate::internals::context::Context;
use crate::internals::json_eq;
use crate::internals::objects::ArrayObject;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use serde_json::Value;

pub fn json_value_eq_array<'a>(
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
        json_eq(context, received_value, expected_value).map_err(|source_error| {
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
