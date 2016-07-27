extern crate irc as irc_lib;
extern crate ncurses;

use std::thread;
use std::sync::mpsc::channel;

mod tui;
mod irc;
mod event;
use tui::Tui;

fn main() {
    let (event_tx, event_rx) = channel();
    let (irc_tx, irc_rx) = channel();
    let irc_thread = thread::spawn({
        move || {
            irc::start(event_tx, irc_rx);
        }
    });
    let mut tui = Tui::new(event_rx, irc_tx);
    tui.event_loop();
    drop(tui);
    irc_thread.join().unwrap();
}
