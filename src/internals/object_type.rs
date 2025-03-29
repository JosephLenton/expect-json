use super::JsonObject;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectType(JsonObject);

impl From<JsonObject> for ObjectType {
    fn from(inner: JsonObject) -> Self {
        Self(inner)
    }
}

impl Display for ObjectType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, r#"object {:?}"#, self.0)
    }
}
