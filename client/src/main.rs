#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use flubber::{Client, ErrorExt};
use futures::{Future, future};
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
    fn model() -> Model {
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

fn run() -> impl Future<Item = (), Error = ()> {
    future::result(MainWin::run(()))
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let client = Client::new();
    runtime.spawn(run());
    runtime.spawn(client.run().map_err(|err| {
        eprintln!("daemon error: {}", err);
        eprintln!("{:?}", err.backtrace())
    }));
    runtime.shutdown_on_idle().wait().unwrap();
}
