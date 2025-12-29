use crate::internals::objects::ValueObject;
use crate::internals::pretty_formatter::pretty_formatter::PrettyFormatter;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Result as FmtResult;

pub trait PrettyDisplay: Display {
    fn pretty_fmt(&self, formatter: &mut PrettyFormatter<'_, '_>) -> FmtResult {
        self.fmt(formatter)
    }

    fn is_indenting(&self) -> bool {
        false
    }
}

impl PrettyDisplay for Value {
    fn pretty_fmt(&self, formatter: &mut PrettyFormatter<'_, '_>) -> FmtResult {
        match self {
            Value::Bool(inner) => write!(formatter, "{inner}"),
            Value::String(inner) => write!(formatter, r#""{inner}""#),
            Value::Null => write!(formatter, "null"),
            Value::Number(inner) => {
                let num_obj = ValueObject::from(self.clone());
                match num_obj {
                    ValueObject::Float(inner) => inner.pretty_fmt(formatter),
                    ValueObject::Integer(inner) => inner.pretty_fmt(formatter),
                    _ => panic!(
                        "Unexpected non-number value, expected a float or an integer, found {inner:?}. (This is a bug, please report at: https://github.com/JosephLenton/expect-json/issues)"
                    ),
                }
            }
            Value::Array(inner) => formatter.write_fmt_array(inner),
            Value::Object(inner) => formatter.write_fmt_object(inner),
        }
    }

    fn is_indenting(&self) -> bool {
        matches!(self, Self::Array(_) | Self::Object(_))
    }
}
