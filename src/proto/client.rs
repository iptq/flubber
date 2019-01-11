use crate::Error;
use bytes::BytesMut;
use serde_cbor;
use tokio_codec::{Decoder, Encoder};

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Init { password: Option<String> },
}

pub struct ClientCodec;

impl Encoder for ClientCodec {
    type Item = ClientMessage;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        // TODO: not use .reserve()?
        let value = serde_cbor::to_vec(&item)?;
        bytes.reserve(value.len());
        bytes.extend(value);
        Ok(())
    }
}

impl Decoder for ClientCodec {
    type Item = ClientMessage;
    type Error = Error;

    fn decode(&mut self, bytes: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        serde_cbor::from_slice(bytes)
            .map(|item| Some(item))
            .map_err(|err| err.into())
    }
}
