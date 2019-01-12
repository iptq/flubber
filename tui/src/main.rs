extern crate flubber;
extern crate futures;
extern crate termion;

mod client;
mod gui;

use std::thread;

use crate::gui::GUI;
use futures::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::unbounded();

    let gui = GUI::new(tx);

    thread::spawn(move || client::run());
    gui.run();
}
