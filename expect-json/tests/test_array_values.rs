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
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([4, 5, 6]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json integers at root[0] are not equal:
    expected 4
        full array [4, 5, 6]
    received 1
        full array [1, 2, 3]",
    );
}

#[test]
fn it_should_not_be_equal_for_different_numeric_arrays_in_sub_arrays_with_one_item_difference() {
    let output = expect_json_eq(
        &json!([[1, 1, 1], [1, 2, 4], [3, 3, 3]]),
        &json!([[1, 1, 1], [1, 2, 3], [3, 3, 3]]),
    )
    .unwrap_err()
    .to_string();

    assert_eq!(
        output,
        "Json integers at root[1][2] are not equal:
    expected 3
        full array [1, 2, 3]
    received 4
        full array [1, 2, 4]",
    );
}

#[test]
fn it_should_not_be_equal_for_different_numeric_arrays_in_sub_arrays_with_many_differences() {
    let output = expect_json_eq(
        &json!([[1, 1, 1], [6, 6, 6], [3, 3, 3]]),
        &json!([[1, 1, 1], [2, 2, 2], [3, 3, 3]]),
    )
    .unwrap_err()
    .to_string();

    assert_eq!(
        output,
        "Json integers at root[1][0] are not equal:
    expected 2
        full array [2, 2, 2]
    received 6
        full array [6, 6, 6]",
    );
}

#[test]
fn it_should_not_be_equal_when_receiving_one_extra_value_at_the_end() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([1, 2]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal, received 1 extra item at the end:
    expected [1, 2]
    received [1, 2, 3]
       extra [3]",
    );
}

#[test]
fn it_should_not_be_equal_when_receiving_many_extra_values_at_the_end() {
    let output = expect_json_eq(&json!([1, 2, 3, 4, 5]), &json!([1, 2]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal, received 3 extra items at the end:
    expected [1, 2]
    received [1, 2, 3, 4, 5]
       extra [3, 4, 5]",
    );
}

#[test]
fn it_should_not_be_equal_when_receiving_extra_values_at_the_start() {
    let output = expect_json_eq(&json!([0, 1, 2]), &json!([1, 2]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal, received 1 extra item at the start:
    expected [1, 2]
    received [0, 1, 2]
       extra [0]",
    );
}

#[test]
fn it_should_not_be_equal_when_receiving_extra_values_in_the_middle() {
    let output = expect_json_eq(&json!([1, 2, 2, 3]), &json!([1, 2, 3]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal:
    expected [1, 2, 3]
    received [1, 2, 2, 3]",
    );
}

#[test]
fn it_should_not_be_equal_when_expecting_extra_values_in_the_middle() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([1, 2, 2, 3]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal:
    expected [1, 2, 2, 3]
    received [1, 2, 3]",
    );
}

#[test]
fn it_should_not_be_equal_when_differences_in_outer_and_inner_array_lengths() {
    let output = expect_json_eq(&json!([1, [2], 3]), &json!([1, [2, 2], 3, 4]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal:
    expected [1, [2, 2], 3, 4]
    received [1, [2], 3]",
    );
}

#[test]
fn it_should_not_be_equal_when_containing_different_values() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([4, 5, 6]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json integers at root[0] are not equal:
    expected 4
        full array [4, 5, 6]
    received 1
        full array [1, 2, 3]",
    );
}

#[test]
fn it_should_not_be_equal_when_missing_expected_value_at_the_end() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([1, 2, 3, 4]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal, missing 1 item at the end:
    expected [1, 2, 3, 4]
    received [1, 2, 3]
     missing [4]",
    );
}

#[test]
fn it_should_not_be_equal_when_missing_many_expected_values_at_the_end() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!([1, 2, 3, 4, 5, 6]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal, missing 3 items at the end:
    expected [1, 2, 3, 4, 5, 6]
    received [1, 2, 3]
     missing [4, 5, 6]",
    );
}

#[test]
fn it_should_not_be_equal_when_missing_expected_value_at_the_start() {
    let output = expect_json_eq(&json!([2, 3, 4]), &json!([1, 2, 3, 4]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal, missing 1 item from the start:
    expected [1, 2, 3, 4]
    received [2, 3, 4]
     missing [1]",
    );
}

#[test]
fn it_should_not_be_equal_when_missing_many_expected_values_at_the_start() {
    let output = expect_json_eq(&json!([2, 3, 4]), &json!([-1, 0, 1, 2, 3, 4]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json arrays at root are not equal, missing 3 items from the start:
    expected [-1, 0, 1, 2, 3, 4]
    received [2, 3, 4]
     missing [-1, 0, 1]",
    );
}

#[test]
fn it_should_not_be_equal_for_arrays_of_different_types() {
    let output = expect_json_eq(&json!([1, 2, 3]), &json!(["1", "2", "3"]))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        r#"Json arrays at root[0] contain different types:
    expected string "1"
        full array ["1", "2", "3"]
    received integer 1
        full array [1, 2, 3]"#,
    );
}
