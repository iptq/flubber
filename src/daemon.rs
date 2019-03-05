use std::net::SocketAddr;
use std::path::Path;

use futures::{
    future::{self, Either},
    stream::SplitStream,
    Future, Stream,
};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio_codec::{Decoder, Framed};

use crate::errors::Error;
use crate::plugins::Plugin;
use crate::proto::{Codec, Packet};
use crate::select::SelectSet;

fn consume_stream<F, S, R>(stream: S, f: F) -> Box<Future<Item = (), Error = Error> + Send + Sync>
where
    S: Stream<Error = Error> + Send + Sync + 'static,
    F: Fn(S::Item) -> R + Send + Sync + 'static,
    R: Future<Item = (), Error = Error> + Send + Sync + 'static,
{
    let result = stream
        .into_future()
        .map_err(|(opt_head, tail)| Error::from(opt_head))
        .and_then(|(opt_head, tail)| match opt_head {
            Some(head) => Either::A(f(head).join(consume_stream(tail, f)).map(|_| ())),
            None => Either::B(consume_stream(tail, f)),
        });
    Box::new(result)
}

pub struct Daemon {
    plugins: Vec<Plugin>,
    reader: SelectSet<i32, SplitStream<Framed<Plugin, Codec<Packet>>>>,
}

impl Daemon {
    pub fn new() -> Self {
        Daemon {
            plugins: Vec::new(),
            reader: SelectSet::new(),
        }
    }

    pub fn add_plugin(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let codec = Codec::<Packet>::new();
        let plugin = Plugin::new(path)?;
        let framed = codec.framed(plugin.clone());
        let (framed_write, framed_read) = framed.split();
        self.plugins.push(plugin);
        self.reader.insert(0, framed_read);
        Ok(())
    }

    pub fn run(self) -> impl Future<Item = (), Error = Error> + Send + Sync {
        let clients = consume_stream(
            TcpListener::bind(&"127.0.0.1:9292".parse::<SocketAddr>().unwrap())
                .unwrap()
                .incoming()
                .map_err(Error::from),
            |client| {
                let codec = Codec::<Packet>::new();
                let framed = codec.framed(client);
                let (framed_write, framed_read) = framed.split();
                framed_read.for_each(|packet| {
                    eprintln!("incoming packet: {:?}", packet);
                    future::ok(())
                })
            },
        );
        let plugins = self.reader.for_each(|packet| {
            println!("packet: {:?}", packet);
            future::ok(())
        });
        plugins.join(clients).map(|_| ())
    }
}
