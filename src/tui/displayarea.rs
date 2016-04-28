use termbox::*;

use irc::client::prelude::*;

use super::shift_up;

pub struct DisplayArea {
}

impl DisplayArea {
    pub fn new() -> DisplayArea {
        DisplayArea {
        }
    }

    pub fn display_message(&self, termbox: &mut Termbox, message: Message) {
        use irc::client::data::Command::*;
        let from = message.source_nickname().unwrap_or("");
        let message = match message.command {
            PRIVMSG(ref target, ref msg) => format!("{} <{}> {}", target, from, msg),
            JOIN(ref chanlist, _, _) => format!("JOIN {}", chanlist),
            NICK(ref nick) => format!("NICK {}", nick),
            _ => return,
        };
        self.display_string(termbox, &message);
    }

    pub fn display_string(&self, termbox: &mut Termbox, string: &str) {
        // TODO: display message properly
        let width = termbox.width();
        let height = termbox.height();
        shift_up(termbox, 0, 0, width as usize, height as usize - 1, 1);
        termbox.put_str(0, height-2, string, WHITE, BLACK);
    }
}
