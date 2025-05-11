use core::fmt::Debug;
use core::ops::Bound;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound = "V: DeserializeOwned")]
pub(crate) enum SerializableBound<V>
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
