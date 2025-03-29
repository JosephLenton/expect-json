use expect_json::*;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn it_should_be_equal_for_same_string_values() {
    let output = expect_json_eq(&json!("aaa"), &json!("aaa"));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(""), &json!(""));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_not_equal_for_different_short_strings() {
    let output = expect_json_eq(&json!("aaa"), &json!("bbb"))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        r#"Json at root is not equal,
    expected string "aaa",
    received string "bbb""#
    );
}

#[test]
fn it_should_be_not_equal_for_different_long_strings() {
    let output = expect_json_eq(
        &json!("aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa"),
        &json!("bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb"),
    )
    .unwrap_err()
    .to_string();

    assert_eq!(
        output,
        r#"Json at root is not equal,
    expected string "aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa",
    received string "bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb""#
    );
}
