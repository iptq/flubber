pub mod client {
    include!(concat!(env!("OUT_DIR"), "/flubber.client.rs"));
}
pub mod common {
    include!(concat!(env!("OUT_DIR"), "/flubber.common.rs"));
}
pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/flubber.plugin.rs"));
}

use std::io::Cursor;
use std::marker::PhantomData;

use bytes::{Bytes, BytesMut, IntoBuf};
use prost::Message;
use tokio_codec::{Decoder, Encoder};

use crate::errors::{Error, ErrorKind};

pub use self::common::packet;
pub use self::common::{Origin, Packet, PacketId};

pub struct Codec<T>(PhantomData<T>);

impl<T> Codec<T> {
    pub fn new() -> Self {
        Codec(PhantomData::default())
    }
}

impl<T: Message> Encoder for Codec<T> {
    type Item = T;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        item.encode(bytes)
            .map_err(|err| Error::with_cause(ErrorKind::Encoding, err))
    }
}

impl<T: Default + Message> Decoder for Codec<T> {
    type Item = T;
    type Error = Error;

    fn decode(&mut self, bytes: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let bytes = bytes.clone().freeze();
        T::decode(bytes)
            .map(Option::Some)
            .map_err(|err| Error::with_cause(ErrorKind::Encoding, err))
    }
}
