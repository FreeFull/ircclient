use irc_lib::client::data::Message;

pub enum Command {
    Join {
        channel: String,
    },
    Part {
        channel: String,
        message: Option<String>,
    },
    PrivMsg {
        target: String,
        message: String,
    },
    Quit {
        message: Option<String>,
    },
    MessageReceived(Message),
}
