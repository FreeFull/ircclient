use ncurses::*;

use irc::client::prelude::*;

pub struct DisplayArea {
    window: WINDOW,
}

impl DisplayArea {
    pub fn new() -> DisplayArea {
        let mut w = 0;
        let mut h = 0;
        getmaxyx(stdscr, &mut h, &mut w);
        let window = subwin(stdscr, h - 2, w, 0, 0);
        scrollok(window, true);
        syncok(window, true);
        DisplayArea {
            window: window,
        }
    }

    pub fn display_message(&self, message: Message) {
        use irc::client::data::Command::*;
        let from = message.source_nickname().unwrap_or("");
        let message = match message.command {
            PRIVMSG(ref target, ref msg) => format!("{} <{}> {}", target, from, msg),
            JOIN(ref chanlist, _, _) => format!("{} has joined {}", from, chanlist),
            NICK(ref nick) => format!("{} is now known as {}", from, nick),
            _ => return,
        };
        waddstr(self.window, &message);
        waddch(self.window, '\n' as u64);
    }
}
