use crate::expect_op;
use crate::internals::objects::ArrayObject;
use crate::internals::objects::ObjectObject;
use crate::internals::objects::StringObject;
use crate::internals::Context;
use crate::ExpectOp;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use crate::JsonType;
use serde_json::Map;
use serde_json::Value;

#[expect_op(internal)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IsEmpty;

impl ExpectOp for IsEmpty {
    fn on_string(&self, context: &mut Context, received: &str) -> ExpectOpResult<()> {
        if !received.is_empty() {
            let error_message = format!(
                r#"expected empty string
    received {}"#,
                StringObject::from(received)
            );
            return Err(ExpectOpError::custom(context, self, error_message));
        }

        Ok(())
    }

    fn on_array(&self, context: &mut Context, received: &[Value]) -> ExpectOpResult<()> {
        if !received.is_empty() {
            let error_message = format!(
                r#"expected empty array
    received {}"#,
                ArrayObject::from(received.iter().cloned())
            );
            return Err(ExpectOpError::custom(context, self, error_message));
        }

        Ok(())
    }

    fn on_object(
        &self,
        context: &mut Context,
        received: &Map<String, Value>,
    ) -> ExpectOpResult<()> {
        if !received.is_empty() {
            let error_message = format!(
                r#"expected empty object
    received {}"#,
                ObjectObject::from(received.clone())
            );
            return Err(ExpectOpError::custom(context, self, error_message));
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::String, JsonType::Array, JsonType::Object]
    }
}

#[cfg(test)]
mod test_is_empty {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_string_is_empty() {
        let left = json!("");
        let right = json!(expect.is_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_string_is_not_empty() {
        let left = json!("🦊");
        let right = json!(expect.is_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect.IsEmpty() error at root:
    expected empty string
    received "🦊""#
            )
        );
    }

    #[test]
    fn it_should_pass_when_array_is_empty() {
        let left = json!([]);
        let right = json!(expect.is_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_array_is_not_empty() {
        let left = json!([1, 2, 3]);
        let right = json!(expect.is_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect.IsEmpty() error at root:
    expected empty array
    received [1, 2, 3]"#
            )
        );
    }

    #[test]
    fn it_should_pass_when_object_is_empty() {
        let left = json!({});
        let right = json!(expect.is_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_object_is_not_empty() {
        let left = json!({ "foo": "bar" });
        let right = json!(expect.is_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect.IsEmpty() error at root:
    expected empty object
    received {{
        "foo": "bar"
    }}"#
            )
        );
    }
}
