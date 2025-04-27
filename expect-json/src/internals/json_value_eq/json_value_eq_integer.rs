use crate::internals::objects::IntegerObject;
use crate::Context;
use crate::ExpectJsonError;
use crate::ExpectJsonResult;
use crate::JsonType;

pub fn json_value_eq_integer(
    context: &mut Context,
    received_number: IntegerObject,
    expected_number: IntegerObject,
) -> ExpectJsonResult<()> {
    if received_number != expected_number {
        return Err(ExpectJsonError::DifferentValues {
            context: context.to_static(),
            json_type: JsonType::Integer,
            received: received_number.into(),
            expected: expected_number.into(),
        });
    }

    Ok(())
}
