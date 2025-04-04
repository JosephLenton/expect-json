use expect_json::testing::assert_json_err;
use expect_json::*;
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
    assert_json_err(
        &json!("aaa"),
        &json!("bbb"),
        r#"Json strings at root are not equal:
    expected "bbb"
    received "aaa""#,
    );
}

#[test]
fn it_should_be_not_equal_for_different_long_strings() {
    assert_json_err(
        &json!("aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa"),
        &json!("bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb"),
        r#"Json strings at root are not equal:
    expected "bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb bbb"
    received "aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa aaa""#,
    );
}
