mod entryline;
mod displayarea;

use std;
use std::sync::mpsc::Receiver;

use ncurses::*;

use irc::client::prelude::*;

use self::entryline::EntryLine;
use self::displayarea::DisplayArea;

pub type MessageReceiver = Receiver<std::io::Result<Message>>;

pub struct Tui {
    display_area: DisplayArea,
    entry_line: EntryLine,
    message_rx: MessageReceiver,
    server: IrcServer,
    running: bool,
}

impl Drop for Tui {
    fn drop(&mut self) {
        endwin();
    }
}

impl Tui {
    pub fn new(message_rx: MessageReceiver, server: IrcServer) -> Tui {
        initscr();
        keypad(stdscr, true);
        raw();
        noecho();
        nonl();
        start_color();
        timeout(50);
        let display_area = DisplayArea::new();
        let entry_line = EntryLine::new();
        Tui {
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
            if let Some(ch) = get_wch() {
                if let Some(line) = self.entry_line.key_input(ch) {
                    self.handle_line(line);
                }
            }
            if !self.running { break; }
            loop {
                use std::sync::mpsc::TryRecvError::*;
                match self.message_rx.try_recv() {
                    Ok(message) => self.display_area.display_message(message.unwrap()),
                    Err(Empty) => break,
                    Err(Disconnected) => break 'main_loop,
                }
            }
            self.entry_line.draw();
            refresh();
        }
    }

    fn chat_target(&self) -> &str {
        // TODO: Don't just hardcode the channel!
        self.server.config().channels.as_ref().and_then(|x| x.first()).map(|x| &**x).unwrap_or("")
    }

    fn handle_line(&mut self, line: String) {
        if line.len() == 0 {
            return;
        }
        if &*line == "quit" {
            self.server.send_quit("Adios").unwrap();
            self.running = false;
            return;
        }
        use irc::client::data::Command::PRIVMSG;
        let target = self.chat_target();
        self.server.send_privmsg(target, &line).unwrap();
        let message = Message::from(PRIVMSG(String::from(target), line));
        self.display_area.display_message(message);
    }
}
