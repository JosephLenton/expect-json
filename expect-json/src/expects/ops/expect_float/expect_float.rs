use crate::expect_op;
use crate::ops::expect_float::ExpectFloatSubOp;
use crate::Context;
use crate::ExpectOp;
use crate::ExpectOpResult;
use crate::JsonType;
use core::ops::RangeBounds;

#[expect_op(internal, name = "float")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpectFloat {
    sub_ops: Vec<ExpectFloatSubOp>,
}

impl ExpectFloat {
    pub(crate) fn new() -> Self {
        Self { sub_ops: vec![] }
    }

    pub fn is_in_range<R>(mut self, range: R) -> Self
    where
        R: RangeBounds<f64>,
    {
        let min = range.start_bound().cloned();
        let max = range.end_bound().cloned();

        self.sub_ops.push(ExpectFloatSubOp::InRange {
            min: min.into(),
            max: max.into(),
        });
        self
    }

    pub fn is_zero(mut self) -> Self {
        self.sub_ops.push(ExpectFloatSubOp::Zero);
        self
    }

    pub fn is_not_zero(mut self) -> Self {
        self.sub_ops.push(ExpectFloatSubOp::NotZero);
        self
    }

    pub fn is_positive(mut self) -> Self {
        self.sub_ops.push(ExpectFloatSubOp::Positive);
        self
    }

    pub fn is_negative(mut self) -> Self {
        self.sub_ops.push(ExpectFloatSubOp::Negative);
        self
    }
}

impl ExpectOp for ExpectFloat {
    fn on_f64(&self, context: &mut Context, received: f64) -> ExpectOpResult<()> {
        for sub_op in &self.sub_ops {
            sub_op.on_f64(self, context, received)?;
        }

        Ok(())
    }

    fn supported_types(&self) -> &'static [JsonType] {
        &[JsonType::Float]
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
        let left = json!(0.0);
        let right = json!(expect::float().is_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_false_for_negative_value() {
        let left = json!(-1.0);
        let right = json!(expect::float().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json float at root is not zero:
    expected 0.0
    received -1.0"#
        );
    }

    #[test]
    fn it_should_be_false_for_positive_value() {
        let left = json!(1.0);
        let right = json!(expect::float().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json float at root is not zero:
    expected 0.0
    received 1.0"#
        );
    }

    #[test]
    fn it_should_be_false_for_min() {
        let left = json!(f64::MIN);
        let right = json!(expect::float().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json float at root is not zero:
    expected 0.0
    received -179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368.0"#
        );
    }

    #[test]
    fn it_should_be_false_for_max() {
        let left = json!(f64::MAX);
        let right = json!(expect::float().is_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json float at root is not zero:
    expected 0.0
    received 179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368.0"#
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
        let left = json!(0.0);
        let right = json!(expect::float().is_not_zero());

        let output = expect_json_eq(&left, &right).unwrap_err().to_string();
        assert_eq!(
            output,
            r#"Json float at root is zero:
    expected non-zero float
    received 0.0"#
        );
    }

    #[test]
    fn it_should_be_true_for_negative_value() {
        let left = json!(-1.0);
        let right = json!(expect::float().is_not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }

    #[test]
    fn it_should_be_true_for_positive_value() {
        let left = json!(1.0);
        let right = json!(expect::float().is_not_zero());

        let output = expect_json_eq(&left, &right);
        assert!(output.is_ok());
    }
}
