use expect_json::testing::assert_json_err;
use serde_json::json;

#[test]
fn it_should_error_if_expected_null_and_received_integer() {
    assert_json_err(
        &json!(null),
        &json!(123),
        r#"Json null received at root, expected not null:
    expected integer 123
    received null"#,
    );
}

#[test]
fn it_should_error_if_expected_null_and_received_string() {
    assert_json_err(
        &json!(null),
        &json!("🦊"),
        r#"Json null received at root, expected not null:
    expected string "🦊"
    received null"#,
    );
}

#[test]
fn it_should_error_if_received_null_when_not_expected() {
    assert_json_err(
        &json!(123.456),
        &json!(null),
        r#"Json is not null at root, expected null:
    expected null
    received float 123.456"#,
    );
}

#[test]
fn it_should_error_if_expected_string_and_received_null() {
    assert_json_err(
        &json!("🦊"),
        &json!(null),
        r#"Json is not null at root, expected null:
    expected null
    received string "🦊""#,
    );
}
