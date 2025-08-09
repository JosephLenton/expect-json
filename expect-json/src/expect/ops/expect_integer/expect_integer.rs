use crate::expect::ops::expect_integer::ExpectIntegerSubOp;
use crate::expect_op;
use crate::expect_op::Context;
use crate::expect_op::ExpectOp;
use crate::expect_op::ExpectOpResult;
use crate::JsonType;
use core::ops::RangeBounds;

#[expect_op(internal, name = "integer")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectInteger {
    sub_ops: Vec<ExpectIntegerSubOp>,
}

impl ExpectInteger {
    pub(crate) fn new() -> Self {
        Self { sub_ops: vec![] }
    }

    pub fn is_in_range<R>(mut self, range: R) -> Self
    where
        R: RangeBounds<i64>,
    {
        let min = range.start_bound().cloned();
        let max = range.end_bound().cloned();

        self.sub_ops.push(ExpectIntegerSubOp::InRange {
            min: min.into(),
            max: max.into(),
        });
        self
    }

    pub fn is_zero(mut self) -> Self {
        self.sub_ops.push(ExpectIntegerSubOp::Zero);
        self
    }

    pub fn is_not_zero(mut self) -> Self {
        self.sub_ops.push(ExpectIntegerSubOp::NotZero);
        self
    }

    pub fn is_positive(mut self) -> Self {
        self.sub_ops.push(ExpectIntegerSubOp::Positive);
        self
    }

    pub fn is_negative(mut self) -> Self {
        self.sub_ops.push(ExpectIntegerSubOp::Negative);
        self
    }
}

impl ExpectOp for ExpectInteger {
    fn on_i64(&self, context: &mut Context, received: i64) -> ExpectOpResult<()> {
        for sub_op in &self.sub_ops {
            sub_op.on_i64(self, context, received)?;
        }

        Ok(())
    }

    fn on_u64(&self, context: &mut Context, received: u64) -> ExpectOpResult<()> {
        for sub_op in &self.sub_ops {
            sub_op.on_u64(self, context, received)?;
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::Integer]
    }
}

#[cfg(test)]
mod test_is_in_range {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_all_values_in_total_range() {
        let left = json!(1);
        let right = json!(expect::integer().is_in_range(..));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(i64::MIN);
        let right = json!(expect::integer().is_in_range(..));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(u64::MAX);
        let right = json!(expect::integer().is_in_range(..));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_all_values_in_partial_range() {
        let left = json!(0);
        let right = json!(expect::integer().is_in_range(-10..10));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(-10);
        let right = json!(expect::integer().is_in_range(-10..10));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(5);
        let right = json!(expect::integer().is_in_range(-10..10));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_all_values_out_of_range() {
        let left = json!(1);
        let right = json!(expect::integer().is_in_range(0..1));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not in range
    expected 0..1
    received 1"#
        );

        let left = json!(-11);
        let right = json!(expect::integer().is_in_range(0..1));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not in range
    expected 0..1
    received -11"#
        );
    }

    #[test]
    fn it_should_be_true_for_value_in_inclusive_range() {
        let left = json!(1.0);
        let right = json!(expect::float().is_in_range(0.0..=1.0));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_positive_value_with_negative_min() {
        let left = json!(5);
        let right = json!(expect::integer().is_in_range(-10..10));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_positive_value_outside_range_with_negative_min() {
        let left = json!(11);
        let right = json!(expect::integer().is_in_range(-10..10));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not in range
    expected -10..10
    received 11"#
        );
    }

    #[test]
    fn it_should_be_false_for_positive_value_outside_range_with_negative_range() {
        let left = json!(11);
        let right = json!(expect::integer().is_in_range(-10..-1));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not in range
    expected -10..-1
    received 11"#
        );
    }
}

#[cfg(test)]
mod test_is_zero {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_zero() {
        let left = json!(0);
        let right = json!(expect::integer().is_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_negative_value() {
        let left = json!(-1);
        let right = json!(expect::integer().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root, is not zero:
    expected 0
    received -1"#
        );
    }

    #[test]
    fn it_should_be_false_for_negative_max() {
        let left = json!(i64::MIN);
        let right = json!(expect::integer().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root, is not zero:
    expected 0
    received -9223372036854775808"#
        );
    }

    #[test]
    fn it_should_be_false_for_positive_value() {
        let left = json!(1);
        let right = json!(expect::integer().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root, is not zero:
    expected 0
    received 1"#
        );
    }

    #[test]
    fn it_should_be_false_for_i64_max() {
        let left = json!(i64::MAX);
        let right = json!(expect::integer().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root, is not zero:
    expected 0
    received 9223372036854775807"#
        );
    }

    #[test]
    fn it_should_be_false_for_u64_max() {
        let left = json!(u64::MAX);
        let right = json!(expect::integer().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root, is not zero:
    expected 0
    received 18446744073709551615"#
        );
    }
}

#[cfg(test)]
mod test_is_not_zero {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_false_for_zero() {
        let left = json!(0);
        let right = json!(expect::integer().is_not_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root, is zero:
    expected non-zero integer
    received 0"#
        );
    }

    #[test]
    fn it_should_be_true_for_negative_value() {
        let left = json!(-1);
        let right = json!(expect::integer().is_not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_negative_max() {
        let left = json!(i64::MIN);
        let right = json!(expect::integer().is_not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_positive_value() {
        let left = json!(1);
        let right = json!(expect::integer().is_not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_i64_max() {
        let left = json!(i64::MAX);
        let right = json!(expect::integer().is_not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_u64_max() {
        let left = json!(u64::MAX);
        let right = json!(expect::integer().is_not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}

#[cfg(test)]
mod test_is_positive {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_zero() {
        let left = json!(0);
        let right = json!(expect::integer().is_positive());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_negative_i64() {
        let left = json!(-1);
        let right = json!(expect::integer().is_positive());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not positive
    received -1"#
        );
    }

    #[test]
    fn it_should_be_true_for_positive_i64() {
        let left = json!(123_i64);
        let right = json!(expect::integer().is_positive());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_positive_u64() {
        let left = json!(123_u64);
        let right = json!(expect::integer().is_positive());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}

#[cfg(test)]
mod test_is_negative {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_false_for_zero() {
        let left = json!(0);
        let right = json!(expect::integer().is_negative());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not negative
    received 0"#
        );
    }

    #[test]
    fn it_should_be_true_for_negative_i64() {
        let left = json!(-1);
        let right = json!(expect::integer().is_negative());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_positive_i64() {
        let left = json!(123_i64);
        let right = json!(expect::integer().is_negative());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not negative
    received 123"#
        );
    }

    #[test]
    fn it_should_be_false_for_positive_u64() {
        let left = json!(123_u64);
        let right = json!(expect::integer().is_negative());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not negative
    received 123"#
        );
    }
}
