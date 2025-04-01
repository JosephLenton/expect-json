use super::JsonValueEqResult;
use crate::internals::context::Context;
use crate::internals::types::ValueType;
use crate::internals::JsonValueEqError;
use crate::SerializeExpectOp;

pub fn json_op_eq_null(
    context: &mut Context<'_>,
    expected_operation: SerializeExpectOp,
) -> JsonValueEqResult<()> {
    match expected_operation {
        #[allow(unreachable_patterns)]
        _ => Err(JsonValueEqError::UnsupportedOperation {
            context: context.to_static(),
            received_type: ValueType::Null,
            expected_operation,
        }),
    }
}
