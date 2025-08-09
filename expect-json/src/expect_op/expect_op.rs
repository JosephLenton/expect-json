use crate::__private::ExpectOpExt;
use crate::expect_op::Context;
use crate::expect_op::ExpectOpError;
use crate::expect_op::ExpectOpResult;
use crate::internals::objects::IntegerObject;
use crate::internals::objects::NullObject;
use crate::internals::objects::ValueObject;
use crate::JsonType;
use serde_json::Map;
use serde_json::Value;
use std::fmt::Debug;

pub trait ExpectOp: ExpectOpExt + Debug + Send + 'static {
    fn on_any(&self, context: &mut Context<'_>, received: &Value) -> ExpectOpResult<()> {
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
    fn on_null(&self, context: &mut Context<'_>) -> ExpectOpResult<()> {
        Err(ExpectOpError::unsupported_operation_type(
            context, self, NullObject,
        ))
    }

    #[allow(unused_variables)]
    fn on_f64(&self, context: &mut Context<'_>, received: f64) -> ExpectOpResult<()> {
        Err(ExpectOpError::unsupported_operation_type(
            context, self, received,
        ))
    }

    #[allow(unused_variables)]
    fn on_u64(&self, context: &mut Context<'_>, received: u64) -> ExpectOpResult<()> {
        Err(ExpectOpError::unsupported_operation_type(
            context, self, received,
        ))
    }

    #[allow(unused_variables)]
    fn on_i64(&self, context: &mut Context<'_>, received: i64) -> ExpectOpResult<()> {
        Err(ExpectOpError::unsupported_operation_type(
            context, self, received,
        ))
    }

    #[allow(unused_variables)]
    fn on_boolean(&self, context: &mut Context<'_>, received: bool) -> ExpectOpResult<()> {
        Err(ExpectOpError::unsupported_operation_type(
            context, self, received,
        ))
    }

    #[allow(unused_variables)]
    fn on_string(&self, context: &mut Context<'_>, received: &str) -> ExpectOpResult<()> {
        Err(ExpectOpError::unsupported_operation_type(
            context,
            self,
            received.to_owned(),
        ))
    }

    #[allow(unused_variables)]
    fn on_array(&self, context: &mut Context<'_>, received: &[Value]) -> ExpectOpResult<()> {
        Err(ExpectOpError::unsupported_operation_type(
            context,
            self,
            received.to_owned(),
        ))
    }

    #[allow(unused_variables)]
    fn on_object(
        &self,
        context: &mut Context<'_>,
        received: &Map<String, Value>,
    ) -> ExpectOpResult<()> {
        Err(ExpectOpError::unsupported_operation_type(
            context,
            self,
            received.to_owned(),
        ))
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[]
    }
}

#[cfg(test)]
mod test_on_any {
    use super::*;
    use crate::internals::objects::ArrayObject;
    use crate::internals::objects::BooleanObject;
    use crate::internals::objects::FloatObject;
    use crate::internals::objects::ObjectObject;
    use crate::internals::objects::StringObject;
    use crate::internals::objects::ValueTypeObject;
    use crate::internals::ExpectOpMeta;
    use serde_json::json;

    // An empty implementation which will hit the errors by default.
    #[crate::expect_op(internal)]
    #[derive(Debug, Clone)]
    struct TestJsonExpectOp;

    impl ExpectOp for TestJsonExpectOp {}

    #[test]
    fn it_should_error_by_default_against_json_null() {
        let mut outer_context = Context::new();
        let received = json!(null);
        let output = TestJsonExpectOp
            .on_any(&mut outer_context, &received)
            .unwrap_err();
        assert!(matches!(
            output,
            ExpectOpError::UnsupportedOperation {
                context,
                received: ValueTypeObject(ValueObject::Null(NullObject)),
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            } if context == outer_context.to_static()
        ));
    }

    #[test]
    fn it_should_error_by_default_against_json_boolean() {
        let mut outer_context = Context::new();
        let received = json!(true);
        let output = TestJsonExpectOp
            .on_any(&mut outer_context, &received)
            .unwrap_err();
        assert!(matches!(
            output,
            ExpectOpError::UnsupportedOperation {
                context,
                received: ValueTypeObject(ValueObject::Boolean(BooleanObject(true))),
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            } if context == outer_context.to_static()
        ));
    }

    #[test]
    fn it_should_error_by_default_against_json_positive_integer() {
        let mut outer_context = Context::new();
        let received = json!(123);
        let output = TestJsonExpectOp
            .on_any(&mut outer_context, &received)
            .unwrap_err();
        assert!(matches!(
            output,
            ExpectOpError::UnsupportedOperation {
                context,
                received: ValueTypeObject(ValueObject::Integer(IntegerObject::Positive(123))),
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            } if context == outer_context.to_static()
        ));
    }

    #[test]
    fn it_should_error_by_default_against_json_negative_integer() {
        let mut outer_context = Context::new();
        let received = json!(-123);
        let output = TestJsonExpectOp
            .on_any(&mut outer_context, &received)
            .unwrap_err();
        assert!(matches!(
            output,
            ExpectOpError::UnsupportedOperation {
                context,
                received: ValueTypeObject(ValueObject::Integer(IntegerObject::Negative(-123))),
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            } if context == outer_context.to_static()
        ));
    }

    #[test]
    fn it_should_error_by_default_against_json_float() {
        let mut outer_context = Context::new();
        let received = json!(123.456);
        let output = TestJsonExpectOp
            .on_any(&mut outer_context, &received)
            .unwrap_err();
        assert!(matches!(
            output,
            ExpectOpError::UnsupportedOperation {
                context,
                received: ValueTypeObject(ValueObject::Float(FloatObject(123.456))),
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            } if context == outer_context.to_static()
        ));
    }

    #[test]
    fn it_should_error_by_default_against_json_string() {
        let mut outer_context = Context::new();
        let received = json!("🦊");
        let output = TestJsonExpectOp
            .on_any(&mut outer_context, &received)
            .unwrap_err();
        assert!(matches!(
            output,
            ExpectOpError::UnsupportedOperation {
                context,
                received,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            } if context == outer_context.to_static()
                && received == ValueTypeObject(ValueObject::String(StringObject("🦊".to_string())))
        ));
    }

    #[test]
    fn it_should_error_by_default_against_json_array() {
        let mut outer_context = Context::new();
        let received = json!([]);
        let output = TestJsonExpectOp
            .on_any(&mut outer_context, &received)
            .unwrap_err();
        assert!(matches!(
            output,
            ExpectOpError::UnsupportedOperation {
                context,
                received,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            } if context == outer_context.to_static()
                && received == ValueTypeObject(ValueObject::Array(ArrayObject(vec![])))
        ));
    }

    #[test]
    fn it_should_error_by_default_against_json_object() {
        let mut outer_context = Context::new();
        let received = json!({});
        let output = TestJsonExpectOp
            .on_any(&mut outer_context, &received)
            .unwrap_err();
        assert!(matches!(
            output,
            ExpectOpError::UnsupportedOperation {
                context,
                received,
                expected_operation: ExpectOpMeta {
                    name: "TestJsonExpectOp",
                    types: &[],
                },
            } if context == outer_context.to_static()
                && received == ValueTypeObject(ValueObject::Object(ObjectObject(Map::new())))
        ));
    }
}
