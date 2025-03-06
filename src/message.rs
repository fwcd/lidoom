use lighthouse_client::protocol::Frame;

use crate::constants::{DOOM_HEIGHT, DOOM_WIDTH};

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
    Frame([u8; 3 * DOOM_WIDTH * DOOM_HEIGHT]),
    UpdateTitle(String),
}
