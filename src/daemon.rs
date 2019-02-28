use std::path::Path;

use futures::{future, Future, Stream};
use tokio::io::Stdin;
use tokio_codec::{Decoder, FramedRead};

use crate::errors::Error;
use crate::plugins::Plugin;
use crate::proto::PluginCodec;
use crate::select::SelectSet;

pub struct Daemon {
    plugins: Vec<Plugin>,
    reader: SelectSet<i32, FramedRead<Stdin, PluginCodec>>,
}

impl Daemon {
    pub fn new() -> Self {
        Daemon {
            plugins: Vec::new(),
            reader: SelectSet::new(),
        }
    }

    pub fn add_plugin(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let codec = PluginCodec::default();
        let plugin = Plugin::new(path)?;
        let framed = codec.framed(plugin.clone());
        self.plugins.push(plugin);
        Ok(())
    }

    pub fn start(self) -> impl Future<Item = (), Error = Error> {
        self.reader.for_each(|packet| {
            println!("packet: {:?}", packet);
            future::ok(())
        })
    }
}
