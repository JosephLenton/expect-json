use crate::expect::ops::expect_float::ExpectFloat;
use crate::expect::ops::utils::SerializableBound;
use crate::expect_core::Context;
use crate::expect_core::ExpectOpError;
use crate::expect_core::ExpectOpResult;
use crate::internals::objects::FloatObject;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpectFloatSubOp {
    InRange {
        min: SerializableBound<f64>,
        max: SerializableBound<f64>,
    },
    Zero,
    NotZero,
    Positive,
    Negative,
}

impl ExpectFloatSubOp {
    pub(crate) fn on_f64(
        &self,
        parent: &ExpectFloat,
        context: &mut Context<'_>,
        received: f64,
    ) -> ExpectOpResult<()> {
        if received.is_nan() {
            return Err(ExpectOpError::custom(
                context,
                parent,
                "float is not a number (this is an internal error, please report it at: https://github.com/JosephLenton/expect-json/issues)",
            ));
        }

        match self {
            Self::InRange { min, max } => {
                if !SerializableBound::contains(*min, *max, received) {
                    return Err(ExpectOpError::custom(
                        context,
                        parent,
                        format!(
                            "float is not in range
    expected {}..{}
    received {}",
                            min.as_lowerbound(),
                            max,
                            FloatObject::from(received)
                        ),
                    ));
                }
            }
            Self::Zero => {
                if received != 0.0 {
                    return Err(ExpectOpError::FloatIsNotZero {
                        context: context.to_static(),
                        received: received.into(),
                    });
                }
            }
            Self::NotZero => {
                if received == 0.0 {
                    return Err(ExpectOpError::FloatIsZero {
                        context: context.to_static(),
                        received: received.into(),
                    });
                }
            }
            Self::Positive => {
                if !received.is_sign_positive() {
                    return Err(ExpectOpError::custom(
                        context,
                        parent,
                        format!(
                            "float is not positive
    received {}",
                            FloatObject::from(received)
                        ),
                    ));
                }
            }
            Self::Negative => {
                if !received.is_sign_negative() {
                    return Err(ExpectOpError::custom(
                        context,
                        parent,
                        format!(
                            "float is not negative
    received {}",
                            FloatObject::from(received)
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}
