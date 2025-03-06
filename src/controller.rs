use anyhow::Result;
use futures::{prelude::*, Stream};
use lighthouse_client::protocol::{InputEvent, ServerMessage};
use tokio::sync::mpsc;

use crate::message::{ControllerMessage, Key};

pub async fn run(
    mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin,
    tx: mpsc::Sender<ControllerMessage>,
) -> Result<()> {
    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;
        match input_event {
            InputEvent::Key(key) => {
                if let Some(key) = convert_key(&key.code) {
                    tx.send(ControllerMessage::Key(key)).await?;
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
        "ArrowLeft" => Some(Key::Left),
        "ArrowRight" => Some(Key::Right),
        "ArrowUp" => Some(Key::Up),
        "ArrowDown" => Some(Key::Down),
        "Enter" => Some(Key::Enter),
        "Escape" => Some(Key::Escape),
        "Shift" => Some(Key::Speed),
        _ if js_key.starts_with("Digit") => Some(Key::Letter(js_key.as_bytes()[5] as char)),
        _ if js_key.starts_with("Key") => Some(Key::Letter(js_key.as_bytes()[3] as char)),
        _ => None,
        // TODO: Map more keys
    }
}
