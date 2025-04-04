use crate::internals::objects::IntegerObject;
use crate::internals::objects::ValueObject;
use crate::internals::types::ValueType;
use crate::internals::Context;
use crate::internals::JsonObject;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use crate::SerializeExpectOp;
use serde_json::Value;

pub trait JsonExpectOp: Into<SerializeExpectOp> {
    fn on_any<'a>(self, context: &mut Context<'a>, received: &'a Value) -> JsonValueEqResult<()> {
        match received {
            Value::Null => self.on_null(context),
            Value::Number(received_number) => {
                let value_num = ValueObject::from(received_number.clone());
                match value_num {
                    ValueObject::Float(received_float) => self.on_float(context, received_float.into()),
                    ValueObject::Integer(received_integer) => self.on_integer(context, received_integer),
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
    fn on_null(self, context: &mut Context<'_>) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Null,
            expected_operation: self.into(),
        })
    }

    #[allow(unused_variables)]
    fn on_float(self, context: &mut Context<'_>, received: f64) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Float,
            expected_operation: self.into(),
        })
    }

    #[allow(unused_variables)]
    fn on_integer(
        self,
        context: &mut Context<'_>,
        received: IntegerObject,
    ) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Integer,
            expected_operation: self.into(),
        })
    }

    #[allow(unused_variables)]
    fn on_boolean(self, context: &mut Context<'_>, received: bool) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Boolean,
            expected_operation: self.into(),
        })
    }

    #[allow(unused_variables)]
    fn on_string<'a>(self, context: &mut Context<'a>, received: &'a str) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::String,
            expected_operation: self.into(),
        })
    }

    #[allow(unused_variables)]
    fn on_array<'a>(
        self,
        context: &mut Context<'a>,
        received: &'a [Value],
    ) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Array,
            expected_operation: self.into(),
        })
    }

    #[allow(unused_variables)]
    fn on_object<'a>(
        self,
        context: &mut Context<'a>,
        received: &'a JsonObject,
    ) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Object,
            expected_operation: self.into(),
        })
    }
}
