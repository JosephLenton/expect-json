use crate::JsonInteger;
use crate::expect::ops::ExpectInteger;
use crate::expect::ops::utils::SerializableBound;
use crate::expect::ops::utils::SerializableBoundContains;
use crate::expect_core::Context;
use crate::expect_core::ExpectOpError;
use crate::expect_core::ExpectOpResult;
use crate::internals::objects::IntegerObject;
use num::Zero;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectIntegerSubOp {
    InRange {
        min: SerializableBound<i64>,
        max: SerializableBound<i64>,
    },
    OutsideRange {
        min: SerializableBound<i64>,
        max: SerializableBound<i64>,
    },
    Zero,
    NotZero,
    Positive,
    Negative,

    GreaterThan {
        expected: JsonInteger,
    },
    GreaterThanEqual {
        expected: JsonInteger,
    },
    LessThan {
        expected: JsonInteger,
    },
    LessThanEqual {
        expected: JsonInteger,
    },
}

impl ExpectIntegerSubOp {
    pub(crate) fn on_i64(
        &self,
        parent: &ExpectInteger,
        context: &mut Context<'_>,
        received: i64,
    ) -> ExpectOpResult<()> {
        match *self {
            Self::InRange { min, max } => on_i64_in_range(parent, context, received, min, max),
            Self::OutsideRange { min, max } => {
                on_i64_outside_range(parent, context, received, min, max)
            }

            Self::Zero => on_zero(context, received),
            Self::NotZero => on_not_zero(context, received),

            Self::Positive => on_positive(parent, context, received),
            Self::Negative => on_negative(parent, context, received),

            Self::GreaterThan { expected: num } => on_comparison(
                parent,
                context,
                received.into(),
                num,
                JsonInteger::gt,
                "greater than",
            ),
            Self::GreaterThanEqual { expected: num } => on_comparison(
                parent,
                context,
                received.into(),
                num,
                JsonInteger::ge,
                "greater than equal",
            ),
            Self::LessThan { expected: num } => on_comparison(
                parent,
                context,
                received.into(),
                num,
                JsonInteger::lt,
                "less than",
            ),
            Self::LessThanEqual { expected: num } => on_comparison(
                parent,
                context,
                received.into(),
                num,
                JsonInteger::le,
                "less than equal",
            ),
        }
    }

    pub(crate) fn on_u64(
        &self,
        parent: &ExpectInteger,
        context: &mut Context<'_>,
        received: u64,
    ) -> ExpectOpResult<()> {
        match *self {
            Self::InRange { min, max } => on_u64_in_range(parent, context, received, min, max),
            Self::OutsideRange { min, max } => {
                on_u64_outside_range(parent, context, received, min, max)
            }

            Self::Zero => on_zero(context, received),
            Self::NotZero => on_not_zero(context, received),

            Self::Positive => on_positive(parent, context, received),
            Self::Negative => on_negative(parent, context, received),

            Self::GreaterThan { expected: num } => on_comparison(
                parent,
                context,
                received.into(),
                num,
                JsonInteger::gt,
                "greater than",
            ),
            Self::GreaterThanEqual { expected: num } => on_comparison(
                parent,
                context,
                received.into(),
                num,
                JsonInteger::ge,
                "greater than equal",
            ),
            Self::LessThan { expected: num } => on_comparison(
                parent,
                context,
                received.into(),
                num,
                JsonInteger::lt,
                "less than",
            ),
            Self::LessThanEqual { expected: num } => on_comparison(
                parent,
                context,
                received.into(),
                num,
                JsonInteger::le,
                "less than equal",
            ),
        }
    }
}

fn on_i64_in_range(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: i64,
    min: SerializableBound<i64>,
    max: SerializableBound<i64>,
) -> ExpectOpResult<()> {
    if !SerializableBound::contains(min, max, received) {
        return Err(ExpectOpError::custom(
            parent,
            context,
            format!(
                "integer is not in range
    expected {}..{}
    received {received}",
                min.as_lowerbound(),
                max,
            ),
        ));
    }

    Ok(())
}

fn on_u64_in_range(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: u64,
    min: SerializableBound<i64>,
    max: SerializableBound<i64>,
) -> ExpectOpResult<()> {
    if !SerializableBound::contains(min, max, received) {
        return Err(ExpectOpError::custom(
            parent,
            context,
            format!(
                "integer is not in range
    expected {}..{}
    received {received}",
                min.as_lowerbound(),
                max,
            ),
        ));
    }

    Ok(())
}

fn on_i64_outside_range(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: i64,
    min: SerializableBound<i64>,
    max: SerializableBound<i64>,
) -> ExpectOpResult<()> {
    if SerializableBound::contains(min, max, received) {
        return Err(ExpectOpError::custom(
            parent,
            context,
            format!(
                "integer is in range
    expected {}..{}
    received {received}",
                min.as_lowerbound(),
                max,
            ),
        ));
    }

    Ok(())
}

fn on_u64_outside_range(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: u64,
    min: SerializableBound<i64>,
    max: SerializableBound<i64>,
) -> ExpectOpResult<()> {
    if SerializableBound::contains(min, max, received) {
        return Err(ExpectOpError::custom(
            parent,
            context,
            format!(
                "integer is in range
    expected {}..{}
    received {received}",
                min.as_lowerbound(),
                max,
            ),
        ));
    }

    Ok(())
}

fn on_zero<I>(context: &mut Context<'_>, received: I) -> ExpectOpResult<()>
where
    I: Zero + Into<IntegerObject>,
{
    if !received.is_zero() {
        return Err(ExpectOpError::IntegerIsNotZero {
            context: context.to_static(),
            received: received.into(),
        });
    }

    Ok(())
}

fn on_not_zero<I>(context: &mut Context<'_>, received: I) -> ExpectOpResult<()>
where
    I: Zero + Into<IntegerObject>,
{
    if received.is_zero() {
        return Err(ExpectOpError::IntegerIsZero {
            context: context.to_static(),
            received: received.into(),
        });
    }

    Ok(())
}

fn on_positive<N>(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: N,
) -> ExpectOpResult<()>
where
    N: IntTrait,
{
    if !received.is_positive() {
        return Err(ExpectOpError::custom(
            parent,
            context,
            format!(
                "integer is not positive
    received {received}"
            ),
        ));
    }

    Ok(())
}

fn on_negative<N>(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: N,
) -> ExpectOpResult<()>
where
    N: IntTrait,
{
    if !received.is_negative() {
        return Err(ExpectOpError::custom(
            parent,
            context,
            format!(
                "integer is not negative
    received {received}"
            ),
        ));
    }

    Ok(())
}

fn on_comparison<F>(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: JsonInteger,
    expected: JsonInteger,
    comparison: F,
    comparison_name: &'static str,
) -> ExpectOpResult<()>
where
    F: Fn(JsonInteger, JsonInteger) -> bool,
{
    if !comparison(received, expected) {
        return Err(ExpectOpError::custom(
            parent,
            context,
            format!(
                "integer is out of bounds,
    expected {comparison_name} {expected}
    received {received}"
            ),
        ));
    }

    Ok(())
}

trait IntTrait: Copy + Display + Debug {
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}

impl IntTrait for i64 {
    #[inline]
    fn is_positive(&self) -> bool {
        *self > 0
    }

    #[inline]
    fn is_negative(&self) -> bool {
        *self < 0
    }
}

impl IntTrait for u64 {
    #[inline]
    fn is_positive(&self) -> bool {
        true
    }

    #[inline]
    fn is_negative(&self) -> bool {
        false
    }
}
