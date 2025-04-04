use crate::internals::objects::ValueObject;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayObject(Vec<ValueObject>);

impl ArrayObject {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<I> From<I> for ArrayObject
where
    I: IntoIterator<Item = Value>,
{
    fn from(inner: I) -> Self {
        let inner_objects = inner.into_iter().map(ValueObject::from).collect();
        Self(inner_objects)
    }
}

impl Display for ArrayObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "[")?;

        for (i, obj) in self.0.iter().enumerate() {
            if i > 0 {
                write!(formatter, ", ")?;
            }

            write!(formatter, "{obj}")?;
        }

        write!(formatter, "]")
    }
}
