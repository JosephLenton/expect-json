use super::json_object::JsonObject;
use super::types::ValueType;
use super::Context;
use super::JsonValueEqError;
use super::JsonValueEqResult;
use crate::SerializeExpectOp;
use serde_json::Number;
use serde_json::Value;

pub trait JsonExpectOp: Into<SerializeExpectOp> {
    fn on_any<'a>(self, context: &mut Context<'a>, received: &'a Value) -> JsonValueEqResult<()> {
        match received {
            Value::Null => self.on_null(context),
            Value::Number(received_number) => self.on_number(context, received_number.clone()),
            Value::String(received_string) => self.on_string(context, received_string),
            Value::Bool(received_boolean) => self.on_boolean(context, *received_boolean),
            Value::Array(received_array) => self.on_array(context, received_array),
            Value::Object(received_object) => self.on_object(context, received_object),
        }
    }

    #[allow(unused_variables)]
    fn on_null<'a>(self, context: &mut Context<'a>) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Null,
            expected_operation: self.into(),
        })
    }

    #[allow(unused_variables)]
    fn on_number<'a>(self, context: &mut Context<'a>, received: Number) -> JsonValueEqResult<()> {
        Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Number,
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
    fn on_boolean<'a>(self, context: &mut Context<'a>, received: bool) -> JsonValueEqResult<()> {
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
