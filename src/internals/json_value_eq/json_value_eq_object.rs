use crate::internals::context::Context;
use crate::internals::json_eq;
use crate::internals::objects::ObjectObject;
use crate::internals::JsonObject;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;

pub fn json_value_eq_object<'a>(
    context: &mut Context<'a>,
    received: &'a JsonObject,
    expected: &'a JsonObject,
) -> JsonValueEqResult<()> {
    let received_len = received.len();
    let expected_len = expected.len();

    if received_len > expected_len {
        let maybe_extra_field = received.keys().find(|key| !expected.contains_key(*key));

        if let Some(extra_field) = maybe_extra_field {
            return Err(JsonValueEqError::ObjectReceivedHasExtraKey {
                context: context.to_static(),
                received_extra_field: extra_field.to_string(),
                received_obj: ObjectObject::from(received.clone()).into(),
                expected_obj: ObjectObject::from(expected.clone()).into(),
            });
        }
    }

    if received.len() != expected.len() {
        return Err(JsonValueEqError::DifferentTypes {
            context: context.to_static(),
            received: ObjectObject::from(received.clone()).into(),
            expected: ObjectObject::from(expected.clone()).into(),
        });
    }

    for (expected_key, expected_value) in expected.iter() {
        let received_value =
            received
                .get(expected_key)
                .ok_or_else(|| JsonValueEqError::ObjectKeyMissing {
                    context: context.to_static(),
                    expected_key: expected_key.to_string(),
                })?;

        context.push(expected_key);
        json_eq(context, received_value, expected_value)?;
        context.pop();
    }

    Ok(())
}
