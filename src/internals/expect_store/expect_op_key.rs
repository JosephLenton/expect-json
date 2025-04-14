use serde::Deserialize;
use serde::Serialize;
use slotmap::new_key_type;
use slotmap::KeyData;

new_key_type! { pub struct ExpectOpKey; }

impl Serialize for ExpectOpKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0.as_ffi())
    }
}

impl<'de> Deserialize<'de> for ExpectOpKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id_raw = u64::deserialize(deserializer)?;
        Ok(ExpectOpKey(KeyData::from_ffi(id_raw)))
    }
}
