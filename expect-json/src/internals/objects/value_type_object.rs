use crate::internals::objects::ArrayObject;
use crate::internals::objects::BooleanObject;
use crate::internals::objects::FloatObject;
use crate::internals::objects::IntegerObject;
use crate::internals::objects::NullObject;
use crate::internals::objects::ObjectObject;
use crate::internals::objects::StringObject;
use crate::internals::objects::ValueObject;
use serde_json::Map;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueTypeObject(pub ValueObject);

impl ValueTypeObject {
    pub fn is_number(&self) -> bool {
        self.0.is_number()
    }
}

impl From<Value> for ValueTypeObject {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self(NullObject.into()),
            Value::String(inner) => Self(StringObject::from(inner).into()),
            Value::Number(inner) => Self(inner.into()),
            Value::Bool(inner) => Self(BooleanObject::from(inner).into()),
            Value::Array(inner) => Self(ArrayObject::from(inner).into()),
            Value::Object(inner) => Self(ObjectObject::from(inner).into()),
        }
    }
}

impl From<bool> for ValueTypeObject {
    fn from(value: bool) -> Self {
        Self(BooleanObject::from(value).into())
    }
}

impl From<u64> for ValueTypeObject {
    fn from(value: u64) -> Self {
        Self(IntegerObject::from(value).into())
    }
}

impl From<i64> for ValueTypeObject {
    fn from(value: i64) -> Self {
        Self(IntegerObject::from(value).into())
    }
}

impl From<f64> for ValueTypeObject {
    fn from(value: f64) -> Self {
        Self(FloatObject::from(value).into())
    }
}

impl From<String> for ValueTypeObject {
    fn from(value: String) -> Self {
        Self(StringObject::from(value).into())
    }
}

impl From<Vec<Value>> for ValueTypeObject {
    fn from(values: Vec<Value>) -> Self {
        Self(ArrayObject::from(values).into())
    }
}

impl From<Map<String, Value>> for ValueTypeObject {
    fn from(values: Map<String, Value>) -> Self {
        Self(ObjectObject::from(values).into())
    }
}

impl From<NullObject> for ValueTypeObject {
    fn from(inner: NullObject) -> Self {
        Self(inner.into())
    }
}

impl From<ArrayObject> for ValueTypeObject {
    fn from(inner: ArrayObject) -> Self {
        Self(inner.into())
    }
}

impl From<BooleanObject> for ValueTypeObject {
    fn from(inner: BooleanObject) -> Self {
        Self(inner.into())
    }
}

impl From<StringObject> for ValueTypeObject {
    fn from(inner: StringObject) -> Self {
        Self(inner.into())
    }
}

impl From<FloatObject> for ValueTypeObject {
    fn from(inner: FloatObject) -> Self {
        Self(inner.into())
    }
}

impl From<IntegerObject> for ValueTypeObject {
    fn from(inner: IntegerObject) -> Self {
        Self(inner.into())
    }
}

impl From<ObjectObject> for ValueTypeObject {
    fn from(inner: ObjectObject) -> Self {
        Self(inner.into())
    }
}

impl From<ValueObject> for ValueTypeObject {
    fn from(inner: ValueObject) -> Self {
        Self(inner)
    }
}

impl Display for ValueTypeObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            ValueObject::Null(inner) => write!(formatter, "{inner}"),
            ValueObject::String(inner) => write!(formatter, "string {inner}"),
            ValueObject::Float(inner) => write!(formatter, "float {inner}"),
            ValueObject::Integer(inner) => write!(formatter, "integer {inner}"),
            ValueObject::Boolean(inner) => write!(formatter, "boolean {inner}"),
            ValueObject::Array(inner) => write!(formatter, "array {inner}"),
            ValueObject::Object(inner) => write!(formatter, "object {inner}"),
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
        let output = ValueTypeObject::from(json!(null));
        assert_eq!(output, ValueTypeObject::from(NullObject));
    }

    #[test]
    fn it_should_convert_from_json_boolean() {
        let output = ValueTypeObject::from(json!(true));
        assert_eq!(output, ValueTypeObject::from(BooleanObject::from(true)));

        let output = ValueTypeObject::from(json!(false));
        assert_eq!(output, ValueTypeObject::from(BooleanObject::from(false)));
    }

    #[test]
    fn it_should_convert_from_json_floats() {
        let output = ValueTypeObject::from(json!(0.0));
        assert_eq!(output, ValueTypeObject::from(FloatObject::from(0.0)));

        let output = ValueTypeObject::from(json!(123.456));
        assert_eq!(output, ValueTypeObject::from(FloatObject::from(123.456)));

        let output = ValueTypeObject::from(json!(f64::MAX));
        assert_eq!(output, ValueTypeObject::from(FloatObject::from(f64::MAX)));

        let output = ValueTypeObject::from(json!(f64::MIN));
        assert_eq!(output, ValueTypeObject::from(FloatObject::from(f64::MIN)));

        let output = ValueTypeObject::from(json!(f64::consts::PI));
        assert_eq!(
            output,
            ValueTypeObject::from(FloatObject::from(f64::consts::PI))
        );
    }

    #[test]
    fn it_should_convert_from_json_integers() {
        let output = ValueTypeObject::from(json!(0));
        assert_eq!(output, ValueTypeObject::from(IntegerObject::from(0_u64)));

        let output = ValueTypeObject::from(json!(123));
        assert_eq!(output, ValueTypeObject::from(IntegerObject::from(123_u64)));

        let output = ValueTypeObject::from(json!(u64::MAX));
        assert_eq!(output, ValueTypeObject::from(IntegerObject::from(u64::MAX)));

        let output = ValueTypeObject::from(json!(u64::MIN));
        assert_eq!(output, ValueTypeObject::from(IntegerObject::from(u64::MIN)));

        let output = ValueTypeObject::from(json!(i64::MAX));
        assert_eq!(
            output,
            ValueTypeObject::from(IntegerObject::from(i64::MAX as u64))
        );

        let output = ValueTypeObject::from(json!(i64::MIN));
        assert_eq!(output, ValueTypeObject::from(IntegerObject::from(i64::MIN)));
    }

    #[test]
    fn it_should_convert_from_json_strings() {
        let output = ValueTypeObject::from(json!(""));
        assert_eq!(output, ValueTypeObject::from(StringObject::from("")));

        let output = ValueTypeObject::from(json!("abc123"));
        assert_eq!(output, ValueTypeObject::from(StringObject::from("abc123")));
    }

    #[test]
    fn it_should_convert_from_json_arrays() {
        let output = ValueTypeObject::from(json!([]));
        assert_eq!(output, ValueTypeObject::from(ArrayObject::from(empty())));

        let output = ValueTypeObject::from(json!([0, 123.456, "something"]));
        assert_eq!(
            output,
            ValueTypeObject::from(ArrayObject::from([
                json!(0),
                json!(123.456),
                json!("something")
            ]))
        );
    }

    #[test]
    fn it_should_convert_from_json_objects() {
        let output = ValueTypeObject::from(json!({}));
        assert_eq!(
            output,
            ValueTypeObject::from(ObjectObject::from_iter(empty()))
        );

        let output = ValueTypeObject::from(json!({
            "age": 30,
            "name": "Joe",
            "ids": [1, 2, 3],
        }));
        assert_eq!(
            output,
            ValueTypeObject::from(ObjectObject::from_iter([
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
        let output = format!("{}", ValueTypeObject::from(NullObject));
        assert_eq!(output, "null");
    }

    #[test]
    fn it_should_display_boolean() {
        let output = format!("{}", ValueTypeObject::from(BooleanObject::from(true)));
        assert_eq!(output, "boolean true");

        let output = format!("{}", ValueTypeObject::from(BooleanObject::from(false)));
        assert_eq!(output, "boolean false");
    }

    #[test]
    fn it_should_display_integer_type() {
        let output = format!("{}", ValueTypeObject::from(IntegerObject::from(0_u64)));
        assert_eq!(output, "integer 0");

        let output = format!("{}", ValueTypeObject::from(IntegerObject::from(123_u64)));
        assert_eq!(output, "integer 123");

        let output = format!("{}", ValueTypeObject::from(IntegerObject::from(0_i64)));
        assert_eq!(output, "integer 0");

        let output = format!("{}", ValueTypeObject::from(IntegerObject::from(123_i64)));
        assert_eq!(output, "integer 123");

        let output = format!("{}", ValueTypeObject::from(IntegerObject::from(-123_i64)));
        assert_eq!(output, "integer -123");
    }

    #[test]
    fn it_should_display_float_type() {
        let output = format!("{}", ValueTypeObject::from(FloatObject::from(0.0)));
        assert_eq!(output, "float 0.0");

        let output = format!("{}", ValueTypeObject::from(FloatObject::from(123.456)));
        assert_eq!(output, "float 123.456");

        let output = format!("{}", ValueTypeObject::from(FloatObject::from(-123.456)));
        assert_eq!(output, "float -123.456");
    }

    #[test]
    fn it_should_display_string_type() {
        let output = format!("{}", ValueTypeObject::from(StringObject::from("")));
        assert_eq!(output, r#"string """#);

        let output = format!("{}", ValueTypeObject::from(StringObject::from("something")));
        assert_eq!(output, r#"string "something""#);
    }

    #[test]
    fn it_should_display_array_type() {
        let output = format!("{}", ValueTypeObject::from(ArrayObject::from(empty())));
        assert_eq!(output, "array []");
    }

    #[test]
    fn it_should_display_object_type() {
        let obj = Map::from_iter(empty());
        let output = format!("{}", ValueTypeObject::from(ObjectObject::from(obj)));
        assert_eq!(output, "object {}");
    }
}
