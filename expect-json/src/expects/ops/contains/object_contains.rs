use crate::expect_op;
use crate::expects::ExpectOp;
use crate::internals::Context;
use crate::internals::ExpectOpMeta;
use crate::internals::JsonValueEqError;
use crate::internals::JsonValueEqResult;
use crate::JsonType;
use serde_json::Map;
use serde_json::Value;

#[expect_op(internal)]
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectContains {
    values: Map<String, Value>,
}

impl ObjectContains {
    pub(crate) fn new(values: Map<String, Value>) -> Self {
        Self { values }
    }
}

impl ExpectOp for ObjectContains {
    fn on_object(
        &self,
        context: &mut Context,
        received_values: &Map<String, Value>,
    ) -> JsonValueEqResult<()> {
        for (key, expected_value) in &self.values {
            let received_value = received_values.get(key).ok_or_else(|| {
                JsonValueEqError::ObjectKeyMissingForExpectOp {
                    context: context.to_static(),
                    expected_key: key.to_owned(),
                    expected_operation: ExpectOpMeta::new(self),
                }
            })?;

            context.push(key.to_owned());
            context.json_eq(received_value, expected_value)?;
            context.pop();
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::Array]
    }
}

#[cfg(test)]
mod test_object_contains {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_objects() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right =
            json!(expect.contains(json!({ "name": "John", "age": 30, "scores": [1, 2, 3] })));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_reversed_identical_objects() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right =
            json!(expect.contains(json!({ "scores": [1, 2, 3], "age": 30, "name": "John" })));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_partial_contains() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right = json!(expect.contains(json!({ "name": "John", "age": 30 })));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_nested_contains() {
        let left = json!({ "name": "John", "comments": [
            {
                "text": "Hello",
                "author": {
                    "name": "Jane",
                    "age": 25
                }
            },
            {
                "text": "Goodbye",
                "author": {
                    "name": "John",
                    "age": 30
                }
            }
        ]});

        let right = json!(expect.contains(json!({ "comments": expect.contains([
            json!({
                "text": "Hello",
                "author": expect.contains(
                    json!({
                        "name": "Jane",
                    })
                )
            }),
        ])})));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{}", output.unwrap_err().to_string());
    }

    #[test]
    fn it_should_error_for_same_fields_but_different_values() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right =
            json!(expect.contains(json!({ "name": "Joe", "age": 31, "scores": [4, 5, 6] })));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json integers at root.age are not equal:
    expected 31
    received 30"#
        );
    }

    #[test]
    fn it_should_be_ok_for_empty_contains() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right = json!(expect.contains(json!({})));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_ok_for_empty_on_empty_object() {
        let left = json!({});
        let right = json!(expect.contains(json!({})));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_if_used_against_the_wrong_type() {
        let left = json!("🦊");
        let right = json!(expect.contains(json!({})));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json comparison on unsupported type, at root:
    expect.Contains() cannot be performed against string,
    only supported type is: array"#
        );
    }

    #[test]
    fn it_should_error_for_nested_contains_via_array_on_differences() {
        let left = json!({ "name": "John", "comments": [
            {
                "text": "Hello",
                "author": {
                    "name": "🦊",
                    "age": 25
                }
            },
            {
                "text": "Goodbye",
                "author": {
                    "name": "John",
                    "age": 30
                }
            }
        ]});

        let right = json!(expect.contains(json!({ "comments": expect.contains([
            json!({
                "text": "Hello",
                "author": expect.contains(
                    json!({
                        "name": "Jane",
                    })
                )
            }),
        ])})));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json array at root.comments does not contain expected value:
    expected array to contain {
        "author": expect.Contains(),
        "text": "Hello"
    }, but it was not found.
    received [
        {
            "author": {
                "age": 25,
                "name": "🦊"
            },
            "text": "Hello"
        },
        {
            "author": {
                "age": 30,
                "name": "John"
            },
            "text": "Goodbye"
        }
    ]"#
        );
    }

    #[test]
    fn it_should_error_for_nested_contains_via_object_with_inner_contains_error() {
        let left = json!({
            "name": "John",
            "comment": {
                "text": "Hello",
                "author": {
                    "name": "🦊",
                    "age": 25
                }
            },
        });

        let right = json!(expect.contains(json!({ "comment":
            expect.contains(
                json!({
                    "text": "Hello",
                    "author": expect.contains(
                        json!({
                            "name": "Jane",
                        })
                    )
                })
            )
        })));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json strings at root.comment.author.name are not equal:
    expected "Jane"
    received "🦊""#
        );
    }

    #[test]
    fn it_should_error_for_nested_contains_via_different_object_with_inner_contains_error() {
        let left = json!({
            "name": "John",
            "comment": {
                "text": "Hello",
                "author": {
                    "name": "Jane",
                    "age": 25
                }
            },
        });

        let right = json!(expect.contains(json!({ "comment":
            expect.contains(
                json!({
                    "text": "Hello",
                    "author": expect.contains(
                        json!({
                            "something_else": "🦊",
                        })
                    )
                })
            )
        })));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json object at root.comment.author is missing key for ObjectContains:
    expected field 'something_else',
    but it was not found"#
        );
    }
}
