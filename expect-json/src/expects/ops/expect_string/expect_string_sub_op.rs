use crate::internals::objects::StringObject;
use crate::internals::Context;
use crate::ops::ExpectString;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use crate::JsonType;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectStringSubOp {
    IsEmpty,
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
            ExpectStringSubOp::IsEmpty => {
                ExpectStringSubOp::on_string_is_empty(parent, context, received)
            }
            ExpectStringSubOp::MinLen(min_len) => {
                ExpectStringSubOp::on_string_min_len(*min_len, parent, context, received)
            }
            ExpectStringSubOp::MaxLen(max_len) => {
                ExpectStringSubOp::on_string_max_len(*max_len, parent, context, received)
            }
            ExpectStringSubOp::Contains(contains) => {
                ExpectStringSubOp::on_string_contains(contains, parent, context, received)
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

    fn on_string_min_len(
        min_len: usize,
        parent: &ExpectString,
        context: &mut Context<'_>,
        received: &str,
    ) -> ExpectOpResult<()> {
        if received.len() < min_len {
            let error_message = format!(
                "String is too short, expected length at least {}, received length {}",
                min_len,
                received.len()
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
                "String is too long, expected length at most {}, received length {}",
                max_len,
                received.len()
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
