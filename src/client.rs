use std::net::SocketAddr;

use futures::{future, sync::mpsc, Future, Stream};
use tokio::net::TcpStream;
use tokio_codec::Decoder;

use crate::proto::Codec;
use crate::{Error, Packet};

pub struct Client {
    from_gui: mpsc::UnboundedReceiver<Packet>,
    to_gui: mpsc::UnboundedSender<Packet>,
}

impl Client {
    pub fn new(
        from_gui: mpsc::UnboundedReceiver<Packet>,
        to_gui: mpsc::UnboundedSender<Packet>,
    ) -> Client {
        Client { from_gui, to_gui }
    }

    pub fn run(self) -> impl Future<Item = (), Error = Error> {
        TcpStream::connect(&"127.0.0.1:9292".parse::<SocketAddr>().unwrap())
            .map_err(Error::from)
            .and_then(|stream| {
                let codec = Codec::<Packet>::new();
                let framed = codec.framed(stream);
                let (framed_write, framed_read) = framed.split();
                framed_read.for_each(|_| future::ok(()))
            })
    }
}
