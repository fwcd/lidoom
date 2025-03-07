use std::collections::HashMap;

use anyhow::Result;
use lighthouse_client::protocol::Direction;
use tokio::sync::mpsc;

use crate::message::{Action, ControllerMessage, GamepadButton, GamepadStick, Key, MapperMessage};

pub async fn run(
    mut rx: mpsc::Receiver<ControllerMessage>,
    tx: mpsc::Sender<MapperMessage>,
) -> Result<()> {
    let mut active_stick_action: HashMap<GamepadStick, Action> = HashMap::new();

    while let Some(message) = rx.recv().await {
        match message {
            ControllerMessage::Key { key, down } => {
                tx.send(MapperMessage::Action { action: key_to_action(key), down }).await?;
            },
            ControllerMessage::GamepadButton { button, down } => {
                if let Some(action) = gamepad_button_to_action(button) {
                    tx.send(MapperMessage::Action { action, down }).await?;
                }
            },
            ControllerMessage::GamepadStick { stick, value } => {
                macro_rules! pop_active_action {
                    () => {
                        if let Some(action) = active_stick_action.remove(&stick) {
                            tx.send(MapperMessage::Action { action, down: false }).await?;
                        }
                    };
                }

                let in_deadzone = value.length() < 0.1;
                if in_deadzone {
                    pop_active_action!();
                } else {
                    let opt_dir = Direction::approximate_from(value);
                    let opt_action = match stick {
                        GamepadStick::Left => opt_dir.map(movement_dir_to_action),
                        GamepadStick::Right => opt_dir.and_then(camera_dir_to_action),
                    };

                    if let Some(action) = opt_action {
                        tx.send(MapperMessage::Action { action, down: true }).await?;
                        if Some(action) != active_stick_action.get(&stick).cloned() {
                            pop_active_action!();
                            active_stick_action.insert(stick, action);
                        }
                    }
                }
            },
        }
    }
    Ok(())
}

fn key_to_action(key: Key) -> Action {
    match key {
        Key::ArrowRight => Action::Right,
        Key::ArrowLeft => Action::Left,
        Key::ArrowUp => Action::Up,
        Key::ArrowDown => Action::Down,
        Key::Letter('W') => Action::Up,
        Key::Letter('S') => Action::Down,
        Key::Letter('A') => Action::StrafeLeft,
        Key::Letter('D') => Action::StrafeRight,
        Key::Ctrl => Action::Use,
        Key::Space => Action::Fire,
        Key::Shift => Action::Speed,
        Key::Escape => Action::Escape,
        Key::Enter => Action::Enter,
        Key::Letter(c) => Action::KeyLetter(c),
    }
}

fn gamepad_button_to_action(button: GamepadButton) -> Option<Action> {
    match button {
        GamepadButton::DPad(dir) => Some(movement_dir_to_action(dir)),
        _ => None,
    }
}

fn movement_dir_to_action(dir: Direction) -> Action {
    match dir {
        Direction::Up => Action::Up,
        Direction::Down => Action::Down,
        Direction::Left => Action::StrafeLeft,
        Direction::Right => Action::StrafeRight,
    }
}

fn camera_dir_to_action(dir: Direction) -> Option<Action> {
    match dir {
        Direction::Left => Some(Action::Left),
        Direction::Right => Some(Action::Right),
        _ => None,
    }
}
