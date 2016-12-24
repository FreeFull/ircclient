use std;

use termion::event::Key;

pub struct EntryLine {
    string: String,
}

impl EntryLine {
    pub fn new() -> EntryLine {
        EntryLine {
            string: String::new(),
        }
    }

    pub fn key_input(&mut self, key: Key) -> Option<String> {
        use termion::event::Key::*;
        match key {
            Char('\n') => {
                let mut string = String::new();
                std::mem::swap(&mut string, &mut self.string);
                return Some(string)
            }
            Backspace => {
                self.string.pop();
            }
            Char(ch) => self.string.push(ch),
            _ => {}
        }
        None
    }

    pub fn draw(&self) {
        // TODO: Handle lines longer than the screen.
        use termion::{self, clear, cursor};
        let (_, max_y) = termion::terminal_size().unwrap();
        print!("{}{}", cursor::Goto(1, max_y), clear::AfterCursor);
        for ch in self.string.chars() {
            match ch {
                '\u{00}' => print!("\0"),
                '\u{01}'...'\u{1f}' => print!("^{:?}", ch),
                '\u{7f}' => print!("^?"),
                '\u{80}'...'\u{9f}' => print!("@{:?}", ch),
                _ => print!("{}", ch),
            }
        }
        print!("{}", cursor::Show);
    }
}
