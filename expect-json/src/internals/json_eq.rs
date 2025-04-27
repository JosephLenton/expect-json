use crate::internals::json_value_eq;
use crate::Context;
use crate::ExpectJsonResult;
use crate::SerializeExpectOp;
use serde_json::Value;

pub fn json_eq<'a>(
    context: &mut Context<'a>,
    received: &'a Value,
    expected: &'a Value,
) -> ExpectJsonResult<()> {
    if let Some(expected_op) = SerializeExpectOp::maybe_parse(expected) {
        expected_op
            .inner
            .on_any(context, received)
            .map_err(Into::into)
    } else {
        json_value_eq(context, received, expected)
    }
}
