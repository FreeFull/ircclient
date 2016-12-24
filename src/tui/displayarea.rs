use event::ChatEvent;

use std::collections::VecDeque;
use std::cell::RefCell;

use termion::{self, cursor, clear};

pub struct DisplayArea {
    messages: Messages,
}

impl DisplayArea {
    pub fn new() -> DisplayArea {
        DisplayArea {
            messages: Messages::with_max(100),
        }
    }

    pub fn show_event(&self, event: &ChatEvent) {
        use irc_lib::client::data::Command::*;
        let from = event.source_nickname().unwrap_or("");
        let message = match event.message.command {
            PRIVMSG(ref target, ref msg) => format!("{} <{}> {}", target, from, msg),
            NOTICE(ref target, ref msg) => format!("!{} <{}> {}", target, from, msg),
            JOIN(ref channel, _, _) => format!("{} has joined {}", from, channel),
            NICK(ref new_nick) => format!("{} is now known as {}", from, new_nick),
            _ => format!("{}", event.message),
        };
        self.messages.add_message(message);
    }

    pub fn add_message<S: Into<String>>(&self, message: S) {
        let message = message.into();
        self.messages.add_message(message.into());
    }

    pub fn self_message(&self, message: &str) {
        self.add_message(format!("<> {}", message));
    }

    pub fn draw_last_message(&self) {
        let messages = self.messages.storage.borrow();
        if let Some(message) = messages.back() {
            let (_, max_y) = termion::terminal_size().unwrap();
            print!("{}{}", cursor::Goto(1, max_y - 1), clear::AfterCursor);
            print!("{}\n\n", message);
        }
    }

    pub fn redraw(&self) {
        let (_, max_y) = termion::terminal_size().unwrap();
        print!("{}{}", clear::All, cursor::Goto(1, max_y));
        self.messages.for_all(|message| print!("{}\r\n", message));
        print!("\n");
    }
}

struct Messages {
    max_len: usize,
    storage: RefCell<VecDeque<String>>,
}

impl Messages {
    fn with_max(max_len: usize) -> Messages {
        Messages {
            max_len: max_len,
            storage: RefCell::new(VecDeque::with_capacity(100)),
        }
    }

    fn add_message(&self, message: String) {
        let mut storage = self.storage.borrow_mut();
        while storage.len() >= self.max_len {
            storage.pop_front();
        }
        storage.push_back(message);
    }

    fn for_all<F: FnMut(&str)>(&self, mut closure: F) {
        let storage = self.storage.borrow();
        for message in storage.iter() {
            closure(message);
        }
    }
}
