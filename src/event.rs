use chrono::{DateTime, Local};
use irc_lib::client::data::Message;
use termion::event::Key;

use std::sync::mpsc::{Sender, Receiver};
use std::io;

pub type EventSender = Sender<io::Result<Event>>;
pub type EventReceiver = Receiver<io::Result<Event>>;

pub enum Event {
    Input(Key),
    Chat(ChatEvent),
}

pub struct ChatEvent {
    pub about_self: bool,
    pub is_query: bool,
    pub message: Message,
    pub timestamp: DateTime<Local>,
}

impl ChatEvent {
    pub fn new(message: Message, about_self: bool, is_query: bool) -> ChatEvent {
        ChatEvent {
            about_self: about_self,
            is_query: is_query,
            message: message,
            timestamp: Local::now(),
        }
    }

    pub fn source_nickname(&self) -> Option<&str> {
        self.message.source_nickname()
    }
}
