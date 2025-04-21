use crate::expects::ops::Contains;
use crate::ops::IsEmpty;
use crate::ops::IsoDateTime;
use crate::ExpectNot;
use serde_json::Value;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Expect {
    pub not: ExpectNot,
}

impl Expect {
    pub(crate) const fn new() -> Self {
        Self { not: ExpectNot }
    }

    pub fn contains<V>(self, values: V) -> Contains
    where
        V: Into<Value>,
    {
        Contains::new(values)
    }

    pub fn iso_date_time(self) -> IsoDateTime {
        IsoDateTime::new()
    }

    pub fn is_empty(self) -> IsEmpty {
        IsEmpty
    }
}

#[cfg(test)]
mod test_new {
    use super::*;

    #[test]
    fn it_should_return_correct_structure() {
        let expect = Expect::new();
        assert_eq!(expect, Expect { not: ExpectNot });
    }
}
