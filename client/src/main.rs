#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use flubber::{Client, ErrorExt, Packet};
use futures::{future, sync::mpsc, Future};
use gtk::prelude::*;
use relm::Widget;
use relm_attributes::widget;
use tokio::runtime::Runtime;

#[derive(Msg)]
pub enum Message {
    Quit,
}

#[derive(Default)]
pub struct Model {}

#[widget]
impl Widget for MainWin {
    fn model(from_flubber: mpsc::UnboundedReceiver<Packet>) -> Model {
        Model::default()
    }

    fn update(&mut self, evt: Message) {
        match evt {
            Message::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "flubber",
            gtk::TextView {},
            delete_event(_, _) => (Message::Quit, Inhibit(false)),
        }
    }
}

fn run(from_flubber: mpsc::UnboundedReceiver<Packet>) -> impl Future<Item = (), Error = ()> {
    future::result(MainWin::run(from_flubber))
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let (from_flubber_tx, from_flubber_rx) = mpsc::unbounded();
    let client = Client::new();
    runtime.spawn(run(from_flubber_rx));
    runtime.spawn(client.run().map_err(|err| {
        eprintln!("client error: {}", err);
        eprintln!("{:?}", err.backtrace())
    }));
    runtime.shutdown_on_idle().wait().unwrap();
}
