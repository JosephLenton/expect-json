use super::Contains;
use super::ExpectMagicId;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use strum::IntoStaticStr;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializeExpect {
    pub magic_id: ExpectMagicId,
    pub inner: SerializeExpectOp,
}

impl SerializeExpect {
    pub fn new(inner: SerializeExpectOp) -> Self {
        Self {
            magic_id: ExpectMagicId::__ExpectJson_MagicId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__,
            inner,
        }
    }

    pub fn maybe_parse(value: &Value) -> Option<Self> {
        if !Self::has_magic_id(value) {
            return None;
        }

        let obj = serde_json::from_value::<Self>(value.clone())
            .expect("Failed to decode internal expect structure from Json");
        Some(obj)
    }
}

impl SerializeExpect {
    pub fn has_magic_id(value: &Value) -> bool {
        value
            .as_object()
            .and_then(|obj| obj.get_key_value("magic_id"))
            .map(|(_, maybe_value_str)| ExpectMagicId::is_magic_id_value(maybe_value_str))
            .unwrap_or_default()
    }
}

impl<S> From<S> for SerializeExpect
where
    SerializeExpectOp: From<S>,
{
    fn from(inner: S) -> Self {
        Self {
            magic_id: Default::default(),
            inner: SerializeExpectOp::from(inner),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, IntoStaticStr)]
#[serde(tag = "type")]
pub enum SerializeExpectOp {
    Contains(Contains),
}

#[cfg(test)]
mod test_serialize {
    use crate::expect;
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    #[test]
    fn it_should_serialize_into_expected_structure_with_magic_id() {
        let output = json!(expect.contains([1, 2, 3]));
        let expected = json!({
            "magic_id": "__ExpectJson_MagicId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__",
            "inner": {
                "type": "Contains",
                "values": [1, 2, 3],
            },
        });

        assert_json_eq!(output, expected);
    }
}
