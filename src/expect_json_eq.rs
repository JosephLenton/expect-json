use crate::internals::json_eq;
use crate::internals::Context;
use crate::ExpectJsonEqError;
use crate::ExpectJsonEqResult;
use serde::Serialize;

pub fn expect_json_eq<R, E>(received_raw: &R, expected_raw: &E) -> ExpectJsonEqResult<()>
where
    R: Serialize,
    E: Serialize,
{
    let received =
        serde_json::to_value(received_raw).map_err(ExpectJsonEqError::FailedToSerialiseOther)?;
    let expected =
        serde_json::to_value(expected_raw).map_err(ExpectJsonEqError::FailedToSerialiseExpected)?;

    let mut context = Context::new();
    json_eq(&mut context, &received, &expected)?;

    Ok(())
}
