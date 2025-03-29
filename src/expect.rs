use serde_json::Value;
use serde::Serialize;

pub fn capture<V>(value: V) -> CapturedObject
where
    V: Serialize
{
    CapturedObject::new(value)
}

pub struct CapturedObject {
    value: Value,
}

impl CapturedObject {
    fn new<V>(raw_value: V) -> Self
    where
        V: Serialize
    {
        Self {
            value: serde_json::to_value(raw_value).expect("Failed to turn given value into Json"),
        }
    }

    pub fn equals<V>(other: V) -> bool
    where
        other: Serialize
    {

    }
}
