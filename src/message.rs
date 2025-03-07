use lighthouse_client::protocol::{Direction, Frame, Vec2};

/// A key on the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
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

/// A trigger on the gamepad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadTrigger {
    Left,
    Right,
}

/// A button on the gamepad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    DPad(Direction),
    Menu,
    Cluster(Direction),
    Trigger(GamepadTrigger),
}

/// A stick on the gamepad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadStick {
    Left,
    Right,
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

/// A message sent from controller or gui -> mapper.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControllerMessage {
    Key { key: Key, down: bool },
    GamepadButton { button: GamepadButton, down: bool },
    GamepadStick { stick: GamepadStick, value: Vec2<f64> },
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
#[allow(dead_code)]
pub enum GUIMessage {
    Frame(Vec<u8>),
    UpdateTitle(String),
}
