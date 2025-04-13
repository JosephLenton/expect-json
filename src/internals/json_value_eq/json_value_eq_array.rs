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
    // The Expected array is longer,
    //
    // Add some special cases to give better error messages.
    if expected_array.len() > received_array.len() {
        if let Some(missing_in_received) = has_more_at_end(received_array, expected_array) {
            return Err(JsonValueEqError::ArrayMissingAtEnd {
                context: context.to_static(),
                received_array: ArrayObject::from(received_array.to_owned()),
                expected_array: ArrayObject::from(expected_array.to_owned()),
                missing_in_received: ArrayObject::from(missing_in_received),
            });
        }

        if let Some(missing_in_received) = has_more_at_start(received_array, expected_array) {
            return Err(JsonValueEqError::ArrayMissingAtStart {
                context: context.to_static(),
                received_array: ArrayObject::from(received_array.to_owned()),
                expected_array: ArrayObject::from(expected_array.to_owned()),
                missing_in_received: ArrayObject::from(missing_in_received),
            });
        }

        return Err(JsonValueEqError::ArrayValuesAreDifferent {
            context: context.to_static(),
            received_array: ArrayObject::from(received_array.to_owned()),
            expected_array: ArrayObject::from(expected_array.to_owned()),
        });
    }

    // The Expected array is longer,
    //
    // Add some special cases to give better error messages.
    if expected_array.len() < received_array.len() {
        if let Some(extra_in_received) = has_more_at_end(expected_array, received_array) {
            return Err(JsonValueEqError::ArrayExtraAtEnd {
                context: context.to_static(),
                received_array: ArrayObject::from(received_array.to_owned()),
                expected_array: ArrayObject::from(expected_array.to_owned()),
                extra_in_received: ArrayObject::from(extra_in_received),
            });
        }

        if let Some(extra_in_received) = has_more_at_start(expected_array, received_array) {
            return Err(JsonValueEqError::ArrayExtraAtStart {
                context: context.to_static(),
                received_array: ArrayObject::from(received_array.to_owned()),
                expected_array: ArrayObject::from(expected_array.to_owned()),
                extra_in_received: ArrayObject::from(extra_in_received),
            });
        }

        return Err(JsonValueEqError::ArrayValuesAreDifferent {
            context: context.to_static(),
            received_array: ArrayObject::from(received_array.to_owned()),
            expected_array: ArrayObject::from(expected_array.to_owned()),
        });
    }

    for (index, (expected_value, received_value)) in
        expected_array.iter().zip(received_array).enumerate()
    {
        context.push(index);
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

fn has_more_at_end<'a>(
    left: &'a [Value],
    right: &'a [Value],
) -> Option<impl Iterator<Item = Value> + 'a> {
    let mut zipped_arrays_at_start = left.iter().zip(right);
    let is_all_equal_at_start = zipped_arrays_at_start.all(|(left, right)| left == right);

    if is_all_equal_at_start {
        let missing_in_left = right[left.len()..].iter().cloned();
        return Some(missing_in_left);
    }

    None
}

fn has_more_at_start<'a>(
    left: &'a [Value],
    right: &'a [Value],
) -> Option<impl Iterator<Item = Value> + 'a> {
    let mut zipped_arrays_at_end = left.iter().rev().zip(right.iter().rev());
    let is_all_equal_at_end = zipped_arrays_at_end.all(|(left, right)| left == right);
    if is_all_equal_at_end {
        let len_diff = right.len() - left.len();
        let missing_in_left = right[0..len_diff].iter().cloned();
        return Some(missing_in_left);
    }

    None
}
