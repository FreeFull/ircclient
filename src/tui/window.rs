use super::displayarea::DisplayArea;

use event;

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

    pub fn name(&self) -> &str {
        self.id.name().unwrap_or("Status")
    }

    pub fn id(&self) -> &WindowId {
        &self.id
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
    }

    pub fn handle_event(&mut self, event: event::ChatEvent) {
        use irc_lib::client::data::Command::*;
        match event.message.command {
            PRIVMSG(..) => {
                // TODO: Proper implementation
                self.current_window().display.show_event(&event);
            }
            JOIN(ref channel, _, _) => {
                let window = self.join(channel);
                window.display.show_event(&event);
            }
            _ => {}
        }
    }

    fn get_index_by_name(&self, name: &str) -> Option<usize> {
        for (i, window) in self.windows.iter().enumerate() {
            if window.id().name() == Some(name) {
                return Some(i)
            }
        }
        None
    }

    fn join(&mut self, channel: &str) -> &Window {
       if let Some(i) = self.get_index_by_name(channel) {
           return &self.windows[i];
       }
       self.windows.push(Window::new(WindowId::Channel { name: String::from(channel) }));
       let len = self.windows.len();
       self.change_to(len);
       self.current_window()
    }
}
