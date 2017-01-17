use std;

use rustyline::line_buffer::LineBuffer;
use unicode_width::UnicodeWidthStr;

use termion::event::Key;

pub struct EntryLine {
    string: LineBuffer,
}

impl EntryLine {
    pub fn new() -> EntryLine {
        EntryLine {
            string: LineBuffer::with_capacity(512),
        }
    }

    pub fn key_input(&mut self, key: Key) -> Option<String> {
        use termion::event::Key::*;
        match key {
            Char('\n') => {
                let mut string = LineBuffer::with_capacity(512);
                std::mem::swap(&mut string, &mut self.string);
                return Some(string.into_string())
            }
            Backspace => {
                self.string.backspace();
            }
            Delete => {
                self.string.delete();
            }
            Left => {
                self.string.move_left();
            }
            Right => {
                self.string.move_right();
            }
            Home => {
                self.string.move_home();
            }
            End => {
                self.string.move_end();
            }
            Ctrl('w') => {
                self.string.delete_word();
            }
            Ctrl('u') => {
                self.string.update("", 0);
            }
            Char(ch) => {
                self.string.insert(ch);
            }
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
        let cursor_pos = self.string[..self.string.pos()].width() as u16 + 1;
        print!("{}{}", cursor::Goto(cursor_pos, max_y), cursor::Show);
    }
}
