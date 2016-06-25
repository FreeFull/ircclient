use super::displayarea::DisplayArea;
use irc_utils::irc_equal;
use irc::client::prelude::*;

enum WindowKind {
    Channel,
    Query,
    Status,
}

pub struct Window {
    display: DisplayArea,
    name: String,
    kind: WindowKind,
}

impl Window {
    fn new<S>(name: S, kind: WindowKind) -> Window
    where S: Into<String>
    {
        Window {
            display: DisplayArea::new(),
            name: name.into(),
            kind: kind,
        }
    }

    fn draw(&self) {
        self.display.draw();
    }

    pub fn name(&self) -> &str {
        &self.name
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
    server: IrcServer,
}

impl Windows {
    pub fn new(server: IrcServer) -> Windows {
        Windows {
            status: Window::new("*status*", WindowKind::Status),
            windows: Vec::new(),
            current_window: CurrentWindow::Status,
            server: server,
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

    pub fn current_channel(&self) -> Option<&str> {
        let window = self.current_window();
        if let WindowKind::Channel = window.kind {
            Some(window.name())
        } else {
            None
        }
    }

    pub fn draw(&self) {
        self.current_window().draw();
    }

    pub fn join(&mut self, chanlist: &str) {
        for (i, win) in self.windows.iter().enumerate() {
            if irc_equal(&win.name, chanlist) {
                self.current_window = CurrentWindow::Other(i);
                return;
            }
        }
        self.windows.push(Window::new(chanlist, WindowKind::Channel));
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

    pub fn part(&mut self, chanlist: &str) {
        self.windows.retain(|x| !irc_equal(&x.name, chanlist));
    }

    pub fn change_to(&mut self, i: usize) {
        if i == 0 {
            self.current_window = CurrentWindow::Status;
        } else if i <= self.windows.len() {
            self.current_window = CurrentWindow::Other(i-1);
        }
    }

    pub fn handle_message(&mut self, message: Message) {
        let source_nickname = message.source_nickname().unwrap_or("");
        use irc::client::data::Command::*;
        match message.command {
            PRIVMSG(ref target, ref message) => {
            }
            NOTICE(ref target, ref message) => {
            }
            NICK(ref newname) => {
            }
            // https://tools.ietf.org/html/rfc2812#section-3.2.1
            // Depends on server not sending a list of channels.
            JOIN(ref chanlist, _, _) => {
                if source_nickname == self.server.current_nickname() {
                    self.join(chanlist);
                } else {
                }
            }
            PART(ref chanlist, ref message) => {
                if source_nickname == self.server.current_nickname() {
                    self.part(chanlist);
                } else {
                }
            }
            QUIT(ref message) => {
            }
            Response(ref response, ref arguments, ref suffix) => {}
            _ => {}
        }
    }
}
