use anyhow::Result;
use lighthouse_client::{Lighthouse, TokioWebSocket};
use tokio::sync::mpsc;
use tracing::debug;

use crate::message::UpdaterMessage;

pub async fn run(lh: Lighthouse<TokioWebSocket>, mut rx: mpsc::Receiver<UpdaterMessage>) -> Result<()> {
    while let Some(UpdaterMessage::Frame(frame)) = rx.recv().await {
        // Send the rendered frame to the lighthouse
        lh.put_model(frame).await?;
        debug!("Sent frame");
    }
    Ok(())
}
