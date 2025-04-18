use crate::internals::context::Context;
use crate::internals::objects::IntegerObject;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use crate::JsonType;

pub fn json_value_eq_integer(
    context: &mut Context,
    received_number: IntegerObject,
    expected_number: IntegerObject,
) -> JsonValueEqResult<()> {
    if received_number != expected_number {
        return Err(JsonValueEqError::DifferentValues {
            context: context.to_static(),
            json_type: JsonType::Integer,
            received: received_number.into(),
            expected: expected_number.into(),
        });
    }

    Ok(())
}
