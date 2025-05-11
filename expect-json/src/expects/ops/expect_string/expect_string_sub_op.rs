use crate::internals::objects::StringObject;
use crate::ops::ExpectString;
use crate::Context;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use crate::JsonType;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectStringSubOp {
    IsEmpty,
    IsNotEmpty,
    MinLen(usize),
    MaxLen(usize),
    Contains(String),
}

impl ExpectStringSubOp {
    pub(crate) fn on_string(
        &self,
        parent: &ExpectString,
        context: &mut Context<'_>,
        received: &str,
    ) -> ExpectOpResult<()> {
        match self {
            Self::IsEmpty => Self::on_string_is_empty(parent, context, received),
            Self::IsNotEmpty => Self::on_string_is_not_empty(parent, context, received),
            Self::MinLen(min_len) => Self::on_string_min_len(*min_len, parent, context, received),
            Self::MaxLen(max_len) => Self::on_string_max_len(*max_len, parent, context, received),
            Self::Contains(contains) => {
                Self::on_string_contains(contains, parent, context, received)
            }
        }
    }

    fn on_string_is_empty(
        parent: &ExpectString,
        context: &mut Context<'_>,
        received: &str,
    ) -> ExpectOpResult<()> {
        if !received.is_empty() {
            let error_message = format!(
                r#"expected empty string
    received {}"#,
                StringObject::from(received)
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_string_is_not_empty(
        parent: &ExpectString,
        context: &mut Context<'_>,
        received: &str,
    ) -> ExpectOpResult<()> {
        if received.is_empty() {
            let error_message = format!(
                r#"expected non-empty string
    received {}"#,
                StringObject::from(received)
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_string_min_len(
        min_len: usize,
        parent: &ExpectString,
        context: &mut Context<'_>,
        received: &str,
    ) -> ExpectOpResult<()> {
        if received.len() < min_len {
            let error_message = format!(
                r#"expected string to have at least {} characters, but it has {},
    received {}"#,
                min_len,
                received.len(),
                StringObject::from(received),
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_string_max_len(
        max_len: usize,
        parent: &ExpectString,
        context: &mut Context<'_>,
        received: &str,
    ) -> ExpectOpResult<()> {
        if received.len() > max_len {
            let error_message = format!(
                r#"expected string to have at most {} characters, but it has {},
    received {}"#,
                max_len,
                received.len(),
                StringObject::from(received),
            );
            return Err(ExpectOpError::custom(context, parent, error_message));
        }

        Ok(())
    }

    fn on_string_contains(
        expected_sub_string: &str,
        _parent: &ExpectString,
        context: &mut Context<'_>,
        received: &str,
    ) -> ExpectOpResult<()> {
        if !received.contains(expected_sub_string) {
            return Err(ExpectOpError::ContainsNotFound {
                context: context.to_static(),
                json_type: JsonType::String,
                expected: StringObject::from(expected_sub_string).into(),
                received: StringObject::from(received).into(),
            });
        }

        Ok(())
    }
}
