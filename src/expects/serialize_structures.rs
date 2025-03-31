use super::ExpectMagicId;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializeExpect<'a> {
    pub magic_id: ExpectMagicId,
    pub inner: SerializeExpectOp<'a>,
    pub is_not: bool,
}

impl SerializeExpect<'static> {
    pub fn maybe_parse(value: &Value) -> Option<Self> {
        if !Self::has_magic_id(value) {
            return None;
        }

        let obj = serde_json::from_value::<Self>(value.clone())
            .expect("Failed to decode internal expect structure from Json");
        Some(obj)
    }
}

impl SerializeExpect<'_> {
    pub fn has_magic_id(value: &Value) -> bool {
        value
            .as_object()
            .and_then(|obj| obj.get_key_value("magic_id"))
            .map(|(_, maybe_value_str)| ExpectMagicId::is_magic_id_value(maybe_value_str))
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SerializeExpectOp<'a> {
    Contains { values: Cow<'a, Vec<Value>> },
}

#[cfg(test)]
mod test_serialize {
    use crate::expects::Contains;
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    #[test]
    fn it_should_serialize_into_expected_structure_with_magic_id() {
        let output = json!(Contains::new([1, 2, 3]));
        let expected = json!({
            "magic_id": "__ExpectJson_MagicId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__",
            "inner": {
                "type": "Contains",
                "values": [1, 2, 3],
            },
            "is_not": false,
        });

        assert_json_eq!(output, expected);
    }
}
