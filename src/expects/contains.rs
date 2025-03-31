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
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_numeric_arrays() {
        let left = json!([1, 2, 3]);
        let right = json!(expect.contains([1, 2, 3]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}
