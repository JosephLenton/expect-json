use crate::expects::ExpectMagicId;
use crate::ExpectOp;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[doc(hidden)]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerializeExpectOp {
    pub magic_id: ExpectMagicId,
    pub inner: Box<dyn ExpectOp>,
}

impl SerializeExpectOp {
    pub fn new(inner: Box<dyn ExpectOp>) -> Self {
        Self {
            magic_id: ExpectMagicId::__ExpectJson_MagicId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__,
            inner,
        }
    }

    pub fn maybe_parse(value: &Value) -> Option<Self> {
        if !Self::has_magic_id(value) {
            return None;
        }

        let obj = serde_json::from_value(value.clone())
            .expect("Failed to decode internal expect structure from Json");
        Some(obj)
    }

    pub fn maybe_parse_from_obj(object: &Map<String, Value>) -> Option<Box<dyn ExpectOp>> {
        if !Self::has_object_magic_id(object) {
            return None;
        }

        let inner = object.get("inner")?;
        let obj = serde_json::from_value(inner.clone())
            .expect("Failed to decode internal expect structure from Json");
        Some(obj)
    }
}

impl SerializeExpectOp {
    pub fn has_magic_id(value: &Value) -> bool {
        value.as_object().is_some_and(Self::has_object_magic_id)
    }

    pub fn has_object_magic_id(object: &Map<String, Value>) -> bool {
        object
            .get("magic_id")
            .is_some_and(ExpectMagicId::is_magic_id_value)
    }
}

/*
#[cfg(test)]
mod test_serialize {
    use crate::expect;
    use serde_json::json;
    use crate::expects::expect;
    use crate::ops::Contains;
    use crate::SerializeExpectOp2;

    #[test]
    fn it_should_serialize_into_expected_structure_with_magic_id() {
        let output = json!(expect.contains([1, 2, 3]));
        let contains = Contains::new(json!([1, 2, 3]));

        let deserialized = serde_json::from_value::<SerializeExpectOp2>(output).unwrap();
        assert_eq!(deserialized, SerializeExpectOp2 {
            magic_id: ExpectMagicId::__ExpectJson_MagicId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__,
            inner: Box::new(contains),
        });
    }
}
*/
