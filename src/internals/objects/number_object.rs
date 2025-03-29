use serde_json::Number;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NumberObject {
    Float(f64),
    PositiveInteger(u64),
    NegativeInteger(i64),
}

impl From<Number> for NumberObject {
    fn from(json_number: Number) -> Self {
        Self::from(&json_number)
    }
}

impl From<&Number> for NumberObject {
    fn from(value: &Number) -> Self {
        if value.is_f64() {
            let n = value
                .as_f64()
                .expect("Expected to convert serde_json::Number to f64");
            NumberObject::Float(n)
        } else if value.is_u64() {
            let n = value
                .as_u64()
                .expect("Expected to convert serde_json::Number to u64");
            NumberObject::PositiveInteger(n)
        } else {
            let n = value
                .as_i64()
                .expect("Expected to convert serde_json::Number to i64");
            NumberObject::NegativeInteger(n)
        }
    }
}

impl Display for NumberObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Float(n) => {
                let n_str = format_f64_with_minimum_one_precision(*n);
                write!(formatter, "{n_str}")
            }
            Self::PositiveInteger(n) => write!(formatter, "{}", *n),
            Self::NegativeInteger(n) => write!(formatter, "{}", *n),
        }
    }
}

fn format_f64_with_minimum_one_precision(n: f64) -> String {
    let float_str = format!("{n}");
    if !float_str.contains(".") {
        format!("{n:.min_precision$}", min_precision = 1)
    } else {
        float_str
    }
}
