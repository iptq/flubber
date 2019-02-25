use bytes::BytesMut;
use tokio_codec::{Decoder, Encoder};

use crate::errors::{Error, ErrorKind};
use crate::message::PluginMessage;

#[derive(Default)]
pub struct PluginCodec;

impl PluginCodec {
    pub fn new() -> Self {
        PluginCodec
    }
}

impl Encoder for PluginCodec {
    type Item = PluginMessage;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        serde_cbor::to_vec(&item)
            .map(|vec| bytes.extend_from_slice(vec.as_slice()))
            .map_err(|err| Error::with_cause(ErrorKind::EncodingError, err))
    }
}

impl Decoder for PluginCodec {
    type Item = PluginMessage;
    type Error = Error;

    fn decode(&mut self, bytes: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        serde_cbor::from_slice(bytes)
            .map(Option::Some)
            .map_err(|err| Error::with_cause(ErrorKind::EncodingError, err))
    }
}
