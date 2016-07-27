use ncurses::*;

use event::ChatEvent;

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

    pub fn show_event(&self, event: ChatEvent) {
        use event::ChatEventKind::*;
        let from = event.source_nickname().unwrap_or("");
        let message = match event.event {
            RoomMsg(ref room, ref msg) => format!("{} <{}> {}", room, from, msg),
            Join(ref channel) => format!("{} has joined {}", from, channel),
            NickChange(ref new_nick) => format!("{} is now known as {}", from, new_nick),
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
