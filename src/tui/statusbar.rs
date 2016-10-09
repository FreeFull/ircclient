use ncurses::*;

use super::window::Windows;

pub struct StatusBar {
    window: WINDOW,
}

impl Drop for StatusBar {
    fn drop(&mut self) {
        delwin(self.window);
    }
}

impl StatusBar {
    pub fn new() -> StatusBar {
        let mut w = 0;
        let mut h = 0;
        getmaxyx(stdscr, &mut h, &mut w);
        let window = newwin(1, w, h-2, 0);
        StatusBar {
            window: window,
        }
    }

    pub fn draw(&self, windows: &Windows) {
        werase(self.window);
        let cur_win_number = windows.current_window_number();
        let cur_win_name = windows.current_window().name();
        let highest_win = windows.highest_window_index();
        let string =
            format!("[{}: {}] Highest window: {} ",
                   cur_win_number,
                   cur_win_name,
                   highest_win);
        waddstr(self.window, &string);
        waddstr(self.window, "[Act:");
        for (index, activity) in windows.activity() {
            use super::window::ActivityLevel::*;
            match activity {
                Inactive => continue,
                Active => {
                    waddstr(self.window, &format!(" {}", index));
                }
                Hilight => {
                    wcolor_set(self.window, 1);
                    waddstr(self.window, &format!(" {}", index));
                    wcolor_set(self.window, 0);
                }
            }
        }
        waddstr(self.window, "]");
        wnoutrefresh(self.window);
    }
}
