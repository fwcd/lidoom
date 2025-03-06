use anyhow::Result;
use tokio::sync::mpsc;

use crate::message::{Action, ControllerMessage, Key, MapperMessage};

pub async fn run(
    mut rx: mpsc::Receiver<ControllerMessage>,
    tx: mpsc::Sender<MapperMessage>,
) -> Result<()> {
    while let Some(message) = rx.recv().await {
        match message {
            ControllerMessage::Key { key, down } => {
                tx.send(MapperMessage::Action { action: key_to_action(key), down }).await?;
            }
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
