use crate::internals::JsonObject;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectObject(JsonObject);

impl From<JsonObject> for ObjectObject {
    fn from(inner: JsonObject) -> Self {
        Self(inner)
    }
}

impl Display for ObjectObject {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, r#"{:?}"#, self.0)
    }
}
