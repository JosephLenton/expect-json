use crate::internals::objects::ArrayObject;
use crate::internals::objects::BooleanObject;
use crate::internals::objects::FloatObject;
use crate::internals::objects::IntegerObject;
use crate::internals::objects::NullObject;
use crate::internals::objects::ObjectObject;
use crate::internals::objects::StringObject;
use crate::internals::pretty_formatter::PrettyDisplay;
use crate::internals::pretty_formatter::PrettyFormatter;
use serde_json::Number;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueObject {
    Null(NullObject),
    String(StringObject),
    Float(FloatObject),
    Integer(IntegerObject),
    Boolean(BooleanObject),
    Array(ArrayObject),
    Object(ObjectObject),
}

impl ValueObject {
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Integer(_) | Self::Float(_))
    }
}

impl From<Value> for ValueObject {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null(NullObject),
            Value::String(inner) => Self::String(inner.into()),
            Value::Number(number) => number.into(),
            Value::Bool(inner) => Self::Boolean(inner.into()),
            Value::Array(inner) => Self::Array(inner.into()),
            Value::Object(inner) => Self::Object(inner.into()),
        }
    }
}

impl From<Number> for ValueObject {
    fn from(number: Number) -> Self {
        if number.is_f64() {
            let n = number
                .as_f64()
                .expect("Expected to convert serde_json::Number to f64");
            Self::Float(n.into())
        } else if number.is_u64() {
            let n = number
                .as_u64()
                .expect("Expected to convert serde_json::Number to u64");
            Self::Integer(n.into())
        } else {
            let n = number
                .as_i64()
                .expect("Expected to convert serde_json::Number to i64");
            Self::Integer(n.into())
        }
    }
}

impl From<NullObject> for ValueObject {
    fn from(inner: NullObject) -> Self {
        Self::Null(inner)
    }
}

impl From<ArrayObject> for ValueObject {
    fn from(inner: ArrayObject) -> Self {
        Self::Array(inner)
    }
}

impl From<BooleanObject> for ValueObject {
    fn from(inner: BooleanObject) -> Self {
        Self::Boolean(inner)
    }
}

impl From<StringObject> for ValueObject {
    fn from(inner: StringObject) -> Self {
        Self::String(inner)
    }
}

impl From<FloatObject> for ValueObject {
    fn from(inner: FloatObject) -> Self {
        Self::Float(inner)
    }
}

impl From<IntegerObject> for ValueObject {
    fn from(inner: IntegerObject) -> Self {
        Self::Integer(inner)
    }
}

impl From<ObjectObject> for ValueObject {
    fn from(inner: ObjectObject) -> Self {
        Self::Object(inner)
    }
}

impl Display for ValueObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let mut pretty_formatter = PrettyFormatter::new(formatter);
        self.pretty_fmt(&mut pretty_formatter)?;

        Ok(())
    }
}

impl PrettyDisplay for ValueObject {
    fn pretty_fmt(&self, formatter: &mut PrettyFormatter<'_, '_>) -> FmtResult {
        match self {
            Self::Null(inner) => inner.pretty_fmt(formatter),
            Self::String(inner) => inner.pretty_fmt(formatter),
            Self::Float(inner) => inner.pretty_fmt(formatter),
            Self::Integer(inner) => inner.pretty_fmt(formatter),
            Self::Boolean(inner) => inner.pretty_fmt(formatter),
            Self::Array(inner) => inner.pretty_fmt(formatter),
            Self::Object(inner) => inner.pretty_fmt(formatter),
        }
    }

    fn is_indenting(&self) -> bool {
        match self {
            Self::Null(inner) => inner.is_indenting(),
            Self::String(inner) => inner.is_indenting(),
            Self::Float(inner) => inner.is_indenting(),
            Self::Integer(inner) => inner.is_indenting(),
            Self::Boolean(inner) => inner.is_indenting(),
            Self::Array(inner) => inner.is_indenting(),
            Self::Object(inner) => inner.is_indenting(),
        }
    }
}

#[cfg(test)]
mod test_from {
    use super::*;
    use serde_json::json;
    use std::f64;
    use std::iter::empty;

    #[test]
    fn it_should_convert_from_json_null() {
        let output = ValueObject::from(json!(null));
        assert_eq!(output, ValueObject::Null(NullObject));
    }

    #[test]
    fn it_should_convert_from_json_boolean() {
        let output = ValueObject::from(json!(true));
        assert_eq!(output, ValueObject::Boolean(BooleanObject::from(true)));

        let output = ValueObject::from(json!(false));
        assert_eq!(output, ValueObject::Boolean(BooleanObject::from(false)));
    }

    #[test]
    fn it_should_convert_from_json_floats() {
        let output = ValueObject::from(json!(0.0));
        assert_eq!(output, ValueObject::Float(FloatObject::from(0.0)));

        let output = ValueObject::from(json!(123.456));
        assert_eq!(output, ValueObject::Float(FloatObject::from(123.456)));

        let output = ValueObject::from(json!(f64::MAX));
        assert_eq!(output, ValueObject::Float(FloatObject::from(f64::MAX)));

        let output = ValueObject::from(json!(f64::MIN));
        assert_eq!(output, ValueObject::Float(FloatObject::from(f64::MIN)));

        let output = ValueObject::from(json!(f64::consts::PI));
        assert_eq!(
            output,
            ValueObject::Float(FloatObject::from(f64::consts::PI))
        );
    }

    #[test]
    fn it_should_convert_from_json_integers() {
        let output = ValueObject::from(json!(0));
        assert_eq!(output, ValueObject::Integer(IntegerObject::from(0_u64)));

        let output = ValueObject::from(json!(123));
        assert_eq!(output, ValueObject::Integer(IntegerObject::from(123_u64)));

        let output = ValueObject::from(json!(u64::MAX));
        assert_eq!(output, ValueObject::Integer(IntegerObject::from(u64::MAX)));

        let output = ValueObject::from(json!(u64::MIN));
        assert_eq!(output, ValueObject::Integer(IntegerObject::from(u64::MIN)));

        let output = ValueObject::from(json!(i64::MAX));
        assert_eq!(
            output,
            ValueObject::Integer(IntegerObject::from(i64::MAX as u64))
        );

        let output = ValueObject::from(json!(i64::MIN));
        assert_eq!(output, ValueObject::Integer(IntegerObject::from(i64::MIN)));
    }

    #[test]
    fn it_should_convert_from_json_strings() {
        let output = ValueObject::from(json!(""));
        assert_eq!(output, ValueObject::String(StringObject::from("")));

        let output = ValueObject::from(json!("abc123"));
        assert_eq!(output, ValueObject::String(StringObject::from("abc123")));
    }

    #[test]
    fn it_should_convert_from_json_arrays() {
        let output = ValueObject::from(json!([]));
        assert_eq!(output, ValueObject::Array(ArrayObject::from(empty())));

        let output = ValueObject::from(json!([0, 123.456, "something"]));
        assert_eq!(
            output,
            ValueObject::Array(ArrayObject::from([
                json!(0),
                json!(123.456),
                json!("something")
            ]))
        );
    }

    #[test]
    fn it_should_convert_from_json_objects() {
        let output = ValueObject::from(json!({}));
        assert_eq!(
            output,
            ValueObject::Object(ObjectObject::from_iter(empty()))
        );

        let output = ValueObject::from(json!({
            "age": 30,
            "name": "Joe",
            "ids": [1, 2, 3],
        }));
        assert_eq!(
            output,
            ValueObject::Object(ObjectObject::from_iter([
                ("age".to_string(), json!(30)),
                ("name".to_string(), json!("Joe")),
                ("ids".to_string(), json!([1, 2, 3])),
            ]))
        );
    }
}

#[cfg(test)]
mod test_fmt {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Map;
    use std::iter::empty;

    #[test]
    fn it_should_display_null() {
        let output = format!("{}", ValueObject::from(NullObject));
        assert_eq!(output, "null");
    }

    #[test]
    fn it_should_display_boolean() {
        let output = format!("{}", ValueObject::from(BooleanObject::from(true)));
        assert_eq!(output, "true");

        let output = format!("{}", ValueObject::from(BooleanObject::from(false)));
        assert_eq!(output, "false");
    }

    #[test]
    fn it_should_display_integer_type() {
        let output = format!("{}", ValueObject::from(IntegerObject::from(0_u64)));
        assert_eq!(output, "0");

        let output = format!("{}", ValueObject::from(IntegerObject::from(123_u64)));
        assert_eq!(output, "123");

        let output = format!("{}", ValueObject::from(IntegerObject::from(0_i64)));
        assert_eq!(output, "0");

        let output = format!("{}", ValueObject::from(IntegerObject::from(123_i64)));
        assert_eq!(output, "123");

        let output = format!("{}", ValueObject::from(IntegerObject::from(-123_i64)));
        assert_eq!(output, "-123");
    }

    #[test]
    fn it_should_display_float_type() {
        let output = format!("{}", ValueObject::from(FloatObject::from(0.0)));
        assert_eq!(output, "0.0");

        let output = format!("{}", ValueObject::from(FloatObject::from(123.456)));
        assert_eq!(output, "123.456");

        let output = format!("{}", ValueObject::from(FloatObject::from(-123.456)));
        assert_eq!(output, "-123.456");
    }

    #[test]
    fn it_should_display_string_type() {
        let output = format!("{}", ValueObject::from(StringObject::from("")));
        assert_eq!(output, r#""""#);

        let output = format!("{}", ValueObject::from(StringObject::from("something")));
        assert_eq!(output, r#""something""#);
    }

    #[test]
    fn it_should_display_array_type() {
        let output = format!("{}", ValueObject::from(ArrayObject::from(empty())));
        assert_eq!(output, "[]");
    }

    #[test]
    fn it_should_display_object_type() {
        let obj = Map::from_iter(empty());
        let output = format!("{}", ValueObject::from(ObjectObject::from(obj)));
        assert_eq!(output, "{}");
    }
}
