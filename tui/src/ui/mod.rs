use std::io::{self, Write};

use flubber::{ClientMessage, Error};
use futures::sync::mpsc;
use termion::{
    self, color, cursor,
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
    style,
};

pub struct GUI {
    terminal: MouseTerminal<AlternateScreen<RawTerminal<io::Stdout>>>,
    done: bool,
}

impl GUI {
    pub fn new(_: mpsc::UnboundedSender<ClientMessage>) -> GUI {
        let stdout = io::stdout();
        let raw_stdout = stdout.into_raw_mode().unwrap();

        let alt_screen = AlternateScreen::from(raw_stdout);
        let terminal = MouseTerminal::from(alt_screen);

        GUI {
            terminal,
            done: false,
        }
    }

    fn update(&mut self, event: Event) {
        // println!("evt: {:?}", event);
        match event {
            Event::Key(Key::Esc) => self.done = true,
            _ => (),
        }
    }

    fn draw(&mut self, rows: u16, cols: u16) -> Result<(), Error> {
        // draw the buffer list
        // TODO: determine width of buffer list
        let buflist_width = 10;

        write!(self.terminal, "{}{}", cursor::Goto(1, 1), cursor::Hide)?;
        for row in 0..rows {
            write!(self.terminal, "{}", style::Reset)?;
            for col in 0..cols {
                if col == buflist_width + 1 {
                    write!(
                        self.terminal,
                        "{}\u{2502}{}",
                        color::Fg(color::Green),
                        style::Reset
                    )?;
                    continue;
                } else if row == 0 && col > buflist_width + 1 {
                    write!(self.terminal, "{} ", color::Bg(color::Green))?;
                } else {
                    write!(self.terminal, " ")?;
                }
            }
        }

        // move the cursor to the correct location
        write!(self.terminal, "{}", cursor::Goto(1, 1))?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let stdin = io::stdin();

        let (cols, rows) = termion::terminal_size().unwrap();
        self.draw(rows, cols)?;
        self.terminal.flush().unwrap();

        for event in stdin.events() {
            // terminal size
            let (cols, rows) = termion::terminal_size().unwrap();

            // update
            let event = event.unwrap();
            self.update(event);

            self.draw(rows, cols)?;
            self.terminal.flush()?;

            if self.done {
                break;
            }
        }

        write!(self.terminal, "{}", cursor::Show)?;
        Ok(())
    }
}
