use anyhow::Result;
use futures::{prelude::*, Stream};
use lighthouse_client::protocol::{Direction, GamepadAxis2DEvent, GamepadButtonEvent, GamepadControlEvent, InputEvent, KeyEvent, ServerMessage};
use tokio::sync::mpsc;

use crate::message::{ControllerMessage, GamepadButton, GamepadStick, GamepadTrigger, Key};

pub async fn run(
    mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin,
    tx: mpsc::Sender<ControllerMessage>,
) -> Result<()> {
    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;

        match input_event {
            InputEvent::Key(KeyEvent { code, down, .. }) => {
                if let Some(key) = convert_key(&code) {
                    tx.send(ControllerMessage::Key { key, down }).await?;
                }
            },
            InputEvent::Gamepad(gamepad) => match gamepad.control {
                GamepadControlEvent::Button(GamepadButtonEvent { index, down, .. }) => {
                    if let Some(button) = convert_gamepad_button(index) {
                        tx.send(ControllerMessage::GamepadButton { button, down }).await?;
                    }
                },
                GamepadControlEvent::Axis2D(GamepadAxis2DEvent { index, value }) => {
                    if let Some(stick) = convert_gamepad_axis2d(index) {
                        tx.send(ControllerMessage::GamepadStick { stick, value }).await?;
                    }
                },
                _ => {},
            },
            _ => {},
        }
    }

    Ok(())
}

fn convert_key(js_key: &str) -> Option<Key> {
    match js_key {
        "ArrowLeft" => Some(Key::ArrowLeft),
        "ArrowRight" => Some(Key::ArrowRight),
        "ArrowUp" => Some(Key::ArrowUp),
        "ArrowDown" => Some(Key::ArrowDown),
        "Enter" => Some(Key::Enter),
        "Escape" => Some(Key::Escape),
        "ShiftLeft" => Some(Key::Shift),
        "ShiftRight" => Some(Key::Shift),
        "Space" => Some(Key::Space),
        "CtrlLeft" => Some(Key::Ctrl),
        "CtrlRight" => Some(Key::Ctrl),
        _ if js_key.starts_with("Digit") => Some(Key::Letter(js_key.as_bytes()[5] as char)),
        _ if js_key.starts_with("Key") => Some(Key::Letter(js_key.as_bytes()[3] as char)),
        _ => None,
        // TODO: Map more keys
    }
}

fn convert_gamepad_button(button_idx: usize) -> Option<GamepadButton> {
    // See https://www.w3.org/TR/gamepad/#dfn-standard-gamepad
    match button_idx {
        0 => Some(GamepadButton::Cluster(Direction::Down)),
        1 => Some(GamepadButton::Cluster(Direction::Right)),
        2 => Some(GamepadButton::Cluster(Direction::Left)),
        3 => Some(GamepadButton::Cluster(Direction::Up)),
        6 => Some(GamepadButton::Trigger(GamepadTrigger::Left)),
        7 => Some(GamepadButton::Trigger(GamepadTrigger::Right)),
        12 => Some(GamepadButton::DPad(Direction::Up)),
        13 => Some(GamepadButton::DPad(Direction::Down)),
        14 => Some(GamepadButton::DPad(Direction::Left)),
        15 => Some(GamepadButton::DPad(Direction::Right)),

        _ => None,
    }
}

fn convert_gamepad_axis2d(axis2d_idx: usize) -> Option<GamepadStick> {
    // See https://github.com/ProjectLighthouseCAU/nighthouse/blob/77db0a00d93bcc538f9ea6455005fc8a2f29b46c/src/common/protocol/input/new.ts#L60-L66
    match axis2d_idx {
        0 => Some(GamepadStick::Left),
        1 => Some(GamepadStick::Right),
        _ => None,
    }
}
