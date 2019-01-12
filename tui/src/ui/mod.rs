mod buffer;
mod input;

use std::collections::HashMap;
use std::io::{self, Write};

use flubber::{ClientMessage, Error};
use futures::{
    Future,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    sync::oneshot, future,
};
use termion::{
    self, color, cursor,
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
    style,
};
use uuid::Uuid;

use self::buffer::{Buffer};

pub struct GUI {
    buffer_list: HashMap<Uuid, Buffer>,
    active_buffer: Option<Uuid>,

    from_thread: UnboundedReceiver<ClientMessage>,
    to_thread: UnboundedSender<ClientMessage>,
    stop: Option<oneshot::Sender<()>>,

    terminal: MouseTerminal<AlternateScreen<RawTerminal<io::Stdout>>>,
    done: bool,
}

impl GUI {
    pub fn new(
        from_thread: UnboundedReceiver<ClientMessage>,
        to_thread: UnboundedSender<ClientMessage>,
        stop: oneshot::Sender<()>,
    ) -> GUI {
        let stdout = io::stdout();
        let raw_stdout = stdout.into_raw_mode().unwrap();

        let alt_screen = AlternateScreen::from(raw_stdout);
        let terminal = MouseTerminal::from(alt_screen);

        GUI {
            buffer_list: HashMap::new(),
            active_buffer: None,

            from_thread,
            to_thread,
            stop: Some(stop),

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
        let names_width = 10;
        let timename_width = 10;

        // number of available rows for messages
        let available_rows = rows - 3;
        let available_cols = cols - (buflist_width + 1);

        write!(self.terminal, "{}{}", cursor::Goto(1, 1), cursor::Hide)?;
        let mut row = 0;
        while row < rows {
            let mut col = 0;
            write!(self.terminal, "{}", style::Reset)?;
            while col < cols {
                if col == buflist_width + 1 {
                    write!(
                        self.terminal,
                        "{}\u{2502}{}",
                        color::Fg(color::Green),
                        style::Reset
                    )?;
                } else if (row == 0 || row == rows - 2) && col == buflist_width + 2 {
                    write!(self.terminal, "{}Flubber", color::Bg(color::Green))?;
                    col += 6;
                } else {
                    write!(self.terminal, " ")?;
                }
                col += 1;
            }
            row += 1;
        }

        // move the cursor to the correct location
        write!(
            self.terminal,
            "{}{}",
            cursor::Goto(buflist_width + 3, rows),
            cursor::Show
        )?;

        Ok(())
    }

    pub fn run(&mut self) -> impl Future<Item = (), Error = Error> {
        let stdin = io::stdin();

        let (cols, rows) = termion::terminal_size().unwrap();
        self.draw(rows, cols).unwrap();
        self.terminal.flush().unwrap();

        for event in stdin.events() {
            // terminal size
            let (cols, rows) = termion::terminal_size().unwrap();

            // update
            let event = event.unwrap();
            self.update(event);

            self.draw(rows, cols).unwrap();
            self.terminal.flush().unwrap();

            if self.done {
                let stop = self.stop.take().unwrap();
                stop.send(()).ok();
                break;
            }
        }
        future::ok(())
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        write!(self.terminal, "{}{}", style::Reset, cursor::Show).unwrap();
    }
}
