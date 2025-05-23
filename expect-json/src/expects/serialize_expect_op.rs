use crate::expects::ExpectOpMarkerId;
use crate::ExpectOpSerialize;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

thread_local! {
    static SERIALIZATION_IS_AT_ROOT: AtomicUsize = const { AtomicUsize::new(0) };
}

#[doc(hidden)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializeExpectOp {
    magic_id: ExpectOpMarkerId,
    pub inner: Box<dyn ExpectOpSerialize>,
}

impl SerializeExpectOp {
    pub(crate) fn has_magic_id(value: &Value) -> bool {
        value.as_object().is_some_and(Self::has_object_magic_id)
    }

    pub(crate) fn has_object_magic_id(object: &Map<String, Value>) -> bool {
        object
            .get("magic_id")
            .is_some_and(ExpectOpMarkerId::is_magic_id_value)
    }

    pub fn serialize<E, S>(expect_op: &E, serializer: S) -> Result<S::Ok, S::Error>
    where
        E: ExpectOpSerialize + Clone + crate::__private::serde_trampoline::Serialize,
        S: serde::Serializer,
    {
        let is_at_root =
            SERIALIZATION_IS_AT_ROOT.with(|is_at_root| is_at_root.fetch_add(1, Ordering::Acquire));

        let result = if is_at_root == 0 {
            SerializeExpectOp::new(Box::new(expect_op.clone())).serialize(serializer)
        } else {
            crate::__private::serde_trampoline::Serialize::serialize(expect_op, serializer)
        };

        SERIALIZATION_IS_AT_ROOT.with(|is_at_root| {
            is_at_root.fetch_sub(1, Ordering::Release);
        });

        result
    }

    pub fn new(inner: Box<dyn ExpectOpSerialize>) -> Self {
        Self {
            magic_id:
                ExpectOpMarkerId::__ExpectJson_MarkerId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__,
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

    pub fn maybe_parse_from_obj(object: &Map<String, Value>) -> Option<Box<dyn ExpectOpSerialize>> {
        if !Self::has_object_magic_id(object) {
            return None;
        }

        let inner = object.get("inner")?;
        let obj = serde_json::from_value(inner.clone())
            .expect("Failed to decode internal expect structure from Json");
        Some(obj)
    }
}

impl From<SerializeExpectOp> for Box<dyn ExpectOpSerialize> {
    fn from(value: SerializeExpectOp) -> Self {
        value.inner
    }
}

#[cfg(test)]
mod test_serialize {
    use crate::expect;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn it_should_serialize_into_expected_structure_with_expect_id_marker() {
        let output = json!(expect::array().contains([1, 2, 3]));
        assert_eq!(
            output,
            json!({
                "magic_id": "__ExpectJson_MarkerId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__",
                "inner": {
                    "sub_ops": [
                        {
                            "Contains": [
                                1,
                                2,
                                3,
                            ],
                        },
                    ],
                    "type": "ExpectArray"
                },
            })
        );
    }

    #[test]
    fn it_should_serialize_an_op_within_an_inner_op_inside() {
        let output = json!(expect::array().contains([expect::object().contains(json!({
            "name": "John",
            "age": 30,
        }))]));

        assert_eq!(
            output,
            json!({
                "magic_id": "__ExpectJson_MarkerId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__",
                "inner": {
                    "sub_ops": [
                        {
                            "Contains": [
                                {
                                    "magic_id": "__ExpectJson_MarkerId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__",
                                    "inner": {
                                        "sub_ops": [
                                            {
                                                "Contains": {
                                                    "age": 30,
                                                    "name": "John",
                                                },
                                            },
                                        ],
                                        "type": "ExpectObject"
                                    },
                                },
                            ],
                        },
                    ],
                    "type": "ExpectArray"
                },
            })
        );
    }
}
