use std::sync::mpsc as chan;
use event::Event;
use irc_lib::client::prelude::*;

pub mod irc_utils;
mod command;

// TODO
pub type Sender = chan::Sender<()>;

pub fn start(event_tx: chan::Sender<Event>, irc_rx: chan::Receiver<()>) {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        // TODO
    }
}
