use crate::internals::objects::IntegerObject;
use crate::internals::objects::ValueObject;
use crate::internals::Context;
use crate::internals::JsonValueEqResult;
use crate::JsonType;
use serde_json::Map;
use serde_json::Value;
use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait ExpectOp: Debug + Send + 'static {
    fn on_any(&self, context: &mut Context<'_>, received: &Value) -> JsonValueEqResult<()> {
        match received {
            Value::Null => self.on_null(context),
            Value::Number(received_number) => {
                let value_num = ValueObject::from(received_number.clone());
                match value_num {
                    ValueObject::Float(received_float) => self.on_f64(context, received_float.into()),
                    ValueObject::Integer(IntegerObject::Positive(received_integer)) => self.on_u64(context, received_integer),
                    ValueObject::Integer(IntegerObject::Negative(received_integer)) => self.on_i64(context, received_integer),
                    _ => panic!("Unexpected non-number value, expected a float or an integer, found {value_num:?}. (This is a bug, please report at: https://github.com/JosephLenton/expect-json/issues)"),
                }
            }
            Value::String(received_string) => self.on_string(context, received_string),
            Value::Bool(received_boolean) => self.on_boolean(context, *received_boolean),
            Value::Array(received_array) => self.on_array(context, received_array),
            Value::Object(received_object) => self.on_object(context, received_object),
        }
    }

    #[allow(unused_variables)]
    fn on_null(&self, context: &mut Context<'_>) -> JsonValueEqResult<()> {
        Err(context.unsupported_expect_op_type(JsonType::Null, self))
    }

    #[allow(unused_variables)]
    fn on_f64(&self, context: &mut Context<'_>, received: f64) -> JsonValueEqResult<()> {
        Err(context.unsupported_expect_op_type(JsonType::Float, self))
    }

    #[allow(unused_variables)]
    fn on_u64(&self, context: &mut Context<'_>, received: u64) -> JsonValueEqResult<()> {
        Err(context.unsupported_expect_op_type(JsonType::Integer, self))
    }

    #[allow(unused_variables)]
    fn on_i64(&self, context: &mut Context<'_>, received: i64) -> JsonValueEqResult<()> {
        Err(context.unsupported_expect_op_type(JsonType::Integer, self))
    }

    #[allow(unused_variables)]
    fn on_boolean(&self, context: &mut Context<'_>, received: bool) -> JsonValueEqResult<()> {
        Err(context.unsupported_expect_op_type(JsonType::Boolean, self))
    }

    #[allow(unused_variables)]
    fn on_string(&self, context: &mut Context<'_>, received: &str) -> JsonValueEqResult<()> {
        Err(context.unsupported_expect_op_type(JsonType::String, self))
    }

    #[allow(unused_variables)]
    fn on_array(&self, context: &mut Context<'_>, received: &[Value]) -> JsonValueEqResult<()> {
        Err(context.unsupported_expect_op_type(JsonType::Array, self))
    }

    #[allow(unused_variables)]
    fn on_object(
        &self,
        context: &mut Context<'_>,
        received: &Map<String, Value>,
    ) -> JsonValueEqResult<()> {
        Err(context.unsupported_expect_op_type(JsonType::Object, self))
    }

    fn name(&self) -> &'static str {
        "<Unknown ExpectOp>"
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[]
    }
}

#[cfg(test)]
mod test_on_any {
    use super::*;
    use crate::internals::ExpectOpMeta;
    use crate::internals::JsonValueEqError;
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json::json;

    // An empty implementation which will hit the errors by default.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestJsonExpectOp;

    #[typetag::serde]
    impl ExpectOp for TestJsonExpectOp {
        fn name(&self) -> &'static str {
            "TestJsonExpectOp"
        }
    }

    #[test]
    fn it_should_error_by_default_against_json_null() {
        let mut context = Context::new();
        let received = json!(null);
        let output = TestJsonExpectOp
            .on_any(&mut context, &received)
            .unwrap_err();
        assert_eq!(
            output,
            JsonValueEqError::UnsupportedOperation {
                context: context.to_static(),
                received_type: JsonType::Null,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            }
        );
    }

    #[test]
    fn it_should_error_by_default_against_json_boolean() {
        let mut context = Context::new();
        let received = json!(true);
        let output = TestJsonExpectOp
            .on_any(&mut context, &received)
            .unwrap_err();
        assert_eq!(
            output,
            JsonValueEqError::UnsupportedOperation {
                context: context.to_static(),
                received_type: JsonType::Boolean,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            }
        );
    }

    #[test]
    fn it_should_error_by_default_against_json_positive_integer() {
        let mut context = Context::new();
        let received = json!(123);
        let output = TestJsonExpectOp
            .on_any(&mut context, &received)
            .unwrap_err();
        assert_eq!(
            output,
            JsonValueEqError::UnsupportedOperation {
                context: context.to_static(),
                received_type: JsonType::Integer,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            }
        );
    }

    #[test]
    fn it_should_error_by_default_against_json_negative_integer() {
        let mut context = Context::new();
        let received = json!(-123);
        let output = TestJsonExpectOp
            .on_any(&mut context, &received)
            .unwrap_err();
        assert_eq!(
            output,
            JsonValueEqError::UnsupportedOperation {
                context: context.to_static(),
                received_type: JsonType::Integer,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            }
        );
    }

    #[test]
    fn it_should_error_by_default_against_json_float() {
        let mut context = Context::new();
        let received = json!(123.456);
        let output = TestJsonExpectOp
            .on_any(&mut context, &received)
            .unwrap_err();
        assert_eq!(
            output,
            JsonValueEqError::UnsupportedOperation {
                context: context.to_static(),
                received_type: JsonType::Float,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            }
        );
    }

    #[test]
    fn it_should_error_by_default_against_json_string() {
        let mut context = Context::new();
        let received = json!("🦊");
        let output = TestJsonExpectOp
            .on_any(&mut context, &received)
            .unwrap_err();
        assert_eq!(
            output,
            JsonValueEqError::UnsupportedOperation {
                context: context.to_static(),
                received_type: JsonType::String,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            }
        );
    }

    #[test]
    fn it_should_error_by_default_against_json_array() {
        let mut context = Context::new();
        let received = json!([]);
        let output = TestJsonExpectOp
            .on_any(&mut context, &received)
            .unwrap_err();
        assert_eq!(
            output,
            JsonValueEqError::UnsupportedOperation {
                context: context.to_static(),
                received_type: JsonType::Array,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            }
        );
    }

    #[test]
    fn it_should_error_by_default_against_json_object() {
        let mut context = Context::new();
        let received = json!({});
        let output = TestJsonExpectOp
            .on_any(&mut context, &received)
            .unwrap_err();
        assert_eq!(
            output,
            JsonValueEqError::UnsupportedOperation {
                context: context.to_static(),
                received_type: JsonType::Object,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            }
        );
    }
}
