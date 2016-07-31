pub enum Command {
    Join(String),
    Part(::tui::window::WindowId),
}
