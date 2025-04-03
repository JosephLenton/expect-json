use crate::expects::ops::ArrayContains;
use crate::expects::ops::StringContains;
use crate::internals::Context;
use crate::internals::JsonExpectOp;
use crate::internals::JsonValueEqResult;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use strum::IntoStaticStr;

#[derive(Clone, Debug, Serialize, Deserialize, IntoStaticStr)]
#[serde(tag = "type")]
pub enum SerializeExpectOp {
    ArrayContains(ArrayContains),
    StringContains(StringContains),
}

impl SerializeExpectOp {
    pub fn on_any<'a>(
        self,
        context: &mut Context<'a>,
        received: &'a Value,
    ) -> JsonValueEqResult<()> {
        match self {
            Self::ArrayContains(inner) => inner.on_any(context, received),
            Self::StringContains(inner) => inner.on_any(context, received),
        }
    }
}
