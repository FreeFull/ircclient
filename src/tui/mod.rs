mod entryline;
mod displayarea;
mod window;

use std;
use std::sync::mpsc::{Sender, Receiver};

use ncurses::*;

use self::entryline::EntryLine;
use self::displayarea::DisplayArea;
use self::window::{Windows, Window};

use event::Event;
use irc;

pub struct Tui {
    entry_line: EntryLine,
    event_rx: Receiver<Event>,
    irc_tx: irc::Sender,
    windows: Windows,
    running: bool,
}

impl Drop for Tui {
    fn drop(&mut self) {
        endwin();
    }
}

impl Tui {
    pub fn new(event_rx: Receiver<Event>, irc_tx: irc::Sender) -> Tui {
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
            event_rx: event_rx,
            irc_tx: irc_tx,
            windows: Windows::new(),
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
                match self.event_rx.try_recv() {
                    Ok(event) => self.windows.handle_event(event),
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
            unimplemented!();
        } else {
            // TODO: Show error
        }
    }

    fn handle_command(&mut self, command: &str, body: &str) {
        match command {
            "join" => unimplemented!(),
            "part" => {
                if unimplemented!() {
                    unimplemented!();
                }
            },
            "quit" => {
                self.irc_tx.send(unimplemented!()).unwrap();
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
