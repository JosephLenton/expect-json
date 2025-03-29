use crate::internals::json_value_eq;
use crate::internals::Context;
use crate::ExpectJsonEqError;
use crate::ExpectJsonEqResult;
use serde::Serialize;

pub fn expect_json_eq<E, O>(expected_raw: &E, other_raw: &O) -> ExpectJsonEqResult<()>
where
    E: Serialize,
    O: Serialize,
{
    let expected =
        serde_json::to_value(expected_raw).map_err(ExpectJsonEqError::FailedToSerialiseExpected)?;
    let other =
        serde_json::to_value(other_raw).map_err(ExpectJsonEqError::FailedToSerialiseOther)?;

    let mut context = Context::new();
    json_value_eq(&mut context, &expected, &other)?;

    Ok(())
}
