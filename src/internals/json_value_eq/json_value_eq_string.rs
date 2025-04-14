use crate::internals::context::Context;
use crate::internals::objects::StringObject;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use crate::JsonType;

pub fn json_value_eq_string(
    context: &mut Context,
    received: &str,
    expected: &str,
) -> JsonValueEqResult<()> {
    if received != expected {
        return Err(JsonValueEqError::DifferentValues {
            context: context.to_static(),
            json_type: JsonType::String,
            received: StringObject::from(received.to_string()).into(),
            expected: StringObject::from(expected.to_string()).into(),
        });
    }

    Ok(())
}
