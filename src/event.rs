pub struct ChatEvent {
    source_nickname: Option<String>,
    pub event: ChatEventKind,
}

impl ChatEvent {
    pub fn source_nickname(&self) -> Option<&str> {
        self.source_nickname.as_ref().map(|x| &**x)
    }
}

pub enum ChatEventKind {
    RoomMsg(String, String),
    Join(String),
    NickChange(String),
}
