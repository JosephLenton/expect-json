use crate::expect::ops::utils::SerializableBound;
use crate::expect::ops::utils::SerializableBoundContains;
use crate::expect::ops::ExpectInteger;
use crate::expect_core::Context;
use crate::expect_core::ExpectOpError;
use crate::expect_core::ExpectOpResult;
use crate::internals::objects::IntegerObject;
use num::Zero;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

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
}

impl ExpectIntegerSubOp {
    pub(crate) fn on_i64(
        &self,
        parent: &ExpectInteger,
        context: &mut Context<'_>,
        received: i64,
    ) -> ExpectOpResult<()> {
        match self {
            Self::InRange { min, max } => on_i64_in_range(parent, context, received, *min, *max),
            Self::OutsideRange { min, max } => {
                on_i64_outside_range(parent, context, received, *min, *max)
            }
            Self::Zero => on_zero(context, received),
            Self::NotZero => on_not_zero(context, received),
            Self::Positive => on_i64_positive(parent, context, received),
            Self::Negative => on_i64_negative(parent, context, received),
        }
    }

    pub(crate) fn on_u64(
        &self,
        parent: &ExpectInteger,
        context: &mut Context<'_>,
        received: u64,
    ) -> ExpectOpResult<()> {
        match self {
            Self::InRange { min, max } => on_u64_in_range(parent, context, received, *min, *max),
            Self::OutsideRange { min, max } => {
                on_u64_outside_range(parent, context, received, *min, *max)
            }
            Self::Zero => on_zero(context, received),
            Self::NotZero => on_not_zero(context, received),
            Self::Positive => {
                // Do nothing, all u64 values are positive
                Ok(())
            }
            Self::Negative => Err(ExpectOpError::custom(
                parent,
                context,
                format!(
                    "integer is not negative
    received {received}"
                ),
            )),
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

fn on_i64_positive(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: i64,
) -> ExpectOpResult<()> {
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

fn on_i64_negative(
    parent: &ExpectInteger,
    context: &mut Context<'_>,
    received: i64,
) -> ExpectOpResult<()> {
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
