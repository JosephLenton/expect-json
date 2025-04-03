use crate::internals::context::Context;
use crate::internals::objects::FloatObject;
use crate::internals::types::ValueType;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;

pub fn json_value_eq_float(
    context: &mut Context,
    received_number: f64,
    expected_number: f64,
) -> JsonValueEqResult<()> {
    if received_number != expected_number {
        let received = FloatObject::from(received_number);
        let expected = FloatObject::from(expected_number);

        return Err(JsonValueEqError::DifferentValues {
            context: context.to_static(),
            json_type: ValueType::Float,
            received: received.into(),
            expected: expected.into(),
        });
    }

    Ok(())
}
