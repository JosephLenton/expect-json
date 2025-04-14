use serde_json::Number;
use serde_json::Value;
use std::borrow::Borrow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ValueType {
    Null,
    String,
    Float,
    Integer,
    Boolean,
    Array,
    Object,
}

impl From<&Value> for ValueType {
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

impl From<&Number> for ValueType {
    fn from(number: &Number) -> Self {
        if number.is_f64() {
            ValueType::Float
        } else {
            ValueType::Integer
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let value_type_str: &str = self.borrow();
        write!(formatter, "{value_type_str}")
    }
}

impl Borrow<str> for ValueType {
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
        let output = ValueType::from(&json!(0.0));
        assert_eq!(output, ValueType::Float);

        let output = ValueType::from(&json!(123.456));
        assert_eq!(output, ValueType::Float);

        let output = ValueType::from(&json!(f64::MAX));
        assert_eq!(output, ValueType::Float);

        let output = ValueType::from(&json!(f64::MIN));
        assert_eq!(output, ValueType::Float);

        let output = ValueType::from(&json!(f64::consts::PI));
        assert_eq!(output, ValueType::Float);
    }

    #[test]
    fn it_should_convert_json_ints_to_integer() {
        let output = ValueType::from(&json!(0));
        assert_eq!(output, ValueType::Integer);

        let output = ValueType::from(&json!(123));
        assert_eq!(output, ValueType::Integer);

        let output = ValueType::from(&json!(u64::MAX));
        assert_eq!(output, ValueType::Integer);

        let output = ValueType::from(&json!(u64::MIN));
        assert_eq!(output, ValueType::Integer);

        let output = ValueType::from(&json!(i64::MAX));
        assert_eq!(output, ValueType::Integer);

        let output = ValueType::from(&json!(i64::MIN));
        assert_eq!(output, ValueType::Integer);
    }

    #[test]
    fn it_should_convert_json_null_to_null() {
        let output = ValueType::from(&json!(null));
        assert_eq!(output, ValueType::Null);
    }

    #[test]
    fn it_should_convert_json_boolean_to_boolean() {
        let output = ValueType::from(&json!(true));
        assert_eq!(output, ValueType::Boolean);

        let output = ValueType::from(&json!(false));
        assert_eq!(output, ValueType::Boolean);
    }

    #[test]
    fn it_should_convert_json_string_to_string() {
        let output = ValueType::from(&json!(""));
        assert_eq!(output, ValueType::String);

        let output = ValueType::from(&json!("abc123"));
        assert_eq!(output, ValueType::String);
    }

    #[test]
    fn it_should_convert_json_array_to_array() {
        let output = ValueType::from(&json!([]));
        assert_eq!(output, ValueType::Array);

        let output = ValueType::from(&json!([0, 123.456, "something"]));
        assert_eq!(output, ValueType::Array);
    }

    #[test]
    fn it_should_convert_json_object_to_object() {
        let output = ValueType::from(&json!({}));
        assert_eq!(output, ValueType::Object);

        let output = ValueType::from(&json!({
            "age": 30,
            "name": "Joe",
            "ids": [1, 2, 3],
        }));
        assert_eq!(output, ValueType::Object);
    }
}

#[cfg(test)]
mod test_fmt {
    use super::*;

    #[test]
    fn it_should_display_null() {
        let output = format!("{}", ValueType::Null);
        assert_eq!(output, "null");
    }

    #[test]
    fn it_should_display_boolean() {
        let output = format!("{}", ValueType::Boolean);
        assert_eq!(output, "boolean");
    }

    #[test]
    fn it_should_display_integer_type() {
        let output = format!("{}", ValueType::Integer);
        assert_eq!(output, "integer");
    }

    #[test]
    fn it_should_display_float_type() {
        let output = format!("{}", ValueType::Float);
        assert_eq!(output, "float");
    }

    #[test]
    fn it_should_display_string_type() {
        let output = format!("{}", ValueType::String);
        assert_eq!(output, "string");
    }

    #[test]
    fn it_should_display_array_type() {
        let output = format!("{}", ValueType::Array);
        assert_eq!(output, "array");
    }

    #[test]
    fn it_should_display_object_type() {
        let output = format!("{}", ValueType::Object);
        assert_eq!(output, "object");
    }
}
