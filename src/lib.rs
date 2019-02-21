#[macro_use]
extern crate serde_derive;

mod buffer;
mod config;
pub mod conn;
mod errors;
mod plugin;
mod proto;

use crate::conn::Listener;
use futures::{
    future::{self, FutureResult},
    Future, Stream,
};
use radix_trie::Trie;
use tokio::{
    io::AsyncRead,
    net::{TcpListener, UnixListener},
};
use tokio_codec::FramedRead;

pub use crate::buffer::Buffer;
pub use crate::config::{Config, ConnectionConfig};
pub use crate::errors::Error;
pub use crate::plugin::Plugin;
pub use crate::proto::*;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Flubber {
    config: Config,
    plugins: Vec<Plugin>,
    buffers: Trie<String, Buffer>,
}

impl Flubber {
    pub fn from_config(config: Config) -> Flubber {
        Flubber {
            config,
            plugins: Vec::new(),
            buffers: Trie::new(),
        }
    }

    pub fn run(&mut self) -> impl Future<Item = (), Error = ()> {
        self.buffers
            .insert(String::from("flubber"), Buffer::root_buffer());

        let client_connection = {
            let conn = match self.config.client_connection {
                ConnectionConfig::Unix { ref path } => {
                    UnixListener::bind(path).map(|listener| Listener::Unix(listener))
                }
                ConnectionConfig::Tcp { ref addr } => {
                    // TODO: implement TLS
                    TcpListener::bind(addr).map(|listener| Listener::Tcp(listener))
                }
            };
            let conn = conn.unwrap();
            conn.incoming()
                .map_err(|err| {
                    eprintln!("client connection error: {}", err);
                })
                .for_each(|socket| {
                    let (stream, sink) = socket.split();

                    let framed = FramedRead::new(stream, ClientCodec);
                    println!("connected!");
                    framed
                        .for_each(|message| {
                            println!("received message {:?}", message);
                            future::ok(())
                        })
                        // using or_else so an error in a single client doesn't
                        // kill the entire server's stream
                        .or_else(|err| {
                            eprintln!("client error: {}", err);
                            future::ok(())
                        })
                })
        };

        let plugin_connection: FutureResult<_, ()> = future::ok(());

        client_connection.join(plugin_connection).map(|_| ())
    }
}
