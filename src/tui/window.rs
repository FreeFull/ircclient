use super::displayarea::DisplayArea;

use event;

#[derive(Clone)]
pub enum WindowId {
    Channel {
        id: i32,
        name: String,
        server: ServerId
    },
    Query {
        id: i32,
        name: String,
    },
    Status,
}

impl WindowId {
    fn name(&self) -> Option<&str> {
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
            (&Channel { id: id_a, server: ref server_a, .. }, &Channel { id: id_b, server: ref server_b, .. }) =>
                id_a == id_b && server_a == server_b,
            (&Query { id: id_a, .. }, &Query { id: id_b, .. }) => id_a == id_b,
            (&Status, &Status) => true,
            (_, _) => false,
        }
    }
}

#[derive(Clone)]
pub struct ServerId {
    pub id: i32,
    pub name: String,
}

impl PartialEq for ServerId {
    fn eq(&self, rhs: &ServerId) -> bool {
        self.id == rhs.id
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

    pub fn join(&mut self, id: WindowId) {
        for (i, win) in self.windows.iter().enumerate() {
            if win.id == id {
                self.current_window = CurrentWindow::Other(i);
                return;
            }
        }
        self.windows.push(Window::new(id));
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
        unimplemented!();
    }
}
