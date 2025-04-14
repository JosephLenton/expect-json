use crate::expects::ExpectOp;
use crate::internals::objects::ArrayObject;
use crate::internals::types::ValueType;
use crate::internals::Context;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ArrayContains {
    values: Vec<Value>,
}

impl ArrayContains {
    pub(crate) fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
}

impl ExpectOp for ArrayContains {
    fn on_array<'a>(
        &self,
        context: &mut Context<'a>,
        received_values: &'a [Value],
    ) -> JsonValueEqResult<()> {
        let received_items_in_set = received_values.iter().collect::<HashSet<&'a Value>>();

        for expected in &self.values {
            if !received_items_in_set.contains(&expected) {
                return Err(JsonValueEqError::ContainsNotFound {
                    context: context.to_static(),
                    json_type: ValueType::Array,
                    expected: expected.clone().into(),
                    received: ArrayObject::from(received_values.to_owned()).into(),
                });
            }
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "ArrayContains"
    }

    fn supported_types(&self) -> &'static [ValueType] {
        &[ValueType::Array]
    }
}

#[cfg(test)]
mod test_array_contains {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_numeric_arrays() {
        println!("a");
        let left = json!([1, 2, 3]);
        println!("b");
        let right = json!(expect.contains([1, 2, 3]));
        println!("c");

        let output = expect_json_eq(&left, &right);
        println!("d");
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
            r#"Json array at root does not contain expected value:
    expected array to contain 4, but it was not found.
    received [0, 1, 2, 3]"#
        );
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!([0, 1, 2, 3]);
        let right = json!(expect.contains(&[] as &'static [u32]));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_if_used_against_the_wrong_type() {
        let left = json!("🦊");
        let right = json!(expect.contains([4, 5, 6]));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json comparison on unsupported type, at root:
    operation ArrayContains cannot be performed against string,
    only supported type is: array"#
        );
    }
}
