use crate::expect_op;
use crate::internals::Context;
use crate::ops::expect_object::ExpectObjectSubOp;
use crate::ExpectOp;
use crate::ExpectOpResult;
use crate::JsonType;
use serde_json::Map;
use serde_json::Value;

#[expect_op(internal, name = "object")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectObject {
    sub_ops: Vec<ExpectObjectSubOp>,
}

impl ExpectObject {
    pub(crate) fn new() -> Self {
        Self { sub_ops: vec![] }
    }

    pub fn is_empty(mut self) -> Self {
        self.sub_ops.push(ExpectObjectSubOp::IsEmpty);
        self
    }

    pub fn contains<V>(mut self, expected_values: V) -> Self
    where
        V: Into<Value>,
    {
        let value = Into::<Value>::into(expected_values);
        let sub_op = match value {
            Value::Object(values_object) => ExpectObjectSubOp::Contains(values_object),
            _ => {
                let value_type = JsonType::from(&value);
                panic!("object().contains() expected to take object. Received: {value_type}");
            }
        };

        self.sub_ops.push(sub_op);
        self
    }
}

impl ExpectOp for ExpectObject {
    fn on_object(
        &self,
        context: &mut Context,
        received: &Map<String, Value>,
    ) -> ExpectOpResult<()> {
        for sub_op in &self.sub_ops {
            sub_op.on_object(self, context, received)?;
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::Object]
    }
}

#[cfg(test)]
mod test_contains {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_equal_for_identical_objects() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right =
            json!(expect::object()
                .contains(json!({ "name": "John", "age": 30, "scores": [1, 2, 3] })));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_reversed_identical_objects() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right =
            json!(expect::object()
                .contains(json!({ "scores": [1, 2, 3], "age": 30, "name": "John" })));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_equal_for_partial_contains() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right = json!(expect::object().contains(json!({ "name": "John", "age": 30 })));

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

        let right = json!(expect::object().contains(
            json!({ "comments": expect::array().contains([
                json!({
                    "text": "Hello",
                    "author": expect::object().contains(
                        json!({
                            "name": "Jane",
                        })
                    )
                }),
            ])})
        ));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{}", output.unwrap_err().to_string());
    }

    #[test]
    fn it_should_error_for_same_fields_but_different_values() {
        let left = json!({ "name": "John", "age": 30, "scores": [1, 2, 3] });
        let right = json!(
            expect::object().contains(json!({ "name": "Joe", "age": 31, "scores": [4, 5, 6] }))
        );

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
        let right = json!(expect::object().contains(json!({})));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_ok_for_empty_on_empty_object() {
        let left = json!({});
        let right = json!(expect::object().contains(json!({})));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_error_if_used_against_the_wrong_type() {
        let left = json!("🦊");
        let right = json!(expect::object().contains(json!({})));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json comparison on unsupported type, at root:
    expect::object() cannot be performed against string,
    only supported type is: object"#
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

        let right = json!(expect::object().contains(
            json!({ "comments": expect::array().contains([
                json!({
                    "text": "Hello",
                    "author": expect::object().contains(
                        json!({
                            "name": "Jane",
                        })
                    )
                }),
            ])})
        ));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json array at root.comments does not contain expected value:
    expected array to contain {
        "author": expect::object(),
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

        let right = json!(expect::object().contains(json!({ "comment":
            expect::object().contains(
                json!({
                    "text": "Hello",
                    "author": expect::object().contains(
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

    // TODO, is this correct?
    // The error message looks like it is checking the key against an expect op.
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

        let right = json!(expect::object().contains(json!({ "comment":
            expect::object().contains(
                json!({
                    "text": "Hello",
                    "author": expect::object().contains(
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
            r#"Json object at root.comment.author is missing key for object:
    expected field 'something_else',
    but it was not found"#
        );
    }
}

#[cfg(test)]
mod test_is_empty {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_pass_when_object_is_empty() {
        let left = json!({});
        let right = json!(expect::object().is_empty());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "assertion error: {output:#?}");
    }

    #[test]
    fn it_should_fail_when_object_is_not_empty() {
        let left = json!({ "foo": "bar" });
        let right = json!(expect::object().is_empty());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            format!(
                r#"Json expect::object() error at root:
    expected empty object
    received {{
        "foo": "bar"
    }}"#
            )
        );
    }
}
