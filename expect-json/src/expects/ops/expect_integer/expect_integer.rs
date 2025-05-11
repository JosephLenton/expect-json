use crate::expect_op;
use crate::ops::expect_integer::ExpectIntegerSubOp;
use crate::Context;
use crate::ExpectOp;
use crate::ExpectOpResult;
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
            r#"Json integer at root is not zero:
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
            r#"Json integer at root is not zero:
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
            r#"Json integer at root is not zero:
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
            r#"Json integer at root is not zero:
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
            r#"Json integer at root is not zero:
    expected 0
    received 18446744073709551615"#
        );
    }
}
