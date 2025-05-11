use crate::ops::expect_integer::SerializableBound;
use crate::ops::ExpectInteger;
use crate::Context;
use crate::ExpectOpError;
use crate::ExpectOpResult;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectIntegerSubOp {
    InRange {
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
            Self::InRange { min, max } => {
                if !SerializableBound::contains(*min, *max, received) {
                    return Err(ExpectOpError::custom(
                        context,
                        parent,
                        "integer is not in range",
                    ));
                }
            }
            Self::Zero => {
                if received != 0 {
                    return Err(ExpectOpError::IntegerIsNotZero {
                        context: context.to_static(),
                        received: received.into(),
                    });
                }
            }
            Self::NotZero => {
                if received == 0 {
                    return Err(ExpectOpError::IntegerIsZero {
                        context: context.to_static(),
                        received: received.into(),
                    });
                }
            }
            Self::Positive => {
                if !received.is_positive() {
                    return Err(ExpectOpError::custom(
                        context,
                        parent,
                        "integer is not positive",
                    ));
                }
            }
            Self::Negative => {
                if !received.is_negative() {
                    return Err(ExpectOpError::custom(
                        context,
                        parent,
                        "integer is not negative",
                    ));
                }
            }
        }

        Ok(())
    }

    pub(crate) fn on_u64(
        &self,
        parent: &ExpectInteger,
        context: &mut Context<'_>,
        received: u64,
    ) -> ExpectOpResult<()> {
        match self {
            Self::InRange { min, max } => {
                // We can max min up to 0, given all u64 values are positive
                let min_u64 = if min.is_negative() {
                    SerializableBound::Unbounded
                } else {
                    min.into_u64()
                };

                let max_u64 = if max.is_negative() {
                    return Err(ExpectOpError::custom(
                        context,
                        parent,
                        "integer is not in range",
                    ));
                } else {
                    max.into_u64()
                };

                if !SerializableBound::contains(min_u64, max_u64, received) {
                    return Err(ExpectOpError::custom(
                        context,
                        parent,
                        "integer is not in range",
                    ));
                }
            }
            Self::Zero => {
                if received != 0 {
                    return Err(ExpectOpError::IntegerIsNotZero {
                        context: context.to_static(),
                        received: received.into(),
                    });
                }
            }
            Self::NotZero => {
                if received == 0 {
                    return Err(ExpectOpError::IntegerIsZero {
                        context: context.to_static(),
                        received: received.into(),
                    });
                }
            }
            Self::Positive => {
                // Do nothing, all u64 values are positive
            }
            Self::Negative => {
                return Err(ExpectOpError::custom(
                    context,
                    parent,
                    "integer is not negative",
                ));
            }
        }

        Ok(())
    }
}
