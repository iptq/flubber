extern crate flubber;
extern crate futures;
#[macro_use]
extern crate structopt;
extern crate termion;
extern crate tokio;

mod client;
mod ui;

use std::net::SocketAddr;
use std::process;
use std::thread;

use crate::ui::GUI;
use futures::sync::mpsc;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct ConnectionConfig {
    #[structopt(
        name = "path",
        long = "unix",
        help = "Argument is the path to the socket"
    )]
    unix: Option<String>,

    #[structopt(
        name = "addr",
        long = "tcp",
        help = "Argument is the address to bind to (ex: '127.0.0.1:5060')"
    )]
    tcp: Option<SocketAddr>,
}

#[derive(Clone, Debug, StructOpt)]
pub struct Opt {
    #[structopt(flatten)]
    connection: ConnectionConfig,
}

fn main() {
    let args = Opt::from_args();

    let (tx, rx) = mpsc::unbounded();
    let mut gui = GUI::new(tx);

    thread::spawn(move || client::run(args.clone()));
    match gui.run() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Unexpected error: {}", err);
            process::exit(123);
        }
    }
}
