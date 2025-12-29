use expect_json::expect_core::ExpectOp;
use expect_json::expect_core::ExpectOpError;
use expect_json::expect_core::expect_op;
use expect_json::*;
use pretty_assertions::assert_eq;
use serde_json::json;

#[derive(Copy, Clone, Debug, PartialEq)]
#[expect_op]
struct StringIsAllA;

impl ExpectOp for StringIsAllA {
    fn on_string(
        &self,
        context: &mut expect_core::Context<'_>,
        received: &str,
    ) -> expect_core::ExpectOpResult<()> {
        let is_all_as = received.chars().all(|c| c == 'a' || c == 'A');
        if !is_all_as {
            let msg = format!("Expected string to be all a characters, received '{received}'");
            return Err(ExpectOpError::custom(self, context, msg));
        }

        Ok(())
    }

    fn debug_supported_types(&self) -> &'static [JsonType] {
        &[JsonType::String]
    }
}

#[test]
fn it_should_support_custom_expect_ops_that_pass() {
    let output = expect_json_eq(&json!("aaa"), &json!(StringIsAllA));
    assert!(output.is_ok(), "assertion error: {output:#?}");
}

#[test]
fn it_should_fail_with_error_message_given() {
    let output = expect_json_eq(&json!("🦊🦊🦊"), &json!(StringIsAllA))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        r#"Json expect::StringIsAllA() error at root:
    Expected string to be all a characters, received '🦊🦊🦊'"#
    );
}

#[test]
fn it_should_support_an_expect_op_within_an_expect_op() {
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[expect_op]
    struct StringIsAllAParent {
        inner: StringIsAllA,
    }

    impl ExpectOp for StringIsAllAParent {
        fn on_string(
            &self,
            context: &mut expect_core::Context<'_>,
            received: &str,
        ) -> expect_core::ExpectOpResult<()> {
            self.inner.on_string(context, received)
        }
    }

    let parent = StringIsAllAParent {
        inner: StringIsAllA,
    };
    let output = expect_json_eq(&json!("aaa"), &json!(parent));
    assert!(output.is_ok(), "assertion error: {output:#?}");
}
