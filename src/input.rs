use std::thread::{self, JoinHandle};
use std::io;
use event::{Event, EventSender};

pub fn start(event_tx: EventSender) -> JoinHandle<()> {
    thread::spawn(
        move || {
            use termion::input::TermRead;
            for event in io::stdin().keys() {
                let event = event.map(Event::Input);
                if event_tx.send(event).is_err() {
                    break;
                }
            }
        }
    )
}
