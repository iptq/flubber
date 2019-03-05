#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate prost_derive;
#[macro_use]
extern crate serde_derive;

mod client;
mod daemon;
mod errors;
mod handshake;
mod plugins;
pub mod proto;
mod select;

pub use client::Client;
pub use daemon::Daemon;
pub use errors::{Error, ErrorExt, ErrorKind};
pub use proto::{Codec, Packet};
