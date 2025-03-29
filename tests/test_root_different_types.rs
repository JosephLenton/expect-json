use expect_json::*;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn it_should_error_if_expected_null_and_received_integer() {
    let output = expect_json_eq(&json!(null), &json!(123))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        "At root,
    expected null,
    received integer 123"
    );
}

#[test]
fn it_should_error_if_expected_null_and_received_string() {
    let output = expect_json_eq(&json!(null), &json!("🦊"))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        r#"At root,
    expected null,
    received string "🦊""#
    );
}

#[test]
fn it_should_error_if_received_null_when_not_expected() {
    let output = expect_json_eq(&json!(123.456), &json!(null))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        "At root,
    expected float 123.456,
    received null"
    );
}

#[test]
fn it_should_error_if_expected_string_and_received_null() {
    let output = expect_json_eq(&json!("🦊"), &json!(null))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        r#"At root,
    expected string "🦊",
    received null"#
    );
}
