use std::sync::mpsc::*;
use event::ChatEvent;
use std::collections::HashMap;
use irc_lib::client::prelude::*;

pub mod irc_utils;
pub mod command;

struct IrcState {
    channels: HashMap<Vec<u8>, Channel>,
}

struct Channel {
    name: String,
    users: Vec<String>,
}

pub fn start(event_tx: Sender<ChatEvent>, irc_rx: Receiver<command::Command>) {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        unimplemented!();
    }
}
