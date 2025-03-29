use serde_json::Number;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NumberType {
    Float(f64),
    PositiveInteger(u64),
    NegativeInteger(i64),
}

impl From<Number> for NumberType {
    fn from(json_number: Number) -> Self {
        Self::from(&json_number)
    }
}

impl From<&Number> for NumberType {
    fn from(value: &Number) -> Self {
        if value.is_f64() {
            let n = value
                .as_f64()
                .expect("Expected to convert serde_json::Number to f64");
            NumberType::Float(n)
        } else if value.is_u64() {
            let n = value
                .as_u64()
                .expect("Expected to convert serde_json::Number to u64");
            NumberType::PositiveInteger(n)
        } else {
            let n = value
                .as_i64()
                .expect("Expected to convert serde_json::Number to i64");
            NumberType::NegativeInteger(n)
        }
    }
}

impl Display for NumberType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Float(n) => {
                let n_str = format_f64_with_minimum_one_precision(*n);
                write!(formatter, "float {n_str}")
            }
            Self::PositiveInteger(n) => write!(formatter, "integer {}", *n),
            Self::NegativeInteger(n) => write!(formatter, "integer {}", *n),
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
