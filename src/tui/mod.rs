mod entryline;
mod displayarea;
mod window;

use std;
use std::sync::mpsc::Receiver;

use ncurses::*;

use irc::client::prelude::*;

use self::entryline::EntryLine;
use self::displayarea::DisplayArea;
use self::window::{Windows, Window};

pub type MessageReceiver = Receiver<std::io::Result<Message>>;

pub struct Tui {
    entry_line: EntryLine,
    message_rx: MessageReceiver,
    server: IrcServer,
    windows: Windows,
    running: bool,
}

impl Drop for Tui {
    fn drop(&mut self) {
        endwin();
    }
}

impl Tui {
    pub fn new(message_rx: MessageReceiver, server: IrcServer) -> Tui {
        setlocale(LcCategory::all, "");
        initscr();
        keypad(stdscr, true);
        raw();
        noecho();
        nonl();
        start_color();
        timeout(50);
        let entry_line = EntryLine::new();
        Tui {
            entry_line: entry_line,
            message_rx: message_rx,
            server: server.clone(),
            windows: Windows::new(server),
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
                    Ok(message) => self.windows.handle_message(message.unwrap()),
                    Err(Empty) => break,
                    Err(Disconnected) => break 'main_loop,
                }
            }
            self.windows.draw();
            self.entry_line.draw();
            doupdate();
        }
    }

    fn handle_line(&mut self, line: String) {
        if line.len() == 0 {
            return;
        }
        if line.chars().nth(0) == Some('/') {
            let mut line = line[1..].splitn(1, ' ');
            let command = line.next().unwrap_or("");
            let body = line.next().unwrap_or("");
            self.handle_command(command, body);
            return;
        }
        if let Some(target) = self.windows.current_target() {
            self.server.send_privmsg(target.name(), &line);
        } else {
            // TODO: Show error
        }
    }

    fn handle_command(&mut self, command: &str, body: &str) {
        match command {
            "join" => self.server.send_join(body).unwrap(),
            "part" => {
                if let Some(name) = self.windows.current_channel() {
                    self.server.send(Command::PART(String::from(name), Some(String::from(body)))).unwrap();
                }
            },
            "quit" => {
                self.server.send_quit(body).unwrap();
                self.running = false;
            },
            "win" | "w" => {
                if let Some(number) = body.parse::<usize>().ok() {
                    self.windows.change_to(number);
                }
            }
            _ => {} // TODO: Handle unknown command
        }
    }
}
