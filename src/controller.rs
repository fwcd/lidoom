use anyhow::Result;
use futures::{prelude::*, Stream};
use lighthouse_client::protocol::{Direction, InputEvent, KeyEvent, ServerMessage};
use tokio::sync::mpsc;

use crate::message::{ControllerMessage, Key};

pub async fn run(
    mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin,
    tx: mpsc::Sender<ControllerMessage>,
) -> Result<()> {
    let mut last_gamepad_key: Option<Key> = None;

    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;

        match input_event {
            InputEvent::Key(KeyEvent { code, down, .. }) => {
                if let Some(key) = convert_key(&code) {
                    tx.send(ControllerMessage::Key { key, down }).await?;
                }
            },
            InputEvent::Gamepad(gamepad) => {
                let opt_key = if let Some(dir) = gamepad.left_direction() {
                    Some(match dir {
                        Direction::Up => Key::Letter('W'),
                        Direction::Left => Key::Letter('A'),
                        Direction::Down => Key::Letter('S'),
                        Direction::Right => Key::Letter('D'),
                    })
                } else if let Some(dir) = gamepad.right_direction() {
                    match dir {
                        Direction::Left => Some(Key::ArrowLeft),
                        Direction::Right => Some(Key::ArrowRight),
                        _ => None,
                    }
                } else {
                    None
                };
                if let Some(key) = opt_key {
                    tx.send(ControllerMessage::Key { key, down: true }).await?;
                    last_gamepad_key = Some(key);
                } else if let Some(key) = last_gamepad_key.take() {
                    tx.send(ControllerMessage::Key { key, down: false }).await?;
                }
            },
            // TODO: Add gamepad input
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
