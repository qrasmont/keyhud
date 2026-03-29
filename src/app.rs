use iced::window::Id;
use rdev::Key;
use std::collections::HashSet;

#[derive(Default)]
pub struct App {
    pub pressed: HashSet<Key>,
}

#[derive(Debug, Clone)]
pub enum Message {
    KeyPressed(Key),
    KeyReleased(Key),
    WindowOpened(Id),
}
