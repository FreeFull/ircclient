use irc_lib::client::data::Message;

pub struct ChatEvent {
    pub about_self: bool,
    pub message: Message,
}

impl ChatEvent {
    pub fn source_nickname(&self) -> Option<&str> {
        self.message.source_nickname()
    }
}
