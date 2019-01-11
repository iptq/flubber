use crate::Error;
use bytes::BytesMut;
use tokio_codec::{Decoder, Encoder};

pub enum ClientMessage {}

pub struct ClientCodec;

impl Encoder for ClientCodec {
    type Item = ClientMessage;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Decoder for ClientCodec {
    type Item = ClientMessage;
    type Error = Error;

    fn decode(&mut self, bytes: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}
