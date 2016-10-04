use chrono::{DateTime, Local};
use irc_lib::client::data::Message;

pub struct ChatEvent {
    pub about_self: bool,
    pub message: Message,
    pub timestamp: DateTime<Local>,
}

impl ChatEvent {
    pub fn new(message: Message, about_self: bool) -> ChatEvent {
        ChatEvent {
            message: message,
            about_self: about_self,
            timestamp: Local::now(),
        }
    }

    pub fn source_nickname(&self) -> Option<&str> {
        self.message.source_nickname()
    }
}
