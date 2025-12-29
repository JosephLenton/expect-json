use crate::ExpectJsonError;
use crate::ExpectJsonResult;
use crate::JsonType;
use crate::expect_core::Context;
use crate::internals::JsonObject;
use crate::internals::objects::ObjectObject;

pub fn json_value_eq_object<'a>(
    context: &mut Context<'a>,
    received: &'a JsonObject,
    expected: &'a JsonObject,
) -> ExpectJsonResult<()> {
    if context.is_propagated_contains() {
        return json_value_eq_object_contains(context, received, expected);
    }

    let received_len = received.len();
    let expected_len = expected.len();

    // We have a special error case when there is only one extra field,
    // for prettier error output.
    if received_len == expected_len + 1 {
        let maybe_extra_field = received.keys().find(|key| !expected.contains_key(*key));

        if let Some(extra_field) = maybe_extra_field {
            return Err(ExpectJsonError::ObjectReceivedHasExtraKey {
                context: context.to_static(),
                received_extra_field: extra_field.to_string(),
                received_obj: ObjectObject::from(received.clone()).into(),
                expected_obj: ObjectObject::from(expected.clone()).into(),
            });
        }
    }

    // For when received many extra field over what is expected.
    if received_len > expected_len {
        let extra_fields = received
            .keys()
            .filter(|key| !expected.contains_key(*key))
            .cloned()
            .collect();

        return Err(ExpectJsonError::ObjectReceivedHasExtraKeys {
            context: context.to_static(),
            received_extra_fields: extra_fields,
            received_obj: ObjectObject::from(received.clone()).into(),
            expected_obj: ObjectObject::from(expected.clone()).into(),
        });
    }

    if received.len() != expected.len() {
        return Err(ExpectJsonError::DifferentValues {
            context: context.to_static(),
            json_type: JsonType::Object,
            received: ObjectObject::from(received.clone()).into(),
            expected: ObjectObject::from(expected.clone()).into(),
        });
    }

    for (expected_key, expected_value) in expected.iter() {
        let received_value =
            received
                .get(expected_key)
                .ok_or_else(|| ExpectJsonError::ObjectKeyMissing {
                    context: context.to_static(),
                    expected_key: expected_key.to_string(),
                })?;

        context
            .with_path(expected_key)
            .json_eq(received_value, expected_value)?;
    }

    Ok(())
}

pub fn json_value_eq_object_contains<'a>(
    context: &mut Context<'a>,
    received: &'a JsonObject,
    expected: &'a JsonObject,
) -> ExpectJsonResult<()> {
    for (expected_key, expected_value) in expected {
        let received_value =
            received
                .get(expected_key)
                .ok_or_else(|| ExpectJsonError::ObjectKeyMissing {
                    context: context.to_static(),
                    expected_key: expected_key.to_string(),
                })?;

        context
            .with_path(expected_key)
            .json_eq(received_value, expected_value)?;
    }

    Ok(())
}
