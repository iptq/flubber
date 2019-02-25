use std::path::Path;

use futures::Stream;
use tokio_codec::Decoder;

use crate::errors::Error;
use crate::plugins::Plugin;
use crate::proto::PluginCodec;

pub struct Daemon {}

impl Daemon {
    pub fn new() -> Self {
        Daemon {}
    }

    pub fn add_plugin(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let codec = PluginCodec::new();
        let plugin = Plugin::new(path)?;
        let framed = codec.framed(plugin);
        Ok(())
    }

    pub fn start(&self) {}
}
