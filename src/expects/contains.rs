use super::SerializeExpectOp;
use crate::expects::SerializeExpect;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Contains {
    values: Vec<Value>,
}

impl Contains {
    pub(crate) fn new<I, V>(values: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Value>,
    {
        Self {
            values: values.into_iter().map(Into::into).collect(),
        }
    }
}

impl Serialize for Contains {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializeExpect {
            magic_id: Default::default(),
            inner: SerializeExpectOp::from(self),
            is_not: false,
        }
        .serialize(serializer)
    }
}

impl<'a> From<&'a Contains> for SerializeExpectOp<'a> {
    fn from(contains: &'a Contains) -> Self {
        SerializeExpectOp::Contains {
            values: Cow::Borrowed(&contains.values),
        }
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
        let right = json!(expect.contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_reversed_identical_numeric_arrays() {
        let left = json!([1, 2, 3]);
        let right = json!(expect.contains([3, 2, 1]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_partial_contains() {
        let left = json!([0, 1, 2, 3, 4, 5]);
        let right = json!(expect.contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_for_totall_different_values() {
        let left = json!([0, 1, 2, 3]);
        let right = json!(expect.contains([4, 5, 6]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json array at root does not contain value,
    expected array to include the integer 4, but it was not found.
    received [0, 1, 2, 3]"#
        );
    }
}
