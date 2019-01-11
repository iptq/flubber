extern crate bytes;
extern crate dirs;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tokio;
extern crate tokio_codec;

mod buffer;
mod config;
mod conn;
mod errors;
mod plugin;
mod proto;

use crate::config::ConnectionConfig;
use crate::conn::Connection;
use futures::{
    future::{self, FutureResult},
    Future, Stream,
};
use tokio::{net::{TcpListener, UnixListener}, io::AsyncRead};
use tokio_codec::Decoder;

pub use crate::buffer::Buffer;
pub use crate::config::Config;
pub use crate::errors::Error;
pub use crate::plugin::Plugin;
pub use crate::proto::ClientCodec;

pub struct Flubber {
    config: Config,
    buffers: Vec<Buffer>,
    plugins: Vec<Plugin>,
}

impl Flubber {
    pub fn from_config(config: Config) -> Flubber {
        Flubber {
            config,
            buffers: Vec::new(),
            plugins: Vec::new(),
        }
    }

    pub fn run(&self) -> impl Future<Item = (), Error = Error> {
        let client_connection = {
            let conn = match self.config.client_connection {
                ConnectionConfig::Unix { ref path } => {
                    UnixListener::bind(path).map(|listener| Connection::Unix(listener))
                }
                ConnectionConfig::Tcp { ref addr } => {
                    // TODO: implement TLS
                    TcpListener::bind(addr).map(|listener| Connection::Tcp(listener))
                }
            };
            let conn = conn.unwrap();
            conn.incoming().for_each(|socket| {
                let framed = socket.framed(ClientCodec);
                println!("Connected!");
                future::ok(())
            })
        };

        let plugin_connection: FutureResult<_, Error> = future::ok(());

        client_connection.join(plugin_connection).map(|_| ())
    }
}
