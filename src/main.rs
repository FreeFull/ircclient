extern crate irc;
extern crate termbox;
extern crate termbox_sys;

use std::thread;
use std::sync::mpsc::channel;

use irc::client::prelude::*;

mod tui;
use tui::Tui;

mod irc_state;

fn main() {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    let (message_tx, message_rx) = channel();
    let message_thread = thread::spawn({
        let message_tx = message_tx.clone();
        let server = server.clone();
        move || {
            for message in server.iter() {
                if message_tx.send(message).is_err() {
                    break;
                }
            }
        }
    });
    let mut tui = Tui::new(message_rx, server);
    tui.event_loop();
    drop(tui);
    message_thread.join().unwrap();
}
