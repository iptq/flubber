use std::io::{self, Write};

use flubber::ClientMessage;
use futures::sync::mpsc;
use termion::{
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
};

pub struct GUI {}

impl GUI {
    pub fn new(_: mpsc::UnboundedSender<ClientMessage>) -> GUI {
        GUI {}
    }

    pub fn run(&self) {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let raw_stdout = stdout.into_raw_mode().unwrap();

        let mut terminal = MouseTerminal::from(raw_stdout);

        for event in stdin.events() {
            let event = event.unwrap();
            println!("evt: {:?}", event);
            match event {
                Event::Key(Key::Ctrl('c')) => break,
                _ => (),
            }
            terminal.flush().unwrap();
        }
    }
}
