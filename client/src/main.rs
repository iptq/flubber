#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use flubber::{Client, ErrorExt, Packet};
use futures::{future, sync::mpsc, Future};
use gtk::{prelude::* , Orientation::*};
use relm::{Widget, Relm};
use relm_attributes::widget;
use tokio::runtime::Runtime;

#[derive(Msg)]
pub enum Message {
    Quit,
}

pub struct Model {
    relm: Relm<MainWin>,
    buffer: gtk::TextBuffer,
}

#[widget]
impl Widget for MainWin {
    fn model(relm: &Relm<Self>, from_flubber: mpsc::UnboundedReceiver<Packet>) -> Model {
        Model {
            relm: relm.clone(),
            buffer: gtk::TextBuffer::new(None),
        }
    }

    fn update(&mut self, evt: Message) {
        match evt {
            Message::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "flubber",
            property_default_width: 854,
            property_default_height: 480,
            resizable: false,

            delete_event(_, _) => (Message::Quit, Inhibit(false)),

            gtk::Box {
                orientation: Vertical,

                gtk::ScrolledWindow {
                    gtk::TextView {
                        vexpand: true,
                        editable: false,
                        cursor_visible: false,
                        monospace: true,
                        buffer: Some(&self.model.buffer),
                    },
                },
                gtk::Entry {
                    placeholder_text: "type message",
                },
            },
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
