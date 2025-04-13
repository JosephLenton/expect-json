use crate::expects::ops::ArrayContains;
use crate::expects::ops::ArrayContainsNot;
use crate::expects::ops::StringContains;
use crate::expects::ops::StringContainsNot;
use crate::internals::types::ValueType;
use crate::internals::Context;
use crate::internals::JsonExpectOp;
use crate::internals::JsonValueEqResult;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use strum::IntoStaticStr;

#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize, IntoStaticStr, PartialEq)]
#[serde(tag = "type")]
pub enum SerializeExpectOp {
    ArrayContains(ArrayContains),
    ArrayContainsNot(ArrayContainsNot),
    StringContains(StringContains),
    StringContainsNot(StringContainsNot),
}

impl SerializeExpectOp {
    pub fn on_any<'a>(
        self,
        context: &mut Context<'a>,
        received: &'a Value,
    ) -> JsonValueEqResult<()> {
        match self {
            Self::ArrayContains(inner) => inner.on_any(context, received),
            Self::ArrayContainsNot(inner) => inner.on_any(context, received),
            Self::StringContains(inner) => inner.on_any(context, received),
            Self::StringContainsNot(inner) => inner.on_any(context, received),
        }
    }

    pub fn supported_types(&self) -> &'static [ValueType] {
        match self {
            Self::ArrayContains(inner) => inner.supported_types(),
            Self::ArrayContainsNot(inner) => inner.supported_types(),
            Self::StringContains(inner) => inner.supported_types(),
            Self::StringContainsNot(inner) => inner.supported_types(),
        }
    }
}
