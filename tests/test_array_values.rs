use expect_json::testing::assert_json_err;
use expect_json::*;
use serde_json::json;

#[test]
fn it_should_be_equal_for_empty_arrays() {
    let output = expect_json_eq(&json!([]), &json!([]));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_equal_for_identical_array_of_numbers() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([1, 2, 3]));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!([1.1, 2.2, 3.3]), &json!([1.1, 2.2, 3.3]));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!([-1, 0, 1]), &json!([-1, 0, 1]));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_equal_for_identical_array_of_objects() {
    let output = expect_json_eq(&json!([{}, {}]), &json!([{}, {}]));
    assert!(output.is_ok());

    let output = expect_json_eq(
        &json!([{ "min": 1, "max": 2 }]),
        &json!([{ "min": 1, "max": 2 }]),
    );
    assert!(output.is_ok());
}

#[test]
fn it_should_not_be_equal_for_different_numeric_arrays() {
    assert_json_err(
        &json!([1, 2, 3]),
        &json!([4, 5, 6]),
        "Json integers at root[0] are not equal:
    expected 4,
        full array [4, 5, 6]
    received 1
        full array [1, 2, 3]",
    );
}

#[test]
fn it_should_not_be_equal_for_different_numeric_arrays_in_sub_arrays() {
    assert_json_err(
        &json!([[1, 1, 1], [6, 6, 6], [3, 3, 3]]),
        &json!([[1, 1, 1], [2, 2, 2], [3, 3, 3]]),
        "Json integers at root[1][0] are not equal:
    expected 2,
        full array [2, 2, 2]
    received 6
        full array [6, 6, 6]",
    );
}

#[test]
fn it_should_not_be_equal_when_containing_extra_expected_value() {
    assert_json_err(
        &json!([1, 2, 3]),
        &json!([1, 2]),
        "Json arrays at root are not equal, received has extra data:
    expected [1, 2],
    received [1, 2, 3]",
    );
}

#[test]
fn it_should_not_be_equal_when_missing_expected_value() {
    assert_json_err(
        &json!([1, 2, 3]),
        &json!([1, 2, 3, 4]),
        "Json arrays at root are not equal, received is missing data:
    expected [1, 2, 3, 4],
    received [1, 2, 3]",
    );
}

#[test]
fn it_should_not_be_equal_for_arrays_of_different_types() {
    assert_json_err(
        &json!([1, 2, 3]),
        &json!(["1", "2", "3"]),
        r#"Json arrays at root[0] are not equal:
    expected string "1",
        full array ["1", "2", "3"]
    received integer 1
        full array [1, 2, 3]"#,
    );
}
