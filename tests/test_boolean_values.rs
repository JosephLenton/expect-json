use expect_json::testing::assert_json_err;
use expect_json::*;
use serde_json::json;

#[test]
fn it_should_be_equal_for_same_boolean_values() {
    let output = expect_json_eq(&json!(true), &json!(true));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(false), &json!(false));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_not_equal_for_different_boolean_values() {
    assert_json_err(
        &json!(true),
        &json!(false),
        "Json booleans at root are not equal:
    expected false,
    received true",
    );

    assert_json_err(
        &json!(false),
        &json!(true),
        "Json booleans at root are not equal:
    expected true,
    received false",
    );
}
