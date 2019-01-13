mod buffer;
mod input;
mod util;

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use flubber::{ClientMessage, Error};
use futures::{
    future::{self, Either},
    stream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    sync::oneshot,
    Future, Stream,
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

use self::buffer::Buffer;
use self::util::make_padding;

struct State {
    last_typed: String,
    buffer_list: HashMap<Uuid, Buffer>,
    stop: Option<oneshot::Sender<()>>,
}

impl State {
    pub fn new(stop: oneshot::Sender<()>) -> State {
        State {
            last_typed: "a".to_owned(),
            buffer_list: HashMap::new(),
            stop: Some(stop),
        }
    }
}

pub struct GUI {
    active_buffer: Option<Uuid>,
    root_buffer: Option<Uuid>,

    from_thread: Option<UnboundedReceiver<ClientMessage>>,
    to_thread: UnboundedSender<ClientMessage>,

    terminal: Arc<Mutex<MouseTerminal<AlternateScreen<RawTerminal<io::Stdout>>>>>,
    state: Arc<Mutex<State>>,
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
            active_buffer: None,
            root_buffer: None,

            from_thread: Some(from_thread),
            to_thread,

            terminal: Arc::new(Mutex::new(terminal)),
            state: Arc::new(Mutex::new(State::new(stop))),
            done: false,
        }
    }

    fn update(&mut self, event: Event) {
        // println!("evt: {:?}", event);
    }

    fn draw(&mut self, rows: u16, cols: u16) -> Result<(), Error> {
        let mut terminal = self.terminal.lock().unwrap();

        let topic = format!("flubber v{}", flubber::VERSION);
        let bottom_bar = format!("scrollback (N lines)");

        // draw the buffer list
        // TODO: determine width of buffer list
        let buflist_width = 10;
        let names_width = 10;
        let timename_width = 10;

        // number of available rows for messages
        let available_rows = rows - 3;
        let available_cols = cols - (buflist_width + 1);
        let mut lines = Vec::<String>::with_capacity(available_rows as usize);

        write!(terminal, "{}{}", cursor::Goto(1, 1), cursor::Hide)?;
        let mut row = 0;
        while row < rows {
            let mut col = 0;
            write!(terminal, "{}", style::Reset)?;
            while col < cols {
                if col == buflist_width + 1 {
                    write!(
                        terminal,
                        "{}{}{}\u{2502}{}",
                        cursor::Goto(col + 1, row + 1),
                        color::Fg(color::Green),
                        color::Bg(color::Reset),
                        style::Reset
                    )?;
                } else if row == 0 && col == buflist_width + 2 {
                    // write the topic at the top bar
                    let padding = make_padding(' ', available_cols - topic.len() as u16 - 1);
                    write!(
                        terminal,
                        "{}{}{}{}{}",
                        cursor::Goto(col + 1, row + 1),
                        color::Bg(color::Green),
                        color::Fg(color::Black),
                        topic,
                        padding
                    )?;
                    col += topic.len() as u16 - 1;
                } else if row == rows - 2 && col == buflist_width + 2 {
                    // write the bottom bar contents
                    let padding = make_padding(' ', available_cols - bottom_bar.len() as u16 - 1);
                    write!(
                        terminal,
                        "{}{}{}{}{}",
                        cursor::Goto(col + 1, row + 1),
                        color::Bg(color::Green),
                        color::Fg(color::Black),
                        bottom_bar,
                        padding
                    )?;
                    col += bottom_bar.len() as u16 - 1;
                }
                col += 1;
            }
            row += 1;
        }

        // move the cursor to the correct location
        write!(
            terminal,
            "{}{}",
            cursor::Goto(buflist_width + 3, rows),
            cursor::Show
        )?;

        Ok(())
    }

    pub fn run(&mut self) -> impl Future<Item = (), Error = Error> {
        let terminal_events = {
            let stdin = io::stdin();
            stream::iter_ok::<_, Error>(stdin.events()).map(|item| Either::A(item))
        };
        let server_events = {
            let from_thread = self.from_thread.take().unwrap();
            from_thread
                .map(|item| Either::B(item))
                .map_err(|_| unreachable!())
        };
        let event_stream = terminal_events.select(server_events);

        let (cols, rows) = termion::terminal_size().unwrap();
        self.draw(rows, cols).unwrap();
        {
            let mut terminal = self.terminal.lock().unwrap();
            terminal.flush().unwrap();
        }

        let state = self.state.clone();
        let terminal = self.terminal.clone();
        event_stream.for_each(move |event| {
            // perform updates
            {
                let mut state2 = state.lock().unwrap();
                match event {
                    Either::A(terminal_event) => {
                        update_event(&mut *state2, terminal_event.unwrap())
                    }
                    Either::B(server_event) => update_message(&mut *state2, server_event),
                }
            }

            // draw screen and flush
            draw();
            {
                let mut terminal = terminal.lock().unwrap();
                terminal.flush().unwrap();
            }

            future::ok(())
        })

        //     for event in stdin.events() {
        //         // terminal size
        //         let (cols, rows) = termion::terminal_size().unwrap();

        //         // update
        //         let event = event.unwrap();
        //         self.update(event);

        //         self.draw(rows, cols).unwrap();
        //         terminal.flush().unwrap();

        //         if self.done {
        //             let stop = self.stop.take().unwrap();
        //             stop.send(()).ok();
        //             break;
        //         }
        //     }
        // future::ok(())
    }

    pub fn restore(&self) {
        let mut terminal = self.terminal.lock().unwrap();
        write!(terminal, "{}{}", style::Reset, cursor::Show).unwrap();
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        self.restore();
    }
}

fn update_event(state: &mut State, event: Event) {
    let (cols, rows) = termion::terminal_size().unwrap();
    match event {
        Event::Key(Key::Esc) => {
            let stop = state.stop.take().unwrap();
            stop.send(()).ok();
        }
        _ => (),
    }
}

fn update_message(state: &mut State, message: ClientMessage) {}

fn draw() {
    let (cols, rows) = termion::terminal_size().unwrap();
}
