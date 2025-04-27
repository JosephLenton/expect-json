use crate::internals::objects::StringObject;
use crate::Context;
use crate::ExpectJsonError;
use crate::ExpectJsonResult;
use crate::JsonType;

pub fn json_value_eq_string(
    context: &mut Context,
    received: &str,
    expected: &str,
) -> ExpectJsonResult<()> {
    if received != expected {
        return Err(ExpectJsonError::DifferentValues {
            context: context.to_static(),
            json_type: JsonType::String,
            received: StringObject::from(received.to_string()).into(),
            expected: StringObject::from(expected.to_string()).into(),
        });
    }

    Ok(())
}
