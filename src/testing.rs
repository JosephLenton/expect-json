use crate::expect_json_eq;
use assert_json_diff::assert_json_matches_no_panic;
use assert_json_diff::Config;
use pretty_assertions::assert_eq;
use serde_json::Value;

pub fn assert_json_err(received: &Value, expected: &Value, err_message: &str) {
    let output = expect_json_eq(&received, &expected)
        .unwrap_err()
        .to_string();

    let diff_err = assert_json_matches_no_panic(
        &received,
        &expected,
        Config::new(assert_json_diff::CompareMode::Strict),
    )
    .unwrap_err();
    println!(
        r#"

# With
json_eq(
    received = {received:#?},
    expected = {expected:#?},
)
>>>>>>>>>
{diff_err}
------------
{err_message}
<<<<<<<<<<

"#
    );
    assert_eq!(output, err_message);
}
