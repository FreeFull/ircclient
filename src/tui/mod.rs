mod entryline;
mod displayarea;
mod window;
mod statusbar;

use std::sync::mpsc::{Sender, Receiver};
use std::io::{self, Write, stdout};

use termion::raw::{IntoRawMode, RawTerminal};

use self::entryline::EntryLine;
use self::window::Windows;
use self::statusbar::StatusBar;

use event::{Event, EventReceiver};
use irc::command::Command;

pub struct Tui {
    entry_line: EntryLine,
    event_rx: Receiver<io::Result<Event>>,
    irc_tx: Sender<Command>,
    windows: Windows,
    statusbar: StatusBar,
    raw_stdout: RawTerminal<io::Stdout>,
    running: bool,
}

impl Tui {
    pub fn new(event_rx: EventReceiver, irc_tx: Sender<Command>) -> io::Result<Tui> {
        Ok(Tui {
            entry_line: EntryLine::new(),
            event_rx: event_rx,
            irc_tx: irc_tx,
            windows: Windows::new(),
            statusbar: StatusBar::new(),
            raw_stdout: stdout().into_raw_mode()?,
            running: true,
        })
    }

    pub fn event_loop(&mut self) {
        'main_loop:
        loop {
            if !self.running { break; }
            match self.event_rx.recv() {
                Ok(event) => {
                    let event = event.unwrap();
                    match event {
                        Event::Input(key) => {
                            if let Some(line) = self.entry_line.key_input(key) {
                                self.handle_line(line);
                            }
                        },
                        Event::Chat(event) => {
                            self.windows.handle_event(event);
                            self.redraw();
                        }
                    }
                }
                Err(_) => break 'main_loop,
            }
            self.redraw();
        }
    }

    fn redraw(&mut self) {
        self.statusbar.draw(&self.windows);
        self.entry_line.draw();
        self.raw_stdout.flush();
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
        let maybe_body = if body == "" { None } else { Some(body) };
        match command {
            "join" => self.irc_tx.send(Command::Join { channel: String::from(body) }).unwrap(),
            "part" => {
                use self::window::WindowId::*;
                let channel = match *self.windows.current_window().id() {
                    Channel { ref name, .. } => Some(name),
                    _ => None,
                };
                if let Some(channel) = channel {
                    self.irc_tx.send(
                        Command::Part {
                            channel: channel.clone(),
                            message: maybe_body.map(String::from)
                        }).unwrap();
                }
            }
            "query" => {
                // TODO: Display error message when body is empty
                self.windows.query(body).ok();
            }
            "quit" => {
                self.irc_tx.send(Command::Quit { message: maybe_body.map(String::from) }).unwrap();
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
