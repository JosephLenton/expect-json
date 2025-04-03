use expect_json::*;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn it_should_be_equal_for_empty_objects() {
    let output = expect_json_eq(&json!({}), &json!({}));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_equal_for_identical_complex_objects() {
    let complex = json!({
        "string": "abc123",
        "int": 123,
        "integers": [1, 2, 3],
        "float": 123,
        "floats": [1.1, 2.2, 3.3],
        "truthy": true,
        "falsy": false,
        "nullable": null,
        "sub_object": {
            "min": 10,
            "max": 20,
        },
    });

    let output = expect_json_eq(&json!(complex), &json!(complex));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_equal_for_identical_complex_objects_with_sub_objects() {
    let simple_obj = json!({
        "string": "abc123",
        "int": 123,
        "integers": [1, 2, 3],
        "float": 123,
        "floats": [1.1, 2.2, 3.3],
        "truthy": true,
        "falsy": false,
        "nullable": null,
    });

    let complex_obj = json!({
        "array_of_object": [simple_obj],
        "array_of_array_of_object": [[simple_obj], [simple_obj]],
        "obj_of_obj": {
            "inner": simple_obj
        },
        "obj_array_of_obj": {
            "inner": [simple_obj]
        },
    });

    let output = expect_json_eq(&json!(complex_obj), &json!(complex_obj));
    assert!(output.is_ok());
}

#[test]
fn it_should_error_if_expected_has_extra_field() {
    let output = expect_json_eq(&json!({}), &json!({ "extra": "🦊" }))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        r#"Json at root is not equal,
    expected object {
        extra: "🦊",
    },
    received object { }"#
    );
}

#[test]
fn it_should_error_if_received_has_extra_field() {
    let output = expect_json_eq(&json!({ "extra": "🦊" }), &json!({}))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        r#"Json object at root has extra field .extra,
    expected object { },
    received object {
        extra: "🦊",
    }"#
    );
}

#[test]
fn it_should_error_if_fields_differ_in_value() {
    let output = expect_json_eq(&json!({ "extra": "🦊" }), &json!({ "extra": "abc123" }))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        r#"Json at root.extra is not equal,
    expected string "abc123",
    received string "🦊""#
    );
}

#[test]
fn it_should_error_if_fields_differ_in_type() {
    let output = expect_json_eq(&json!({ "extra": "🦊" }), &json!({ "extra": 123 }))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        r#"Json at root.extra is not equal,
    expected integer 123,
    received string "🦊""#
    );
}

#[test]
fn it_should_error_if_fields_differ_in_numeric_type() {
    let output = expect_json_eq(&json!({ "extra": 123 }), &json!({ "extra": 123.456 }))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        r#"Json at root.extra is not equal,
    expected float 123.456,
    received integer 123"#
    );
}
