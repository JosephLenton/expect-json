use crate::internals::context::Context;
use crate::internals::objects::NumberObject;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use serde_json::Number;

pub fn json_value_eq_number(
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
