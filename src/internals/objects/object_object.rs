use crate::internals::objects::pretty_formatter::PrettyDisplay;
use crate::internals::objects::pretty_formatter::PrettyFormatter;
use crate::internals::objects::ValueObject;
use crate::internals::JsonObject;
use serde_json::Map;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectObject(JsonObject);

impl ObjectObject {
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, Value)>,
    {
        let inner = Map::from_iter(iter);
        Self(inner)
    }
}

impl From<JsonObject> for ObjectObject {
    fn from(inner: JsonObject) -> Self {
        Self(inner)
    }
}

impl Display for ObjectObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let mut pretty_formatter = PrettyFormatter::new(formatter);
        self.pretty_fmt(&mut pretty_formatter)?;

        Ok(())
    }
}

impl PrettyDisplay for ObjectObject {
    fn pretty_fmt(&self, formatter: &mut PrettyFormatter<'_, '_>) -> FmtResult {
        if self.0.is_empty() {
            return write!(formatter, "{{ }}");
        }

        writeln!(formatter, "{{")?;
        for (key, value) in &self.0 {
            // TODO, remove this clone
            let value_obj = ValueObject::from(value.clone());
            writeln!(formatter, r#"        "{key}": {value_obj},"#)?;
        }
        write!(formatter, "    }}")?;

        Ok(())
    }
}
