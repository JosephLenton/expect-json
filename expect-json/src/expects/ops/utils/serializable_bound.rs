use crate::internals::objects::FloatObject;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Bound;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound = "V: DeserializeOwned")]
pub enum SerializableBound<V>
where
    V: Debug + Copy + Clone + PartialOrd<V> + Serialize + DeserializeOwned,
{
    Included(V),
    Excluded(V),
    Unbounded,
}

impl<V> SerializableBound<V>
where
    V: Debug + Copy + Clone + PartialOrd<V> + Serialize + DeserializeOwned,
{
    pub(crate) fn contains(min: Self, max: Self, value: V) -> bool {
        let is_min_match = match min {
            Self::Included(min) => value >= min,
            Self::Excluded(min) => value > min,
            Self::Unbounded => true,
        };

        let is_max_match = match max {
            Self::Included(max) => value <= max,
            Self::Excluded(max) => value < max,
            Self::Unbounded => true,
        };

        is_min_match && is_max_match
    }
}

impl<V> SerializableBound<V>
where
    V: Debug + Copy + Clone + PartialOrd<V> + Serialize + DeserializeOwned + Display,
    SerializableLowerBound<V>: Display + Copy,
{
    pub(crate) fn as_lowerbound(self) -> SerializableLowerBound<V> {
        SerializableLowerBound(self)
    }
}

impl SerializableBound<i64> {
    pub fn is_negative(self) -> bool {
        match self {
            Self::Included(value) => value < 0,
            Self::Excluded(value) => value <= 0,
            Self::Unbounded => false,
        }
    }

    pub fn into_u64(self) -> SerializableBound<u64> {
        match self {
            Self::Included(value) => SerializableBound::Included(value.try_into().unwrap()),
            Self::Excluded(value) => SerializableBound::Excluded(value.try_into().unwrap()),
            Self::Unbounded => SerializableBound::Unbounded,
        }
    }
}

impl<V> From<Bound<V>> for SerializableBound<V>
where
    V: Debug + Copy + Clone + PartialOrd<V> + Serialize + DeserializeOwned,
{
    fn from(bound: Bound<V>) -> Self {
        match bound {
            Bound::Included(value) => Self::Included(value),
            Bound::Excluded(value) => Self::Excluded(value),
            Bound::Unbounded => Self::Unbounded,
        }
    }
}

impl Display for SerializableBound<i64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Included(value) => write!(f, "={value}"),
            Self::Excluded(value) => write!(f, "{value}"),
            Self::Unbounded => write!(f, ""),
        }
    }
}

impl Display for SerializableBound<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Included(value) => write!(f, "={}", FloatObject::from(*value)),
            Self::Excluded(value) => write!(f, "{}", FloatObject::from(*value)),
            Self::Unbounded => write!(f, ""),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct SerializableLowerBound<V>(SerializableBound<V>)
where
    V: Debug + Copy + Clone + PartialOrd<V> + Serialize + DeserializeOwned + Display;

impl Display for SerializableLowerBound<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.0 {
            SerializableBound::Included(inner_value) => {
                let inner = SerializableBound::Excluded(inner_value);
                write!(f, "{inner}")
            }
            inner => write!(f, "{inner}"),
        }
    }
}

impl Display for SerializableLowerBound<i64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.0 {
            SerializableBound::Included(inner_value) => {
                let inner = SerializableBound::Excluded(inner_value);
                write!(f, "{inner}")
            }
            inner => write!(f, "{inner}"),
        }
    }
}

#[cfg(test)]
mod test_is_negative {
    use super::*;

    #[test]
    fn it_should_be_true_for_inclusive_minus_one() {
        let bound = SerializableBound::Included(-1);
        assert!(bound.is_negative());
    }

    #[test]
    fn it_should_be_false_for_inclusive_zero() {
        let bound = SerializableBound::Included(0);
        assert!(!bound.is_negative());
    }

    #[test]
    fn it_should_be_true_for_exclusive_minus_one() {
        let bound = SerializableBound::Excluded(-1);
        assert!(bound.is_negative());
    }

    #[test]
    fn it_should_be_true_for_exclusive_zero() {
        let bound = SerializableBound::Excluded(0);
        assert!(bound.is_negative());
    }

    #[test]
    fn it_should_be_false_for_unbounded() {
        let bound = SerializableBound::Unbounded;
        assert!(!bound.is_negative());
    }

    #[test]
    fn it_should_be_false_for_positive_value() {
        let bound = SerializableBound::Included(1);
        assert!(!bound.is_negative());
    }
}
