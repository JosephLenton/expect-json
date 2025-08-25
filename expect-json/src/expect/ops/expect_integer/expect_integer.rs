use crate::expect::ops::expect_integer::ExpectIntegerSubOp;
use crate::expect_core::expect_op;
use crate::expect_core::Context;
use crate::expect_core::ExpectOp;
use crate::expect_core::ExpectOpResult;
use crate::JsonInteger;
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

    pub fn greater_than<N>(mut self, expected: N) -> Self
    where
        N: Into<JsonInteger>,
    {
        self.sub_ops.push(ExpectIntegerSubOp::GreaterThan {
            expected: expected.into(),
        });
        self
    }

    pub fn greater_than_equal<N>(mut self, expected: N) -> Self
    where
        N: Into<JsonInteger>,
    {
        self.sub_ops.push(ExpectIntegerSubOp::GreaterThanEqual {
            expected: expected.into(),
        });
        self
    }

    pub fn less_than<N>(mut self, expected: N) -> Self
    where
        N: Into<JsonInteger>,
    {
        self.sub_ops.push(ExpectIntegerSubOp::LessThan {
            expected: expected.into(),
        });
        self
    }

    pub fn less_than_equal<N>(mut self, expected: N) -> Self
    where
        N: Into<JsonInteger>,
    {
        self.sub_ops.push(ExpectIntegerSubOp::LessThanEqual {
            expected: expected.into(),
        });
        self
    }

    ///
    /// Expect an integer within the given range.
    ///
    /// ```rust
    /// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
    /// #
    /// # use axum::Router;
    /// # use axum::extract::Json;
    /// # use axum::routing::get;
    /// # use axum_test::TestServer;
    /// # use serde_json::json;
    /// #
    /// # let server = TestServer::new(Router::new())?;
    /// #
    /// use axum_test::expect_json;
    ///
    /// let server = TestServer::new(Router::new())?;
    ///
    /// server.get(&"/user/barrington")
    ///     .await
    ///     .assert_json(&json!({
    ///         "name": "Barrington",
    ///         "age": expect_json::integer().in_range(18..=110),
    ///     }));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn in_range<R>(mut self, range: R) -> Self
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

    pub fn outside_range<R>(mut self, range: R) -> Self
    where
        R: RangeBounds<i64>,
    {
        let min = range.start_bound().cloned();
        let max = range.end_bound().cloned();

        self.sub_ops.push(ExpectIntegerSubOp::OutsideRange {
            min: min.into(),
            max: max.into(),
        });

        self
    }

    pub fn zero(mut self) -> Self {
        self.sub_ops.push(ExpectIntegerSubOp::Zero);
        self
    }

    pub fn not_zero(mut self) -> Self {
        self.sub_ops.push(ExpectIntegerSubOp::NotZero);
        self
    }

    pub fn positive(mut self) -> Self {
        self.sub_ops.push(ExpectIntegerSubOp::Positive);
        self
    }

    pub fn negative(mut self) -> Self {
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

    fn debug_supported_types(&self) -> &'static [JsonType] {
        &[JsonType::Integer]
    }
}

#[cfg(test)]
mod test_in_range {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_all_values_in_total_range() {
        let left = json!(1);
        let right = json!(expect::integer().in_range(..));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(i64::MIN);
        let right = json!(expect::integer().in_range(..));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(u64::MAX);
        let right = json!(expect::integer().in_range(..));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_all_values_in_partial_range() {
        let left = json!(0);
        let right = json!(expect::integer().in_range(-10..10));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(-10);
        let right = json!(expect::integer().in_range(-10..10));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(5);
        let right = json!(expect::integer().in_range(-10..10));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_all_values_out_of_range() {
        let left = json!(1);
        let right = json!(expect::integer().in_range(0..1));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not in range
    expected 0..1
    received 1"#
        );

        let left = json!(-11);
        let right = json!(expect::integer().in_range(0..1));

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
        let left = json!(1);
        let right = json!(expect::integer().in_range(0..=1));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_positive_value_with_negative_min() {
        let left = json!(5);
        let right = json!(expect::integer().in_range(-10..10));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_positive_value_outside_range_with_negative_min() {
        let left = json!(11);
        let right = json!(expect::integer().in_range(-10..10));

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
        let right = json!(expect::integer().in_range(-10..-1));

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
mod test_outside_range {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_false_for_all_values_in_total_range() {
        let left = json!(1);
        let right = json!(expect::integer().outside_range(..));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is in range
    expected ..
    received 1"#
        );

        let left = json!(i64::MIN);
        let right = json!(expect::integer().outside_range(..));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is in range
    expected ..
    received -9223372036854775808"#
        );

        let left = json!(u64::MAX);
        let right = json!(expect::integer().outside_range(..));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is in range
    expected ..
    received 18446744073709551615"#
        );
    }

    #[test]
    fn it_should_be_false_for_all_values_overlapping_partial_ranges() {
        let left = json!(0);
        let right = json!(expect::integer().outside_range(-10..10));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is in range
    expected -10..10
    received 0"#
        );

        let left = json!(-10);
        let right = json!(expect::integer().outside_range(-10..10));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is in range
    expected -10..10
    received -10"#
        );

        let left = json!(5);
        let right = json!(expect::integer().outside_range(-10..10));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is in range
    expected -10..10
    received 5"#
        );
    }

    #[test]
    fn it_should_be_true_for_all_values_out_of_range() {
        let left = json!(1);
        let right = json!(expect::integer().outside_range(0..1));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());

        let left = json!(-11);
        let right = json!(expect::integer().outside_range(0..1));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_value_in_inclusive_range() {
        let left = json!(1);
        let right = json!(expect::integer().outside_range(0..=1));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is in range
    expected 0..=1
    received 1"#
        );
    }

    #[test]
    fn it_should_be_false_for_positive_value_with_negative_min() {
        let left = json!(5);
        let right = json!(expect::integer().outside_range(-10..10));

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is in range
    expected -10..10
    received 5"#
        );
    }

    #[test]
    fn it_should_be_true_for_positive_value_outside_range_with_negative_min() {
        let left = json!(11);
        let right = json!(expect::integer().outside_range(-10..10));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_positive_value_outside_range_with_negative_range() {
        let left = json!(11);
        let right = json!(expect::integer().outside_range(-10..-1));

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}

#[cfg(test)]
mod test_zero {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_zero() {
        let left = json!(0);
        let right = json!(expect::integer().zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_negative_value() {
        let left = json!(-1);
        let right = json!(expect::integer().zero());

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
        let right = json!(expect::integer().zero());

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
        let right = json!(expect::integer().zero());

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
        let right = json!(expect::integer().zero());

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
        let right = json!(expect::integer().zero());

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
mod test_not_zero {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_false_for_zero() {
        let left = json!(0);
        let right = json!(expect::integer().not_zero());

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
        let right = json!(expect::integer().not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_negative_max() {
        let left = json!(i64::MIN);
        let right = json!(expect::integer().not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_positive_value() {
        let left = json!(1);
        let right = json!(expect::integer().not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_i64_max() {
        let left = json!(i64::MAX);
        let right = json!(expect::integer().not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_u64_max() {
        let left = json!(u64::MAX);
        let right = json!(expect::integer().not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}

#[cfg(test)]
mod test_positive {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_zero() {
        let left = json!(0);
        let right = json!(expect::integer().positive());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_negative_i64() {
        let left = json!(-1);
        let right = json!(expect::integer().positive());

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
        let right = json!(expect::integer().positive());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_positive_u64() {
        let left = json!(123_u64);
        let right = json!(expect::integer().positive());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}

#[cfg(test)]
mod test_negative {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_false_for_zero() {
        let left = json!(0);
        let right = json!(expect::integer().negative());

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
        let right = json!(expect::integer().negative());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_positive_i64() {
        let left = json!(123_i64);
        let right = json!(expect::integer().negative());

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
        let right = json!(expect::integer().negative());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json expect::integer() error at root:
    integer is not negative
    received 123"#
        );
    }
}

#[cfg(test)]
mod test_less_than {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_correct_positive_comparison() {
        let left = json!(100_u64);
        let right = json!(expect::integer().less_than(101));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_false_for_equal_values() {
        let left = json!(100_u64);
        let right = json!(expect::integer().less_than(100));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            "Json expect::integer() error at root:
    integer is out of bounds,
    expected less than 100
    received 100"
        );
    }

    #[test]
    fn it_should_be_false_for_incorrect_positive_comparison() {
        let left = json!(101_u64);
        let right = json!(expect::integer().less_than(100));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            "Json expect::integer() error at root:
    integer is out of bounds,
    expected less than 100
    received 101"
        );
    }

    #[test]
    fn it_should_be_true_for_correct_negative_positive_mix() {
        let left = json!(-1);
        let right = json!(expect::integer().less_than(0));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_true_for_correct_negative_comparison() {
        let left = json!(-100);
        let right = json!(expect::integer().less_than(-99));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }
}

#[cfg(test)]
mod test_less_than_equal {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_correct_positive_comparison() {
        let left = json!(100_u64);
        let right = json!(expect::integer().less_than_equal(101));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_true_for_equal_values() {
        let left = json!(100_u64);
        let right = json!(expect::integer().less_than_equal(100));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_false_for_incorrect_positive_comparison() {
        let left = json!(101_u64);
        let right = json!(expect::integer().less_than_equal(100));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            "Json expect::integer() error at root:
    integer is out of bounds,
    expected less than equal 100
    received 101"
        );
    }

    #[test]
    fn it_should_be_true_for_correct_negative_positive_mix() {
        let left = json!(-1);
        let right = json!(expect::integer().less_than_equal(0));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_true_for_correct_negative_comparison() {
        let left = json!(-100);
        let right = json!(expect::integer().less_than_equal(-99));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }
}

#[cfg(test)]
mod test_greater_than {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_correct_positive_comparison() {
        let left = json!(101_u64);
        let right = json!(expect::integer().greater_than(100));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_false_for_equal_values() {
        let left = json!(100_u64);
        let right = json!(expect::integer().greater_than(100));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            "Json expect::integer() error at root:
    integer is out of bounds,
    expected greater than 100
    received 100"
        );
    }

    #[test]
    fn it_should_be_false_for_incorrect_positive_comparison() {
        let left = json!(100_u64);
        let right = json!(expect::integer().greater_than(101));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            "Json expect::integer() error at root:
    integer is out of bounds,
    expected greater than 101
    received 100"
        );
    }

    #[test]
    fn it_should_be_true_for_correct_negative_positive_mix() {
        let left = json!(0);
        let right = json!(expect::integer().greater_than(-1));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_true_for_correct_negative_comparison() {
        let left = json!(-99);
        let right = json!(expect::integer().greater_than(-100));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }
}

#[cfg(test)]
mod test_greater_than_equal {
    use crate::expect;
    use crate::expect_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_be_true_for_correct_positive_comparison() {
        let left = json!(101_u64);
        let right = json!(expect::integer().greater_than_equal(100));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_true_for_equal_values() {
        let left = json!(100_u64);
        let right = json!(expect::integer().greater_than_equal(100));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_false_for_incorrect_positive_comparison() {
        let left = json!(100_u64);
        let right = json!(expect::integer().greater_than_equal(101));
        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            "Json expect::integer() error at root:
    integer is out of bounds,
    expected greater than equal 101
    received 100"
        );
    }

    #[test]
    fn it_should_be_true_for_correct_negative_positive_mix() {
        let left = json!(0);
        let right = json!(expect::integer().greater_than_equal(-1));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }

    #[test]
    fn it_should_be_true_for_correct_negative_comparison() {
        let left = json!(-99);
        let right = json!(expect::integer().greater_than_equal(-100));
        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok(), "{output:#?}");
    }
}
