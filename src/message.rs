use lighthouse_client::protocol::Frame;

/// A key used by doom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Right,
    Left,
    Up,
    Down,
    StrafeLeft,
    StrafeRight,
    Fire,
    Use,
    Strafe,
    Speed,
    Escape,
    Enter,
    Letter(char),
}

/// A message sent from the (long-running) controller task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerMessage {
    Key(Key),
}

/// A message sent to the (long-running) updater task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdaterMessage {
    Frame(Frame),
}
