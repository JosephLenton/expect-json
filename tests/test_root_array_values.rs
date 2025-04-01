use expect_json::*;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn it_should_be_equal_for_empty_arrays() {
    let output = expect_json_eq(&json!([]), &json!([]));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_equal_for_identical_numeric_arrays() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([1, 2, 3]));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!([1.1, 2.2, 3.3]), &json!([1.1, 2.2, 3.3]));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!([-1, 0, 1]), &json!([-1, 0, 1]));
    assert!(output.is_ok());
}

#[test]
fn it_should_not_be_equal_for_different_numeric_arrays() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([4, 5, 6]))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        "Json at root[0] is not equal,
    expected integer 4,
        full array [4, 5, 6]
    received integer 1
        full array [1, 2, 3]"
    );
}

#[test]
fn it_should_not_be_equal_when_containing_extra_expected_value() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([1, 2]))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        "Json at root is not equal,
    expected array [1, 2],
    received array [1, 2, 3]"
    );
}

#[test]
fn it_should_not_be_equal_when_missing_expected_value() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([1, 2, 3, 4]))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        "Json at root is not equal,
    expected array [1, 2, 3, 4],
    received array [1, 2, 3]"
    );
}

#[test]
fn it_should_not_be_equal_for_arrays_of_different_types() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!(["1", "2", "3"]))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        r#"Json at root[0] is not equal,
    expected string "1",
        full array ["1", "2", "3"]
    received integer 1
        full array [1, 2, 3]"#
    );
}
