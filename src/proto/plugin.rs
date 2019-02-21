use bytes::BytesMut;
use serde_cbor;
use tokio_codec::{Decoder, Encoder};

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PluginMessage {}

#[derive(Default)]
pub struct PluginCodec;

impl Encoder for PluginCodec {
    type Item = PluginMessage;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        // TODO: not use .reserve()?
        let value = serde_cbor::to_vec(&item)?;
        bytes.reserve(value.len());
        bytes.extend(value);
        Ok(())
    }
}

impl Decoder for PluginCodec {
    type Item = PluginMessage;
    type Error = Error;

    fn decode(&mut self, bytes: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        serde_cbor::from_slice(bytes)
            .map(|item| Some(item))
            .map_err(|err| err.into())
    }
}
