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
mod errors;
mod plugin;
mod proto;

use futures::{future, Future};

pub use crate::buffer::Buffer;
pub use crate::config::Config;
pub use crate::errors::Error;
pub use crate::plugin::Plugin;

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
    pub fn as_future(&self) -> impl Future<Item = (), Error = Error> {
        future::ok(())
    }
}
