#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use flubber::{
    proto::{Packet, PacketId},
    Client, ErrorExt,
};
use futures::{future, sync::mpsc, Future, Stream};
use gtk::{prelude::*, Orientation::*};
use relm::{Component, Relm, Widget};
use ref_thread_local::{ref_thread_local, RefThreadLocal};
use relm_attributes::widget;
use tokio::runtime::Runtime;

#[derive(Msg)]
pub enum Message {
    SendMessage(String),
    Received(Packet),
    Quit,
}

pub struct Model {
    relm: Relm<MainWin>,
    buffer: gtk::TextBuffer,
    sequence: u32,
    to_flubber: mpsc::UnboundedSender<Packet>,
}

#[widget]
impl Widget for MainWin {
    fn model(relm: &Relm<Self>, to_flubber: mpsc::UnboundedSender<Packet>) -> Model {
        Model {
            relm: relm.clone(),
            buffer: gtk::TextBuffer::new(None),
            sequence: 0,
            to_flubber,
        }
    }

    fn update(&mut self, evt: Message) {
        match evt {
            Message::SendMessage(contents) => {
                use flubber::proto::{packet::Kind, plugin::PluginIncomingMessage};
                let new_message = PluginIncomingMessage {
                    timestamp: 17,
                    author: "me".to_owned(),
                    contents: contents,
                };
                let kind = Kind::PluginIncomingMessage(new_message);
                self.model.sequence += 1;
                let packet = Packet {
                    id: Some(PacketId {
                        origin: 1,
                        sequence: self.model.sequence,
                    }),
                    kind: Some(kind),
                };
                self.model.to_flubber.unbounded_send(packet).unwrap();
            }
            Message::Received(packet) => {
                eprintln!("received packet: {:?}", packet);
            }
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

                    activate(entry) => {
                        entry.get_text().map(|text| Message::SendMessage(text))
                    }
                },
            },
        }
    }
}

struct App(pub Component<MainWin>, pub Option<mpsc::UnboundedReceiver<Packet>>);

impl App {
    pub fn new() -> Self {
        let (to_flubber_tx, to_flubber_rx) = mpsc::unbounded();
        let inner = relm::init::<MainWin>(to_flubber_tx).unwrap();
        App(inner, Some(to_flubber_rx))
    }

    pub fn emit(&self, msg: Message) {
        self.0.emit(msg);
    }
}

fn run(to_flubber: mpsc::UnboundedSender<Packet>) -> impl Future<Item = (), Error = ()> {
    future::result(MainWin::run(to_flubber))
}

ref_thread_local! {
    static managed APP: App = {
        gtk::init().unwrap();
        App::new()
    };
}

fn main() {
    gtk::init().unwrap();
    let mut runtime = Runtime::new().unwrap();
    let (from_flubber_tx, from_flubber_rx) = mpsc::unbounded();

    let to_flubber_rx = {
        let mut app = APP.borrow_mut();
        app.1.take().unwrap()
    };
    let client = Client::new(to_flubber_rx, from_flubber_tx);

    runtime.spawn(future::ok(gtk::main()));
    runtime.spawn(from_flubber_rx.for_each(|packet| {
        let app = APP.borrow();
        app.emit(Message::Received(packet));
        future::ok(())
    }));

    // runtime.spawn(run(to_flubber_tx, from_flubber_rx));
    runtime.spawn(client.run().map_err(|err| {
        eprintln!("client error: {}", err);
        eprintln!("{:?}", err.backtrace());
    }));

    runtime.shutdown_on_idle().wait().unwrap();
}
