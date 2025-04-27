use crate::expect_op;
use crate::ops::expect_array::ExpectArraySubOp;
use crate::Context;
use crate::ExpectOp;
use crate::ExpectOpResult;
use crate::JsonType;
use serde_json::Value;
use std::fmt::Debug;

#[expect_op(internal, name = "array")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectArray {
    sub_ops: Vec<ExpectArraySubOp>,
}

impl ExpectArray {
    pub(crate) fn new() -> Self {
        Self { sub_ops: vec![] }
    }

    pub fn is_empty(mut self) -> Self {
        self.sub_ops.push(ExpectArraySubOp::IsEmpty);
        self
    }

    pub fn is_not_empty(mut self) -> Self {
        self.sub_ops.push(ExpectArraySubOp::IsNotEmpty);
        self
    }

    pub fn min_len(mut self, min_len: usize) -> Self {
        self.sub_ops.push(ExpectArraySubOp::MinLen(min_len));
        self
    }

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.sub_ops.push(ExpectArraySubOp::MaxLen(max_len));
        self
    }

    pub fn contains<I, V>(mut self, expected_values: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Value>,
    {
        let inner_expected_values = expected_values
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        self.sub_ops
            .push(ExpectArraySubOp::Contains(inner_expected_values));
        self
    }
}

impl ExpectOp for ExpectArray {
    fn on_array(&self, context: &mut Context, received: &[Value]) -> ExpectOpResult<()> {
        for sub_op in &self.sub_ops {
            sub_op.on_array(self, context, received)?;
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::Array]
    }
}

#[cfg(test)]
mod test_contains {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_numeric_arrays() {
        let left = json!([1, 2, 3]);
        let right = json!(expect::array().contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_reversed_identical_numeric_arrays() {
        let left = json!([1, 2, 3]);
        let right = json!(expect::array().contains([3, 2, 1]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_partial_contains() {
        let left = json!([0, 1, 2, 3, 4, 5]);
        let right = json!(expect::array().contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_for_totall_different_values() {
        let left = json!([0, 1, 2, 3]);
        let right = json!(expect::array().contains([4, 5, 6]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json array at root does not contain expected value:
    expected array to contain 4, but it was not found.
    received [0, 1, 2, 3]"#
        );
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!([0, 1, 2, 3]);
        let right = json!(expect::array().contains([] as [u32; 0]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_if_used_against_the_wrong_type() {
        let left = json!("🦊");
        let right = json!(expect::array().contains([4, 5, 6]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::array() at root, received wrong type:
    expected array
    received string "🦊""#
        );
    }

    #[test]
    fn it_should_handle_nested_contains() {
        let left = json!([
            {
                "text": "Hello",
                "author": "Jane Candle"
            },
            {
                "text": "Goodbye",
                "author": "John Lighthouse"
            }
        ]);

        let right = json!(expect::array().contains([json!({
            "text": "Hello",
            "author": expect::string().contains("Jane"),
        }),]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{}", output.unwrap_err().to_string());
    }
}

#[cfg(test)]
mod test_is_empty {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_array_is_empty() {
        let left = json!([]);
        let right = json!(expect::array().is_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_array_is_not_empty() {
        let left = json!([1, 2, 3]);
        let right = json!(expect::array().is_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::array() error at root:
    expected empty array
    received [1, 2, 3]"#
            )
        );
    }
}

#[cfg(test)]
mod test_is_not_empty {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_array_is_not_empty() {
        let left = json!([1]);
        let right = json!(expect::array().is_not_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_array_is_empty() {
        let left = json!([]);
        let right = json!(expect::array().is_not_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::array() error at root:
    expected non-empty array
    received []"#
            )
        );
    }
}

#[cfg(test)]
mod test_min_len {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_array_has_exactly_enough_elements() {
        let left = json!([1, 2, 3]);
        let right = json!(expect::array().min_len(3));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_pass_when_array_has_more_than_enough_elements() {
        let left = json!([1, 2, 3, 4, 5]);
        let right = json!(expect::array().min_len(3));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_array_has_too_few_elements() {
        let left = json!([1, 2, 3]);
        let right = json!(expect::array().min_len(4));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::array() error at root:
    expected array to have at least 4 elements, but it has 3,
    received [1, 2, 3]"#
            )
        );
    }
}

#[cfg(test)]
mod test_max_len {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_array_has_exactly_enough_elements() {
        let left = json!([1, 2, 3]);
        let right = json!(expect::array().max_len(3));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_pass_when_array_has_less_than_enough_elements() {
        let left = json!([1, 2]);
        let right = json!(expect::array().max_len(6));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_array_has_too_few_elements() {
        let left = json!([1, 2, 3, 4]);
        let right = json!(expect::array().max_len(3));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::array() error at root:
    expected array to have at most 3 elements, but it has 4,
    received [1, 2, 3, 4]"#
            )
        );
    }
}
