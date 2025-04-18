use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub(crate) enum ExpectOpMarkerId {
    /// This is an ID to uniquely identify the Expect Json objects over anything else.
    ///
    /// The ID contains a UUID which I generated on my machine.
    /// It doesn't matter what that value is, or if it's known by others.
    /// All that matters is it is unique enough that it is impossible to
    /// clash with other code by accident.
    #[allow(non_camel_case_types)]
    #[default]
    __ExpectJson_MarkerId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__,
}

impl ExpectOpMarkerId {
    pub fn is_magic_id_value(value: &Value) -> bool {
        value
            .as_str()
            .map(|value_str| {
                value_str == "__ExpectJson_MarkerId_0ABDBD14_93D1_4D73_8E26_0177D8A280A4__"
            })
            .unwrap_or_default()
    }
}
