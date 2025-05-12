use crate::internals::objects::ValueObject;
use crate::internals::pretty_formatter::PrettyDisplay;
use crate::internals::pretty_formatter::PrettyFormatter;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayObject(pub Vec<ValueObject>);

impl ArrayObject {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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
        let mut pretty_formatter = PrettyFormatter::new(formatter);
        self.pretty_fmt(&mut pretty_formatter)?;

        Ok(())
    }
}

impl PrettyDisplay for ArrayObject {
    fn pretty_fmt(&self, formatter: &mut PrettyFormatter<'_, '_>) -> FmtResult {
        formatter.write_fmt_array(&self.0)
    }

    fn is_indenting(&self) -> bool {
        self.0
            .first()
            .map(|value| value.is_indenting())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test_len {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_should_have_len_of_inner_array() {
        let array_object = ArrayObject::from(vec![json!(1), json!(2), json!(3)]);
        assert_eq!(array_object.len(), 3);
    }
}

#[cfg(test)]
mod test_is_empty {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_should_be_true_when_inner_array_is_empty() {
        let array_object = ArrayObject::from(vec![]);
        assert!(array_object.is_empty());
    }

    #[test]
    fn it_should_be_false_when_inner_array_is_not_empty() {
        let array_object = ArrayObject::from(vec![json!(1)]);
        assert!(!array_object.is_empty());
    }
}
