use ncurses::*;

use irc::client::prelude::*;

pub struct DisplayArea {
    window: WINDOW,
}

impl Drop for DisplayArea {
    fn drop(&mut self) {
        delwin(self.window);
    }
}

impl DisplayArea {
    pub fn new() -> DisplayArea {
        let mut w = 0;
        let mut h = 0;
        getmaxyx(stdscr, &mut h, &mut w);
        let window = newwin(h - 2, w, 0, 0);
        scrollok(window, true);
        DisplayArea {
            window: window,
        }
    }

    pub fn add_message(&self, message: Message) {
        use irc::client::data::Command::*;
        let from = message.source_nickname().unwrap_or("");
        let message = match message.command {
            PRIVMSG(ref target, ref msg) => format!("{} <{}> {}", target, from, msg),
            JOIN(ref chanlist, _, _) => format!("{} has joined {}", from, chanlist),
            NICK(ref nick) => format!("{} is now known as {}", from, nick),
            _ => return,
        };
        if (getcury(self.window), getcurx(self.window)) != (0, 0) {
            waddch(self.window, '\n' as u64);
        }
        waddstr(self.window, &message);
    }

    pub fn draw(&self) {
        wnoutrefresh(self.window);
    }
}
