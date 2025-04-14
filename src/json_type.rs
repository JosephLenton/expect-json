use serde_json::Number;
use serde_json::Value;
use std::borrow::Borrow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

/// A helper enum to represent the many types contained in Json.
///
/// This can be built from looking at Json Values.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum JsonType {
    Null,
    String,
    Float,
    Integer,
    Boolean,
    Array,
    Object,
}

impl From<&Value> for JsonType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::String(_) => Self::String,
            Value::Number(number) => number.into(),
            Value::Bool(_) => Self::Boolean,
            Value::Array(_) => Self::Array,
            Value::Object(_) => Self::Object,
        }
    }
}

impl From<&Number> for JsonType {
    fn from(number: &Number) -> Self {
        if number.is_f64() {
            JsonType::Float
        } else {
            JsonType::Integer
        }
    }
}

impl Display for JsonType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let value_type_str: &str = self.borrow();
        write!(formatter, "{value_type_str}")
    }
}

impl Borrow<str> for JsonType {
    fn borrow(&self) -> &'static str {
        match *self {
            Self::Null => "null",
            Self::String => "string",
            Self::Float => "float",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::Array => "array",
            Self::Object => "object",
        }
    }
}

#[cfg(test)]
mod test_from {
    use super::*;
    use serde_json::json;
    use std::f64;

    #[test]
    fn it_should_convert_json_floats_to_float() {
        let output = JsonType::from(&json!(0.0));
        assert_eq!(output, JsonType::Float);

        let output = JsonType::from(&json!(123.456));
        assert_eq!(output, JsonType::Float);

        let output = JsonType::from(&json!(f64::MAX));
        assert_eq!(output, JsonType::Float);

        let output = JsonType::from(&json!(f64::MIN));
        assert_eq!(output, JsonType::Float);

        let output = JsonType::from(&json!(f64::consts::PI));
        assert_eq!(output, JsonType::Float);
    }

    #[test]
    fn it_should_convert_json_ints_to_integer() {
        let output = JsonType::from(&json!(0));
        assert_eq!(output, JsonType::Integer);

        let output = JsonType::from(&json!(123));
        assert_eq!(output, JsonType::Integer);

        let output = JsonType::from(&json!(u64::MAX));
        assert_eq!(output, JsonType::Integer);

        let output = JsonType::from(&json!(u64::MIN));
        assert_eq!(output, JsonType::Integer);

        let output = JsonType::from(&json!(i64::MAX));
        assert_eq!(output, JsonType::Integer);

        let output = JsonType::from(&json!(i64::MIN));
        assert_eq!(output, JsonType::Integer);
    }

    #[test]
    fn it_should_convert_json_null_to_null() {
        let output = JsonType::from(&json!(null));
        assert_eq!(output, JsonType::Null);
    }

    #[test]
    fn it_should_convert_json_boolean_to_boolean() {
        let output = JsonType::from(&json!(true));
        assert_eq!(output, JsonType::Boolean);

        let output = JsonType::from(&json!(false));
        assert_eq!(output, JsonType::Boolean);
    }

    #[test]
    fn it_should_convert_json_string_to_string() {
        let output = JsonType::from(&json!(""));
        assert_eq!(output, JsonType::String);

        let output = JsonType::from(&json!("abc123"));
        assert_eq!(output, JsonType::String);
    }

    #[test]
    fn it_should_convert_json_array_to_array() {
        let output = JsonType::from(&json!([]));
        assert_eq!(output, JsonType::Array);

        let output = JsonType::from(&json!([0, 123.456, "something"]));
        assert_eq!(output, JsonType::Array);
    }

    #[test]
    fn it_should_convert_json_object_to_object() {
        let output = JsonType::from(&json!({}));
        assert_eq!(output, JsonType::Object);

        let output = JsonType::from(&json!({
            "age": 30,
            "name": "Joe",
            "ids": [1, 2, 3],
        }));
        assert_eq!(output, JsonType::Object);
    }
}

#[cfg(test)]
mod test_fmt {
    use super::*;

    #[test]
    fn it_should_display_null() {
        let output = format!("{}", JsonType::Null);
        assert_eq!(output, "null");
    }

    #[test]
    fn it_should_display_boolean() {
        let output = format!("{}", JsonType::Boolean);
        assert_eq!(output, "boolean");
    }

    #[test]
    fn it_should_display_integer_type() {
        let output = format!("{}", JsonType::Integer);
        assert_eq!(output, "integer");
    }

    #[test]
    fn it_should_display_float_type() {
        let output = format!("{}", JsonType::Float);
        assert_eq!(output, "float");
    }

    #[test]
    fn it_should_display_string_type() {
        let output = format!("{}", JsonType::String);
        assert_eq!(output, "string");
    }

    #[test]
    fn it_should_display_array_type() {
        let output = format!("{}", JsonType::Array);
        assert_eq!(output, "array");
    }

    #[test]
    fn it_should_display_object_type() {
        let output = format!("{}", JsonType::Object);
        assert_eq!(output, "object");
    }
}
