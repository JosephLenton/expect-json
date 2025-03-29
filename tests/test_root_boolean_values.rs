use expect_json::*;
use pretty_assertions::assert_eq;
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
        "Json at root is not equal,
    expected boolean true,
    received boolean false"
    );

    let output = expect_json_eq(&json!(false), &json!(true))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        "Json at root is not equal,
    expected boolean false,
    received boolean true"
    );
}
