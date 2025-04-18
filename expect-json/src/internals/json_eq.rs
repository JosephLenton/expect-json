use crate::internals::json_value_eq;
use crate::internals::Context;
use crate::internals::JsonValueEqResult;
use crate::SerializeExpectOp;
use serde_json::Value;

pub fn json_eq<'a>(
    context: &mut Context<'a>,
    received: &'a Value,
    expected: &'a Value,
) -> JsonValueEqResult<()> {
    if let Some(expected_op) = SerializeExpectOp::maybe_parse(expected) {
        expected_op.inner.on_any(context, received)
    } else {
        json_value_eq(context, received, expected)
    }
}
