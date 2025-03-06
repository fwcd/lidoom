use lighthouse_client::protocol::Frame;

/// A key on the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    ArrowRight,
    ArrowLeft,
    ArrowUp,
    ArrowDown,
    Escape,
    Enter,
    Shift,
    Space,
    Ctrl,
    Letter(char),
}

/// A game action to take. Usually this is what keys are mapped to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Right,
    Left,
    Up,
    Down,
    StrafeLeft,
    StrafeRight,
    Escape,
    Enter,
    Use,
    Fire,
    Speed,
    KeyLetter(char),
}

/// A message sent from controller -> mapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerMessage {
    Key { key: Key, down: bool },
}

/// A message sent from mapper -> doom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapperMessage {
    Action { action: Action, down: bool },
}

/// A message sent from doom -> updater.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdaterMessage {
    Frame(Frame),
}

/// A message sent from updater -> gui.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GUIMessage {
    Frame(Vec<u8>),
    UpdateTitle(String),
}
