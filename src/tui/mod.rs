mod entryline;
mod displayarea;
pub mod window;

use std::sync::mpsc::{Sender, Receiver};

use ncurses::*;

use self::entryline::EntryLine;
use self::window::Windows;

use event::ChatEvent;
use irc::command::Command;

pub struct Tui {
    entry_line: EntryLine,
    event_rx: Receiver<ChatEvent>,
    irc_tx: Sender<Command>,
    windows: Windows,
    running: bool,
}

impl Drop for Tui {
    fn drop(&mut self) {
        endwin();
    }
}

impl Tui {
    pub fn new(event_rx: Receiver<ChatEvent>, irc_tx: Sender<Command>) -> Tui {
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
            let mut line = line[1..].splitn(2, ' ');
            let command = line.next().unwrap_or("");
            let body = line.next().unwrap_or("");
            self.handle_command(command, body);
            return;
        }
        if let Some(target) = self.windows.current_target() {
            target.self_message(&line);
            let target = String::from(target.id().name().expect("tui::handle_line target not found"));
            self.irc_tx.send(Command::PrivMsg { target: target, message: line }).unwrap();
        } else {
            // TODO: Show error
        }
    }

    fn handle_command(&mut self, command: &str, body: &str) {
        match command {
            "join" => self.irc_tx.send(Command::Join { channel: String::from(body) }).unwrap(),
            "part" => {
                use self::window::WindowId::*;
                let channel = match *self.windows.current_window().id() {
                    Channel { ref name, .. } => Some(name),
                    _ => None,
                };
                if let Some(channel) = channel {
                    self.irc_tx.send(Command::Part { channel: channel.clone(), message: Some(String::from(body)) }).unwrap();
                }
            }
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
