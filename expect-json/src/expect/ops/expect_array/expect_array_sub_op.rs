use crate::expect::ops::ExpectArray;
use crate::expect_core::Context;
use crate::expect_core::ExpectOpError;
use crate::expect_core::ExpectOpResult;
use crate::internals::objects::ArrayObject;
use crate::JsonType;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectArraySubOp {
    IsEmpty,
    IsNotEmpty,
    MinLen(usize),
    Len(usize),
    MaxLen(usize),
    Contains(Vec<Value>),
    AllUnique,
    AllEqual(Value),
}

impl ExpectArraySubOp {
    pub(crate) fn on_array(
        &self,
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received: &[Value],
    ) -> ExpectOpResult<()> {
        match self {
            Self::IsEmpty => Self::on_array_is_empty(parent, context, received),
            Self::IsNotEmpty => Self::on_array_is_not_empty(parent, context, received),
            Self::MinLen(min_len) => Self::on_array_min_len(*min_len, parent, context, received),
            Self::Len(len) => Self::on_array_len(*len, parent, context, received),
            Self::MaxLen(max_len) => Self::on_array_max_len(*max_len, parent, context, received),
            Self::Contains(expected_values) => {
                Self::on_array_contains(expected_values, parent, context, received)
            }
            Self::AllUnique => Self::on_array_unique(parent, context, received),
            Self::AllEqual(expected_value) => {
                Self::on_array_all_equal(expected_value, parent, context, received)
            }
        }
    }

    fn on_array_is_empty(
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received_values: &[Value],
    ) -> ExpectOpResult<()> {
        if !received_values.is_empty() {
            let error_message = format!(
                r#"expected empty array
    received {}"#,
                ArrayObject::from(received_values.iter().cloned())
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_array_is_not_empty(
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received_values: &[Value],
    ) -> ExpectOpResult<()> {
        if received_values.is_empty() {
            let error_message = format!(
                r#"expected non-empty array
    received {}"#,
                ArrayObject::from(received_values.iter().cloned())
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_array_min_len(
        min_len: usize,
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received: &[Value],
    ) -> ExpectOpResult<()> {
        if received.len() < min_len {
            let error_message = format!(
                r#"expected array to have at least {} elements, but it has {}.
    received {}"#,
                min_len,
                received.len(),
                ArrayObject::from(received.to_owned())
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_array_len(
        len: usize,
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received: &[Value],
    ) -> ExpectOpResult<()> {
        if received.len() != len {
            let error_message = format!(
                r#"expected array to have {} elements, but it has {}.
    received {}"#,
                len,
                received.len(),
                ArrayObject::from(received.to_owned())
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_array_max_len(
        max_len: usize,
        parent: &ExpectArray,
        context: &mut Context<'_>,
        received: &[Value],
    ) -> ExpectOpResult<()> {
        if received.len() > max_len {
            let error_message = format!(
                r#"expected array to have at most {} elements, but it has {}.
    received {}"#,
                max_len,
                received.len(),
                ArrayObject::from(received.to_owned())
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_array_contains(
        expected_values: &[Value],
        _parent: &ExpectArray,
        context: &mut Context<'_>,
        received_values: &[Value],
    ) -> ExpectOpResult<()> {
        // TODO: This is brute force as we don't know if we are containing an inner ExpectOp.
        // Can this be done without a brute force approach?
        for expected in expected_values {
            let is_found = received_values
                .iter()
                .any(|received| context.json_eq(received, expected).is_ok());

            if !is_found {
                return Err(ExpectOpError::ContainsNotFound {
                    context: context.to_static(),
                    json_type: JsonType::Array,
                    expected: expected.clone().into(),
                    received: ArrayObject::from(received_values.to_owned()).into(),
                });
            }
        }

        Ok(())
    }

    fn on_array_unique(
        _parent: &ExpectArray,
        context: &mut Context<'_>,
        received_values: &[Value],
    ) -> ExpectOpResult<()> {
        let mut seen = HashSet::<&Value>::new();

        for (index, value) in received_values.iter().enumerate() {
            let is_duplicate = !seen.insert(value);
            if is_duplicate {
                context.push(index);
                return Err(ExpectOpError::ArrayContainsDuplicate {
                    context: context.to_static(),
                    duplicate: value.clone().into(),
                    received_array: ArrayObject::from(received_values.to_owned()),
                });
            }
        }

        Ok(())
    }

    fn on_array_all_equal(
        expected_value: &Value,
        _parent: &ExpectArray,
        context: &mut Context<'_>,
        received_values: &[Value],
    ) -> ExpectOpResult<()> {
        for (index, value) in received_values.iter().enumerate() {
            context.push(index);

            context.json_eq(value, expected_value).map_err(|error| {
                ExpectOpError::ArrayAllEqual {
                    error: Box::new(error),
                    received_full_array: ArrayObject::from(received_values.to_owned()),
                }
            })?;

            context.pop();
        }

        Ok(())
    }
}
