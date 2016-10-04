extern crate irc as irc_lib;
extern crate ncurses;

use std::sync::mpsc::channel;

mod tui;
mod irc;
mod event;
use tui::Tui;

fn main() {
    let (event_tx, event_rx) = channel();
    let (_irc_threads, irc_tx) = irc::start(event_tx).unwrap();
    let mut tui = Tui::new(event_rx, irc_tx);
    tui.event_loop();
}
