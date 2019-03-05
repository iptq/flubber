#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use flubber::{
    proto::{Packet, PacketId},
    Client, ErrorExt,
};
use futures::{future, sync::mpsc, Future};
use gtk::{prelude::*, Orientation::*};
use relm::{Relm, Widget};
use relm_attributes::widget;
use tokio::runtime::Runtime;

#[derive(Msg)]
pub enum Message {
    SendMessage(String),
    Quit,
}

pub struct Model {
    relm: Relm<MainWin>,
    buffer: gtk::TextBuffer,
    sequence: u32,
    to_flubber: mpsc::UnboundedSender<Packet>,
    from_flubber: mpsc::UnboundedReceiver<Packet>,
}

#[widget]
impl Widget for MainWin {
    fn model(
        relm: &Relm<Self>,
        (to_flubber, from_flubber): (
            mpsc::UnboundedSender<Packet>,
            mpsc::UnboundedReceiver<Packet>,
        ),
    ) -> Model {
        Model {
            relm: relm.clone(),
            buffer: gtk::TextBuffer::new(None),
            sequence: 0,
            to_flubber,
            from_flubber,
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
                self.model.to_flubber.send(packet).unwrap();
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

fn run(
    to_flubber: mpsc::UnboundedSender<Packet>,
    from_flubber: mpsc::UnboundedReceiver<Packet>,
) -> impl Future<Item = (), Error = ()> {
    future::result(MainWin::run((to_flubber, from_flubber)))
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let (to_flubber_tx, to_flubber_rx) = mpsc::unbounded();
    let (from_flubber_tx, from_flubber_rx) = mpsc::unbounded();
    let client = Client::new();
    runtime.spawn(run(to_flubber_tx, from_flubber_rx));
    runtime.spawn(client.run().map_err(|err| {
        eprintln!("client error: {}", err);
        eprintln!("{:?}", err.backtrace())
    }));
    runtime.shutdown_on_idle().wait().unwrap();
}
