use irc_lib::client::data::Message;

pub enum Command {
    Join(String),
    Part(String, Option<String>),
    PrivMsg(String, String),
    MessageReceived(Message),
}
