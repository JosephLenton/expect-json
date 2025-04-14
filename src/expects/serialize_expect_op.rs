use crate::expects::ExpectMagicId;
use crate::internals::expect_store::ExpectOpKey;
use crate::internals::expect_store::ExpectOpStoreKey;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SerializeExpectOp {
    pub magic_id: ExpectMagicId,
    pub store_key: ExpectOpStoreKey,
    pub op_key: ExpectOpKey,
}

impl SerializeExpectOp {
    pub fn new(store_key: ExpectOpStoreKey, op_key: ExpectOpKey) -> Self {
        Self {
            magic_id: ExpectMagicId::__ExpectJson_MagicId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__,
            store_key,
            op_key,
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

impl SerializeExpectOp {
    pub fn has_magic_id(value: &Value) -> bool {
        value
            .as_object()
            .and_then(|obj| obj.get_key_value("magic_id"))
            .map(|(_, maybe_value_str)| ExpectMagicId::is_magic_id_value(maybe_value_str))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test_serialize {
    use crate::expect;
    use serde_json::json;

    #[test]
    fn it_should_serialize_into_expected_structure_with_magic_id() {
        let output = json!(expect.contains([1, 2, 3]));

        assert!(output.is_object());
        let obj = output.as_object().unwrap();
        assert_eq!(
            obj.get("magic_id"),
            Some(&json!(
                "__ExpectJson_MagicId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__"
            ))
        );
        assert!(obj.get("store_key").is_some());
        assert!(obj.get("op_key").is_some());
    }
}
