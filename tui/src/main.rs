extern crate flubber;
extern crate futures;
extern crate termion;

mod client;
mod ui;

use std::thread;

use crate::ui::GUI;
use futures::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::unbounded();
    let mut gui = GUI::new(tx);

    thread::spawn(move || client::run());
    gui.run();
}
