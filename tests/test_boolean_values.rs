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
    let output = expect_json_eq(&json!(true), &json!(false))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json booleans at root are not equal:
    expected false
    received true",
    );

    let output = expect_json_eq(&json!(false), &json!(true))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json booleans at root are not equal:
    expected true
    received false",
    );
}
