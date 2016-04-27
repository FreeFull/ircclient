mod entryline;
mod displayarea;

use std;
use std::sync::mpsc::Receiver;

use termbox::*;
use termbox_sys;

use irc::client::prelude::*;

use self::entryline::EntryLine;
use self::displayarea::DisplayArea;

pub type MessageReceiver = Receiver<std::io::Result<Message>>;

pub struct Tui {
    termbox: Termbox,
    display_area: DisplayArea,
    entry_line: EntryLine,
    message_rx: MessageReceiver,
    server: IrcServer,
    running: bool,
}

impl Tui {
    pub fn new(message_rx: MessageReceiver, server: IrcServer) -> Tui {
        let mut termbox = Termbox::open().unwrap();
        termbox.set_clear_attributes(BLACK, BLACK);
        termbox.clear();
        let display_area = DisplayArea::new();
        let entry_line = EntryLine::new();
        Tui {
            termbox: termbox,
            display_area: display_area,
            entry_line: entry_line,
            message_rx: message_rx,
            server: server,
            running: true,
        }
    }

    pub fn event_loop(&mut self) {
        'main_loop:
        loop {
            if let Some(event) = self.termbox.peek_event(50) {
                use termbox::Event::*;
                match event {
                    Key(key) => {
                        if let Some(line) = self.entry_line.key_input(key) {
                            self.handle_line(line);
                        }
                    }
                    Resize(_size) => {}
                    Mouse(_mouse) => {}
                }
            }
            if !self.running { break; }
            loop {
                use std::sync::mpsc::TryRecvError::*;
                match self.message_rx.try_recv() {
                    Ok(message) => self.display_area.display_message(&mut self.termbox, message.unwrap()),
                    Err(Empty) => break,
                    Err(Disconnected) => break 'main_loop,
                }
            }
            self.entry_line.draw(&mut self.termbox);
            self.termbox.present();
        }
    }

    fn handle_line(&mut self, line: String) {
        if &*line == "quit" {
            self.server.send_quit("Adios").unwrap();
            self.running = false;
            return;
        }
        self.display_area.display_string(&mut self.termbox, &line);
        // TODO: Don't just hardcode the channel!
        self.server.send_privmsg("#rust-offtopic", &line).unwrap();
    }
}

fn shift_up(termbox: &mut Termbox) {
    let width = termbox.width() as usize;
    let height = termbox.height() as usize;
    {
        let output_buffer = termbox.cell_buffer_mut();
        for i in 0..(width * (height-2)) {
            output_buffer[i] = output_buffer[i+width];
        }
    }
    clear_line(termbox, height-2);
}

fn clear_line(termbox: &mut Termbox, line: usize) {
    let width = termbox.width() as usize;
    let output_buffer = termbox.cell_buffer_mut();
    for i in (width * line)..(width * (line+1)) {
        output_buffer[i] =
            termbox_sys::RawCell {
                ch: ' ' as u32,
                bg: BLACK,
                fg: WHITE,
            };
    }
}
