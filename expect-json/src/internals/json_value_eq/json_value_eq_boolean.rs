use crate::internals::context::Context;
use crate::internals::objects::BooleanObject;
use crate::ExpectJsonError;
use crate::ExpectJsonResult;
use crate::JsonType;

pub fn json_value_eq_boolean(
    context: &mut Context,
    received: bool,
    expected: bool,
) -> ExpectJsonResult<()> {
    if expected != received {
        return Err(ExpectJsonError::DifferentValues {
            context: context.to_static(),
            json_type: JsonType::Boolean,
            received: BooleanObject::from(received).into(),
            expected: BooleanObject::from(expected).into(),
        });
    }

    Ok(())
}
