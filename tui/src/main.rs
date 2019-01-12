extern crate flubber;
extern crate futures;
extern crate structopt;
extern crate termion;
extern crate tokio;
extern crate tokio_codec;
extern crate uuid;

mod client;
mod ui;

use std::net::SocketAddr;
use std::process;

use crate::ui::GUI;
use futures::{sync::{mpsc, oneshot}, Future};
use structopt::StructOpt;
use tokio::runtime::Runtime;

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
    let (stop_send, stop_recv) = oneshot::channel();

    let (from_ui, to_ui) = mpsc::unbounded();
    let (from_thread, to_thread) = mpsc::unbounded();

    let mut gui = GUI::new(to_thread, from_ui, stop_send);

    // thread::spawn(move || client::run(args.clone(), to_ui, from_thread));
    // match gui.run() {
    //     Ok(_) => (),
    //     Err(err) => {
    //         eprintln!("Unexpected error: {}", err);
    //         process::exit(123);
    //     }
    // }

    let mut runtime = Runtime::new().unwrap();
    runtime.spawn(client::run(args.clone(), to_ui, from_thread).map_err(|err| {
        eprintln!("client error: {}", err);
    }));
    runtime.spawn(gui.run().map_err(|err| {
        eprintln!("gui error: {}", err);
    }));
    runtime.block_on(stop_recv);
    runtime.shutdown_now();
}
