use crate::internals::pretty_formatter::PrettyDisplay;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FloatObject(f64);

impl From<f64> for FloatObject {
    fn from(num: f64) -> Self {
        Self(num)
    }
}

impl From<FloatObject> for f64 {
    fn from(obj: FloatObject) -> Self {
        obj.0
    }
}

impl Display for FloatObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let n_str = format_f64_with_minimum_one_precision(self.0);
        write!(formatter, "{n_str}")
    }
}

impl PrettyDisplay for FloatObject {}

fn format_f64_with_minimum_one_precision(n: f64) -> String {
    let float_str = format!("{n}");
    if !float_str.contains(".") {
        format!("{n:.min_precision$}", min_precision = 1)
    } else {
        float_str
    }
}
