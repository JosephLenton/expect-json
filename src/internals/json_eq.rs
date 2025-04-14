use crate::internals::expect_store;
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
        println!("deserialized expected_op: {:#?}", expected_op);
        let real_op = expect_store::get_op(expected_op.store_key, expected_op.op_key).unwrap();
        real_op.on_any(context, received)
    } else {
        json_value_eq(context, received, expected)
    }
}
