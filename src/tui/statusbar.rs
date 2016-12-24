use super::window::Windows;

use termion::{self, clear, color, cursor};

pub struct StatusBar {
}

impl StatusBar {
    pub fn new() -> StatusBar {
        StatusBar {
        }
    }

    pub fn draw(&self, windows: &Windows) {
        let (_, rows) = termion::terminal_size().unwrap();
        print!("{}{}", cursor::Hide, cursor::Goto(1, rows - 1));
        print!("{}", clear::CurrentLine);
        let cur_win_number = windows.current_window_number();
        let cur_win_name = windows.current_window().name();
        let highest_win = windows.highest_window_index();
        print!("[{}: {}] Highest window: {} [Act:",
            cur_win_number,
            cur_win_name,
            highest_win);
        for (index, activity) in windows.activity() {
            use super::window::ActivityLevel::*;
            match activity {
                Inactive => continue,
                Active => {
                    print!(" {}", index);
                }
                Hilight => {
                    print!("{}", color::Fg(color::Red));
                    print!(" {}", index);
                    print!("{}", color::Fg(color::Reset));
                }
            }
        }
        print!("]");
    }
}
