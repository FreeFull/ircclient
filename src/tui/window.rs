use super::displayarea::DisplayArea;

use event;
use irc::misc::irc_equal;

#[derive(Clone)]
pub enum WindowId {
    Channel {
        name: String,
    },
    Query {
        name: String,
    },
    Status,
}

impl WindowId {
    pub fn name(&self) -> Option<&str> {
        use self::WindowId::*;
        match *self {
            Channel { ref name, .. } => Some(name),
            Query { ref name, .. } => Some(name),
            Status => None
        }
    }
}

impl PartialEq for WindowId {
    fn eq(&self, rhs: &WindowId) -> bool {
        use self::WindowId::*;
        match (self, rhs) {
            (&Channel { name: ref name_a, .. }, &Channel { name: ref name_b, .. }) => name_a == name_b,
            (&Query { name: ref name_a, .. }, &Query { name: ref name_b, .. }) => name_a == name_b,
            (&Status, &Status) => true,
            (_, _) => false,
        }
    }
}

pub struct Window {
    display: DisplayArea,
    id: WindowId,
}

impl Window {
    fn new(id: WindowId) -> Window {
        Window {
            display: DisplayArea::new(),
            id: id,
        }
    }

    fn draw(&self) {
        self.display.draw();
    }

    fn redraw(&self) {
        self.display.redraw();
    }

    pub fn name(&self) -> &str {
        self.id.name().unwrap_or("Status")
    }

    pub fn id(&self) -> &WindowId {
        &self.id
    }

    pub fn self_message(&self, message: &str) {
        self.display.self_message(message);
    }

    pub fn show_event(&self, event: &event::ChatEvent) {
        self.display.show_event(event);
    }
}

enum CurrentWindow {
    Status,
    Other(usize),
}

pub struct Windows {
    status: Window,
    windows: Vec<Window>,
    current_window: CurrentWindow,
}

impl Windows {
    pub fn new() -> Windows {
        Windows {
            status: Window::new(WindowId::Status),
            windows: Vec::new(),
            current_window: CurrentWindow::Status,
        }
    }

    pub fn current_window(&self) -> &Window {
        match self.current_window {
            CurrentWindow::Status => &self.status,
            CurrentWindow::Other(i) => self.windows.get(i).unwrap_or(&self.status),
        }
    }

    pub fn current_window_number(&self) -> usize {
        match self.current_window {
            CurrentWindow::Status => 0,
            CurrentWindow::Other(i) => i + 1,
        }
    }

    pub fn highest_window_index(&self) -> usize {
        self.windows.len()
    }

    pub fn current_target(&self) -> Option<&Window> {
        if let CurrentWindow::Other(i) = self.current_window {
            self.windows.get(i)
        } else {
            None
        }
    }

    pub fn draw(&self) {
        self.current_window().draw();
    }

    pub fn close_current(&mut self) {
        use self::CurrentWindow as C;
        match self.current_window {
            C::Status => {}
            C::Other(i) => {
                if i < self.windows.len() {
                    self.windows.remove(i);
                }
            }
        }
        self.current_window = C::Status;
    }

    pub fn change_to(&mut self, i: usize) {
        if i == 0 {
            self.current_window = CurrentWindow::Status;
        } else if i <= self.windows.len() {
            self.current_window = CurrentWindow::Other(i-1);
        }
        self.current_window().redraw();
    }

    pub fn handle_event(&mut self, event: event::ChatEvent) {
        use irc_lib::client::data::Command::*;
        match event.message.command {
            PRIVMSG(ref target, ref _message) | NOTICE(ref target, ref _message) => {
                let window_index;
                if event.is_query {
                    let source = event.message.source_nickname().unwrap_or("Unknown nick");
                    window_index = self.open(source, true);
                } else {
                    window_index = self.open(target, false);
                }
                let window = &self.windows[window_index];
                window.show_event(&event);
            }
            JOIN(ref channel, _, _) => {
                let window_index = self.open(channel, false);
                let window = &self.windows[window_index];
                window.show_event(&event);
            }
            _ => {
                self.status.show_event(&event);
            }
        }
    }

    fn get_index_by_name(&self, name: &str) -> Option<usize> {
        for (i, window) in self.windows.iter().enumerate() {
            match window.id().name() {
                Some(win_name) if irc_equal(win_name, name) => return Some(i),
                _ => {}
            }
        }
        None
    }

    // Has to return an index.
    // Returning &Window would cause the mutable borrow to persist.
    fn open(&mut self, name: &str, is_query: bool) -> usize {
        if let Some(i) = self.get_index_by_name(name) {
             return i;
        }
        let name_owned = String::from(name);
        let window;
        if is_query {
            window = Window::new(WindowId::Query { name: name_owned });
        } else {
            window = Window::new(WindowId::Channel { name: name_owned });
        }
        window.display.add_str(name);
        self.windows.push(window);
        let len = self.windows.len();
        self.change_to(len);
        len - 1
    }

    pub fn query(&mut self, name: &str) -> Result<(), ()> {
        let name = name.trim().split(' ').nth(0);
        let name = match name {
            Some(x) => x,
            None => return Err(()),
        };
        let index = self.open(name, true);
        self.change_to(index + 1);
        Ok(())
    }
}
