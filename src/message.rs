#[cfg(feature = "json")]
pub type Value = ::serde_json::Value;

#[cfg(not(feature = "json"))]
pub type Value = ::serde_cbor::Value;

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "origin", content = "seq")]
pub enum Origin {
    Server(u32),
    Plugin(u32),
}

#[derive(Serialize, Deserialize)]
pub enum PluginMessageKind {}

#[derive(Serialize, Deserialize)]
pub struct PluginMessage {
    seq: Origin,
    kind: PluginMessageKind,
    contents: Value,
}
