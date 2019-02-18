use tokio_codec::{Decoder, Encoder};
use serde_cbor;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PluginMessage {
}

pub struct PluginCodec;

impl Encoder for PluginCodec {
	type Item = PluginMessage;
}
