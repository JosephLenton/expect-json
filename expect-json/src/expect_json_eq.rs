use crate::ExpectJsonError;
use crate::ExpectJsonResult;
use crate::expect_core::Context;
use crate::internals::json_eq;
use serde::Serialize;

pub fn expect_json_eq<R, E>(received_raw: &R, expected_raw: &E) -> ExpectJsonResult<()>
where
    R: Serialize,
    E: Serialize,
{
    let received =
        serde_json::to_value(received_raw).map_err(ExpectJsonError::FailedToSerialiseReceived)?;
    let expected =
        serde_json::to_value(expected_raw).map_err(ExpectJsonError::FailedToSerialiseExpected)?;

    let mut context = Context::new();
    json_eq(&mut context, &received, &expected)?;

    Ok(())
}

#[cfg(test)]
mod test_expect_json_eq {
    use super::*;
    use serde::ser::Error;
    use serde_json::json;

    struct FailingSerialize;
    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(S::Error::custom("testing"))
        }
    }

    #[test]
    fn it_should_return_correct_error_if_expect_fails_to_serialise() {
        let received = json!(123);
        let expected = FailingSerialize;
        let error = expect_json_eq(&received, &expected).unwrap_err();
        let error_dbg = format!("{error:?}");

        // This funky debug oriented approach is to work around the test coverage in LLVM Cov.
        // The problem is that testing code is included, and matches causes an explosion in variants.

        // assert_matches!(error, ExpectJsonEqError::FailedToSerialiseExpected(..));
        assert!(error_dbg.starts_with("FailedToSerialiseExpected("));
    }

    #[test]
    fn it_should_return_correct_error_if_received_fails_to_serialise() {
        let received = FailingSerialize;
        let expected = json!(123);
        let error = expect_json_eq(&received, &expected).unwrap_err();
        let error_dbg = format!("{error:?}");

        // assert_matches!(error, ExpectJsonEqError::FailedToSerialiseReceived(..));
        assert!(error_dbg.starts_with("FailedToSerialiseReceived("));
    }

    #[test]
    fn it_should_return_correct_error_if_json_eq_error() {
        let received = json!(123);
        let expected = json!("🦊");
        let error = expect_json_eq(&received, &expected).unwrap_err();
        let error_dbg = format!("{error:?}");

        // assert_matches!(error, ExpectJsonError::DifferentTypes(..));
        assert!(error_dbg.starts_with("DifferentTypes"));
    }
}
