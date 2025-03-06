use anyhow::Result;
use futures::{prelude::*, Stream};
use lighthouse_client::protocol::{InputEvent, ServerMessage};
use tokio::sync::mpsc;

use crate::message::ControllerMessage;

pub async fn run(
    mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin,
    tx: mpsc::Sender<ControllerMessage>,
) -> Result<()> {
    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;
        match input_event {
            InputEvent::Key(key) => {
                // TODO: Send key
            },
            _ => {},
        }
    }

    Ok(())
}
