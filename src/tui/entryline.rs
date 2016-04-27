use std;

use termbox;
use termbox::*;
use termbox_sys;

use super::clear_line;

pub struct EntryLine {
    display_text: Vec<Cell>,
    string: String,
}

impl EntryLine {
    pub fn new() -> EntryLine {
        EntryLine {
            display_text: Vec::new(),
            string: String::new(),
        }
    }

    pub fn key_input(&mut self, key_event: termbox::KeyEvent) -> Option<String> {
        let termbox::KeyEvent { key, .. } = key_event;
        match key {
            KEY_ENTER => {
                self.display_text.truncate(0);
                let mut string = String::new();
                std::mem::swap(&mut string, &mut self.string);
                return Some(string)
            }
            KEY_BACKSPACE | KEY_BACKSPACE2 => {
                self.display_text.pop();
                self.string.pop();
            }
            KEY_SPACE => {
                self.string.push(' ');
                self.display_text.push(
                    termbox_sys::RawCell {
                        ch: ' ' as u32,
                        bg: BLACK,
                        fg: WHITE,
                    }
                );
            }
            _ => {
                if let Some(ch) = key_event.ch {
                    self.string.push(ch);
                    self.display_text.push(
                        termbox_sys::RawCell {
                            ch: ch as u32,
                            bg: BLACK,
                            fg: WHITE,
                        }
                    );
                }
            }
        }
        None
    }

    pub fn draw(&self, termbox: &mut Termbox) {
        let w = termbox.width() as usize;
        let h = termbox.height();
        let len = self.display_text.len();
        let slice = if len < w { &self.display_text[..] } else { &self.display_text[len - w..] };
        clear_line(termbox, h as usize - 1);
        if self.display_text.len() == 0 {
            return;
        }
        termbox.blit(0, h-1, slice.len() as i32, 1, slice);
    }
}
