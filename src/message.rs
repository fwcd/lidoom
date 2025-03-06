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

/// A message sent from the (long-running) controller task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerMessage {
    Key { key: Key, down: bool },
}

/// A message sent to the (long-running) updater task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdaterMessage {
    Frame(Frame),
}

/// A message sent to the GUI.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GUIMessage {
    Frame(Vec<u8>),
    UpdateTitle(String),
}
