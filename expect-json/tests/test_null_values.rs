use expect_json::*;
use serde_json::json;

#[test]
fn it_should_be_equal_for_same_null_values() {
    let output = expect_json_eq(&json!(null), &json!(null));
    assert!(output.is_ok());
}
