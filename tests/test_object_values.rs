use expect_json::testing::assert_json_err;
use expect_json::*;
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
    assert_json_err(
        &json!({}),
        &json!({ "extra": "🦊" }),
        r#"Json objects at root are not equal:
    expected {
        "extra": "🦊"
    }
    received {}"#,
    );
}

#[test]
fn it_should_error_if_objects_have_same_number_but_different_fields() {
    assert_json_err(
        &json!({ "aaa": "🦊" }),
        &json!({ "bbb": "🦊" }),
        r#"Json objects at root are not equal:
    expected object key 'bbb',
    but it was not found"#,
    );
}

#[test]
fn it_should_have_appropriate_error_message_on_fields_with_spaces() {
    assert_json_err(
        &json!({}),
        &json!({ "something extra with spaces": "🦊" }),
        r#"Json objects at root are not equal:
    expected {
        "something extra with spaces": "🦊"
    }
    received {}"#,
    );
}

#[test]
fn it_should_error_if_received_has_extra_field() {
    assert_json_err(
        &json!({ "extra": "🦊" }),
        &json!({}),
        r#"Json object at root has extra field "extra":
    expected {}
    received {
        "extra": "🦊"
    }"#,
    );
}

#[test]
fn it_should_error_if_fields_differ_in_value() {
    assert_json_err(
        &json!({ "extra": "🦊" }),
        &json!({ "extra": "abc123" }),
        r#"Json strings at root.extra are not equal:
    expected "abc123"
    received "🦊""#,
    );
}

#[test]
fn it_should_error_if_fields_differ_in_type() {
    assert_json_err(
        &json!({ "extra": "🦊" }),
        &json!({ "extra": 123 }),
        r#"Json values at root.extra are different types:
    expected integer 123
    received string "🦊""#,
    );
}

#[test]
fn it_should_error_if_fields_differ_in_numeric_type() {
    assert_json_err(
        &json!({ "extra": 123 }),
        &json!({ "extra": 123.456 }),
        r#"Json numbers at root.extra are different types:
    expected float 123.456
    received integer 123"#,
    );
}

#[test]
fn it_should_pretty_print_big_objects_when_it_has_one_extra_field() {
    let received_obj = json!({
        "obj_of_obj": {
            "inner": {
                "string": "abc123",
                "int": 123,
                "integers": [1, 2, 3],
                "float": 123,
                "floats": [1.1, 2.2, 3.3],
                "truthy": true,
                "falsy": false,
                "nullable": null,
            }
        },
    });
    let expected_obj = json!({});

    assert_json_err(
        &received_obj,
        &expected_obj,
        r#"Json object at root has extra field "obj_of_obj":
    expected {}
    received {
        "obj_of_obj": {
            "inner": {
                "falsy": false,
                "float": 123,
                "floats": [1.1, 2.2, 3.3],
                "int": 123,
                "integers": [1, 2, 3],
                "nullable": null,
                "string": "abc123",
                "truthy": true
            }
        }
    }"#,
    );
}

#[test]
fn it_should_pretty_print_big_objects_when_it_has_many_extra_fields() {
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

    let received_obj = json!({
        "array_of_object": [simple_obj],
        "array_of_array_of_object": [[simple_obj], [simple_obj]],
        "obj_of_obj": {
            "inner": simple_obj
        },
        "obj_array_of_obj": {
            "inner": [simple_obj]
        },
    });
    let expected_obj = json!({});

    assert_json_err(
        &received_obj,
        &expected_obj,
        r#"Json object at root has many extra fields over expected:
    expected {}
    received {
        "array_of_array_of_object": [
            [
                {
                    "falsy": false,
                    "float": 123,
                    "floats": [1.1, 2.2, 3.3],
                    "int": 123,
                    "integers": [1, 2, 3],
                    "nullable": null,
                    "string": "abc123",
                    "truthy": true
                }
            ],
            [
                {
                    "falsy": false,
                    "float": 123,
                    "floats": [1.1, 2.2, 3.3],
                    "int": 123,
                    "integers": [1, 2, 3],
                    "nullable": null,
                    "string": "abc123",
                    "truthy": true
                }
            ]
        ],
        "array_of_object": [
            {
                "falsy": false,
                "float": 123,
                "floats": [1.1, 2.2, 3.3],
                "int": 123,
                "integers": [1, 2, 3],
                "nullable": null,
                "string": "abc123",
                "truthy": true
            }
        ],
        "obj_array_of_obj": {
            "inner": [
                {
                    "falsy": false,
                    "float": 123,
                    "floats": [1.1, 2.2, 3.3],
                    "int": 123,
                    "integers": [1, 2, 3],
                    "nullable": null,
                    "string": "abc123",
                    "truthy": true
                }
            ]
        },
        "obj_of_obj": {
            "inner": {
                "falsy": false,
                "float": 123,
                "floats": [1.1, 2.2, 3.3],
                "int": 123,
                "integers": [1, 2, 3],
                "nullable": null,
                "string": "abc123",
                "truthy": true
            }
        }
    }

    extra fields in received:
        array_of_array_of_object,
        array_of_object,
        obj_array_of_obj,
        obj_of_obj,
"#,
    );
}
