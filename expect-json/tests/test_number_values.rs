use expect_json::*;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::f64;
use std::i64;

#[test]
fn it_should_be_equal_for_same_f64_values() {
    let output = expect_json_eq(&json!(0.0), &json!(0.0));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(123.456), &json!(123.456));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(f64::consts::PI), &json!(f64::consts::PI));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(f64::NEG_INFINITY), &json!(f64::NEG_INFINITY));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(f64::INFINITY), &json!(f64::INFINITY));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(f64::INFINITY), &json!(f64::NEG_INFINITY));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_equal_for_same_u64_values() {
    let output = expect_json_eq(&json!(0), &json!(0));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(123), &json!(123));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(u64::MAX), &json!(u64::MAX));
    assert!(output.is_ok());
}

#[test]
fn it_should_be_equal_for_same_i64_values() {
    let output = expect_json_eq(&json!(-0), &json!(-0));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(-123), &json!(-123));
    assert!(output.is_ok());

    let output = expect_json_eq(&json!(i64::MIN), &json!(i64::MIN));
    assert!(output.is_ok());
}

#[test]
fn it_should_not_be_equal_for_different_float_values() {
    let output = expect_json_eq(&json!(123.456), &json!(456.789))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json floats at root are not equal:
    expected 456.789
    received 123.456",
    );
}

#[test]
fn it_should_not_be_equal_for_different_u64_values() {
    let output = expect_json_eq(&json!(123), &json!(456))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json integers at root are not equal:
    expected 456
    received 123",
    );
}

#[test]
fn it_should_not_be_equal_for_different_positive_and_negative_values() {
    let output = expect_json_eq(&json!(100), &json!(-100))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json integers at root are not equal:
    expected -100
    received 100",
    );

    let output = expect_json_eq(&json!(-100), &json!(100))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json integers at root are not equal:
    expected 100
    received -100",
    );
}

#[test]
fn it_should_not_be_equal_for_different_zero_types() {
    let output = expect_json_eq(&json!(0), &json!(0.0))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json numbers at root are different types:
    expected float 0.0
    received integer 0",
    );

    let output = expect_json_eq(&json!(0.0), &json!(0))
        .unwrap_err()
        .to_string();

    assert_eq!(
        output,
        "Json numbers at root are different types:
    expected integer 0
    received float 0.0",
    );
}
