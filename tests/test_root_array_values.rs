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
    use assert_json_diff::assert_json_eq;
    // assert_json_eq!(json!(1), json!(2));
    // assert_json_eq!(json!([1, 2, 3]), json!([4, 5, 6]),);

    let output = expect_json_eq(&json!([1, 2, 3]), &json!([4, 5, 6]))
        .unwrap_err()
        .to_string();
    assert_eq!(
        output,
        "Json at root[0] is not equal,
    expected integer 1,
        full array [1, 2, 3]
    received integer 4
        full array [4, 5, 6]"
    );
}
