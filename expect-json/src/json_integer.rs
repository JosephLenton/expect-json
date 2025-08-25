use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

/// Json integers can be i64 or u64, which cover different ranges.
/// This is a type representing those numbers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonInteger {
    Positive(u64),
    Negative(i64),
}

impl JsonInteger {
    pub(crate) fn gt<O>(self, other: O) -> bool
    where
        O: Into<JsonInteger>,
    {
        let other_json = other.into();
        match (self, other_json) {
            (Self::Positive(l), Self::Positive(r)) => l > r,
            (Self::Positive(l), Self::Negative(r)) => {
                if r < 0 {
                    return true;
                }

                l > (r as u64)
            }
            (Self::Negative(l), Self::Negative(r)) => l > r,
            (Self::Negative(l), Self::Positive(r)) => {
                if l < 0 {
                    return false;
                }

                (l as u64) > r
            }
        }
    }

    pub(crate) fn ge<O>(self, other: O) -> bool
    where
        O: Into<JsonInteger>,
    {
        let other_json = other.into();
        match (self, other_json) {
            (Self::Positive(l), Self::Positive(r)) => l >= r,
            (Self::Positive(l), Self::Negative(r)) => {
                if r < 0 {
                    return true;
                }

                l >= (r as u64)
            }
            (Self::Negative(l), Self::Negative(r)) => l >= r,
            (Self::Negative(l), Self::Positive(r)) => {
                if l < 0 {
                    return false;
                }

                (l as u64) >= r
            }
        }
    }

    pub(crate) fn lt<O>(self, other: O) -> bool
    where
        O: Into<JsonInteger>,
    {
        let other_json = other.into();
        match (self, other_json) {
            (Self::Positive(l), Self::Positive(r)) => l < r,
            (Self::Positive(l), Self::Negative(r)) => {
                if r < 0 {
                    return false;
                }

                l < (r as u64)
            }
            (Self::Negative(l), Self::Negative(r)) => l < r,
            (Self::Negative(l), Self::Positive(r)) => {
                if l < 0 {
                    return true;
                }

                (l as u64) < r
            }
        }
    }

    pub(crate) fn le<O>(self, other: O) -> bool
    where
        O: Into<JsonInteger>,
    {
        let other_json = other.into();
        match (self, other_json) {
            (Self::Positive(l), Self::Positive(r)) => l <= r,
            (Self::Positive(l), Self::Negative(r)) => {
                if r < 0 {
                    return false;
                }

                l <= (r as u64)
            }
            (Self::Negative(l), Self::Negative(r)) => l <= r,
            (Self::Negative(l), Self::Positive(r)) => {
                if l < 0 {
                    return true;
                }

                (l as u64) <= r
            }
        }
    }
}

impl Display for JsonInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::Positive(n) => write!(f, "{n}"),
            Self::Negative(n) => write!(f, "{n}"),
        }
    }
}

impl From<u8> for JsonInteger {
    fn from(n: u8) -> Self {
        Self::Positive(n as u64)
    }
}

impl From<i8> for JsonInteger {
    fn from(n: i8) -> Self {
        Self::Negative(n as i64)
    }
}

impl From<u16> for JsonInteger {
    fn from(n: u16) -> Self {
        Self::Positive(n as u64)
    }
}

impl From<i16> for JsonInteger {
    fn from(n: i16) -> Self {
        Self::Negative(n as i64)
    }
}

impl From<u32> for JsonInteger {
    fn from(n: u32) -> Self {
        Self::Positive(n as u64)
    }
}

impl From<i32> for JsonInteger {
    fn from(n: i32) -> Self {
        Self::Negative(n as i64)
    }
}

impl From<u64> for JsonInteger {
    fn from(n: u64) -> Self {
        Self::Positive(n)
    }
}

impl From<i64> for JsonInteger {
    fn from(n: i64) -> Self {
        Self::Negative(n)
    }
}

impl From<usize> for JsonInteger {
    fn from(n: usize) -> Self {
        Self::Positive(n as u64)
    }
}

impl From<isize> for JsonInteger {
    fn from(n: isize) -> Self {
        Self::Negative(n as i64)
    }
}

#[cfg(test)]
mod test_from {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_contain_value_from_i8() {
        let output = JsonInteger::from(-123_i8);
        assert_eq!(output, JsonInteger::Negative(-123));
    }

    #[test]
    fn it_should_contain_value_from_u8() {
        let output = JsonInteger::from(123_u8);
        assert_eq!(output, JsonInteger::Positive(123));
    }

    #[test]
    fn it_should_contain_value_from_i16() {
        let output = JsonInteger::from(-123_i16);
        assert_eq!(output, JsonInteger::Negative(-123));
    }

    #[test]
    fn it_should_contain_value_from_u16() {
        let output = JsonInteger::from(123_u16);
        assert_eq!(output, JsonInteger::Positive(123));
    }

    #[test]
    fn it_should_contain_value_from_i32() {
        let output = JsonInteger::from(-123_i32);
        assert_eq!(output, JsonInteger::Negative(-123));
    }

    #[test]
    fn it_should_contain_value_from_u32() {
        let output = JsonInteger::from(123_u32);
        assert_eq!(output, JsonInteger::Positive(123));
    }

    #[test]
    fn it_should_contain_value_from_i64() {
        let output = JsonInteger::from(-123_i64);
        assert_eq!(output, JsonInteger::Negative(-123));
    }

    #[test]
    fn it_should_contain_value_from_u64() {
        let output = JsonInteger::from(123_u64);
        assert_eq!(output, JsonInteger::Positive(123));
    }

    #[test]
    fn it_should_contain_value_from_isize() {
        let output = JsonInteger::from(-123_isize);
        assert_eq!(output, JsonInteger::Negative(-123));
    }

    #[test]
    fn it_should_contain_value_from_usize() {
        let output = JsonInteger::from(123_usize);
        assert_eq!(output, JsonInteger::Positive(123));
    }
}

#[cfg(test)]
mod test_fmt {
    use super::*;

    #[test]
    fn it_should_print_positive_numbers_as_themselves() {
        let num = JsonInteger::Positive(123);
        let output = num.to_string();

        assert_eq!(output, "123")
    }

    #[test]
    fn it_should_print_negative_numbers_as_themselves() {
        let num = JsonInteger::Negative(-123);
        let output = num.to_string();

        assert_eq!(output, "-123")
    }
}

#[cfg(test)]
mod test_lt {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(101, 100, false)]
    #[case(100, 100, false)]
    #[case(99, 100, true)]
    fn it_should_handle_u64_to_u64_correctly(
        #[case] l_num: u64,
        #[case] r_num: u64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.lt(r),
            expected,
            "{l_num} < {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, false)]
    #[case(100, 100, false)]
    #[case(99, 100, true)]
    #[case(-101, -100, true)]
    #[case(-100, -100, false)]
    #[case(-99, -100, false)]
    fn it_should_handle_i64_to_i64_correctly(
        #[case] l_num: i64,
        #[case] r_num: i64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.lt(r),
            expected,
            "{l_num} < {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, false)]
    #[case(100, 100, false)]
    #[case(99, 100, true)]
    #[case(0, 0, false)]
    #[case(-0, 0, false)]
    #[case(-1, 0, true)]
    fn it_should_handle_i64_to_u64_correctly(
        #[case] l_num: i64,
        #[case] r_num: u64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.lt(r),
            expected,
            "{l_num} < {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, false)]
    #[case(100, 100, false)]
    #[case(99, 100, true)]
    #[case(0, 0, false)]
    #[case(0, -0, false)]
    #[case(0, -1, false)]
    fn it_should_handle_u64_to_i64_correctly(
        #[case] l_num: u64,
        #[case] r_num: i64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.lt(r),
            expected,
            "{l_num} < {r_num} was expected to be {expected}"
        );
    }
}

#[cfg(test)]
mod test_le {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(101, 100, false)]
    #[case(100, 100, true)]
    #[case(99, 100, true)]
    fn it_should_handle_u64_to_u64_correctly(
        #[case] l_num: u64,
        #[case] r_num: u64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.le(r),
            expected,
            "{l_num} <= {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, false)]
    #[case(100, 100, true)]
    #[case(99, 100, true)]
    #[case(-101, -100, true)]
    #[case(-100, -100, true)]
    #[case(-99, -100, false)]
    fn it_should_handle_i64_to_i64_correctly(
        #[case] l_num: i64,
        #[case] r_num: i64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.le(r),
            expected,
            "{l_num} <= {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, false)]
    #[case(100, 100, true)]
    #[case(99, 100, true)]
    #[case(0, 0, true)]
    #[case(-0, 0, true)]
    #[case(-1, 0, true)]
    fn it_should_handle_i64_to_u64_correctly(
        #[case] l_num: i64,
        #[case] r_num: u64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.le(r),
            expected,
            "{l_num} <= {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, false)]
    #[case(100, 100, true)]
    #[case(99, 100, true)]
    #[case(0, 0, true)]
    #[case(0, -0, true)]
    #[case(0, -1, false)]
    fn it_should_handle_u64_to_i64_correctly(
        #[case] l_num: u64,
        #[case] r_num: i64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.le(r),
            expected,
            "{l_num} <= {r_num} was expected to be {expected}"
        );
    }
}

#[cfg(test)]
mod test_gt {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(101, 100, true)]
    #[case(100, 100, false)]
    #[case(99, 100, false)]
    fn it_should_handle_u64_to_u64_correctly(
        #[case] l_num: u64,
        #[case] r_num: u64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.gt(r),
            expected,
            "{l_num} > {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, true)]
    #[case(100, 100, false)]
    #[case(99, 100, false)]
    #[case(-101, -100, false)]
    #[case(-100, -100, false)]
    #[case(-99, -100, true)]
    fn it_should_handle_i64_to_i64_correctly(
        #[case] l_num: i64,
        #[case] r_num: i64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.gt(r),
            expected,
            "{l_num} > {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, true)]
    #[case(100, 100, false)]
    #[case(99, 100, false)]
    #[case(0, 0, false)]
    #[case(-0, 0, false)]
    #[case(-1, 0, false)]
    fn it_should_handle_i64_to_u64_correctly(
        #[case] l_num: i64,
        #[case] r_num: u64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.gt(r),
            expected,
            "{l_num} > {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, true)]
    #[case(100, 100, false)]
    #[case(99, 100, false)]
    #[case(0, 0, false)]
    #[case(0, -0, false)]
    #[case(0, -1, true)]
    fn it_should_handle_u64_to_i64_correctly(
        #[case] l_num: u64,
        #[case] r_num: i64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.gt(r),
            expected,
            "{l_num} > {r_num} was expected to be {expected}"
        );
    }
}

#[cfg(test)]
mod test_ge {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(101, 100, true)]
    #[case(100, 100, true)]
    #[case(99, 100, false)]
    fn it_should_handle_u64_to_u64_correctly(
        #[case] l_num: u64,
        #[case] r_num: u64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.ge(r),
            expected,
            "{l_num} >= {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, true)]
    #[case(100, 100, true)]
    #[case(99, 100, false)]
    #[case(-101, -100, false)]
    #[case(-100, -100, true)]
    #[case(-99, -100, true)]
    fn it_should_handle_i64_to_i64_correctly(
        #[case] l_num: i64,
        #[case] r_num: i64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.ge(r),
            expected,
            "{l_num} >= {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, true)]
    #[case(100, 100, true)]
    #[case(99, 100, false)]
    #[case(0, 0, true)]
    #[case(-0, 0, true)]
    #[case(-1, 0, false)]
    fn it_should_handle_i64_to_u64_correctly(
        #[case] l_num: i64,
        #[case] r_num: u64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.ge(r),
            expected,
            "{l_num} >= {r_num} was expected to be {expected}"
        );
    }

    #[rstest]
    #[case(101, 100, true)]
    #[case(100, 100, true)]
    #[case(99, 100, false)]
    #[case(0, 0, true)]
    #[case(0, -0, true)]
    #[case(0, -1, true)]
    fn it_should_handle_u64_to_i64_correctly(
        #[case] l_num: u64,
        #[case] r_num: i64,
        #[case] expected: bool,
    ) {
        let l = JsonInteger::from(l_num);
        let r = JsonInteger::from(r_num);
        assert_eq!(
            l.ge(r),
            expected,
            "{l_num} >= {r_num} was expected to be {expected}"
        );
    }
}
