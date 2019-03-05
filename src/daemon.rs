use std::path::Path;

use futures::{future, stream::SplitStream, Future, Stream};
use tokio_codec::{Decoder, Framed};

use crate::errors::Error;
use crate::plugins::Plugin;
use crate::proto::PluginCodec;
use crate::select::SelectSet;

pub struct Daemon {
    plugins: Vec<Plugin>,
    reader: SelectSet<i32, SplitStream<Framed<Plugin, PluginCodec>>>,
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
        let (framed_write, framed_read) = framed.split();
        self.plugins.push(plugin);
        self.reader.insert(0, framed_read);
        Ok(())
    }

    pub fn run(self) -> impl Future<Item = (), Error = Error> {
        println!("{:?}", self.plugins.len());
        self.reader.for_each(|packet| {
            println!("packet: {:?}", packet);
            future::ok(())
        })
    }
}
