use std;

use ncurses::*;
use std::char;

pub struct EntryLine {
    window: WINDOW,
    string: String,
}

impl EntryLine {
    pub fn new() -> EntryLine {
        let mut w = 0;
        let mut h = 0;
        getmaxyx(stdscr(), &mut h, &mut w);
        let window = newwin(1, w, h - 1, 0);
        EntryLine {
            window: window,
            string: String::new(),
        }
    }

    pub fn key_input(&mut self, key: WchResult) -> Option<String> {
        use ncurses::WchResult::*;
        match key {
            KeyCode(code) => match code {
                KEY_ENTER => {
                    let mut string = String::new();
                    std::mem::swap(&mut string, &mut self.string);
                    return Some(string)
                }
                KEY_BACKSPACE => {
                    self.string.pop();
                }
                _ => {}
            },
            Char(ch) => match ch as i32 {
                // ENTER
                10 | 13 => return self.key_input(KeyCode(KEY_ENTER)),
                8 | 127 => return self.key_input(KeyCode(KEY_BACKSPACE)),
                _ => if let Some(c) = char::from_u32(ch) {
                    self.string.push(c);
                },
            },
        }
        None
    }

    pub fn draw(&self) {
        // TODO: Handle lines longer than the screen.
        werase(self.window);
        waddstr(self.window, &self.string);
        wcursyncup(self.window);
        wnoutrefresh(self.window);
    }
}
