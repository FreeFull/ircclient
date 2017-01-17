extern crate irc as irc_lib;
extern crate termion;
extern crate chrono;
extern crate rustyline;
extern crate unicode_width;

use std::sync::mpsc::channel;

mod tui;
mod irc;
mod event;
mod input;
use tui::Tui;

fn main() {
    let (event_tx, event_rx) = channel();
    let (_irc_threads, irc_tx) = irc::start(event_tx.clone()).unwrap();
    let _input_thread = input::start(event_tx);
    let mut tui = Tui::new(event_rx, irc_tx).unwrap();
    tui.event_loop();
}
