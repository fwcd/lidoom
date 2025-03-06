use anyhow::Result;
use lighthouse_client::{Lighthouse, TokioWebSocket};
use tokio::{sync::mpsc, time};
use tracing::debug;

use crate::{constants::UPDATE_INTERVAL, message::UpdaterMessage};

pub async fn run(lh: Lighthouse<TokioWebSocket>, mut rx: mpsc::Receiver<UpdaterMessage>) -> Result<()> {
    while let Some(UpdaterMessage::Frame(frame)) = rx.recv().await {
        // Send the rendered snake to the lighthouse
        lh.put_model(frame).await?;
        debug!("Sent frame");

        // Wait for a short period of time
        // TODO: Do we need this?
        time::sleep(UPDATE_INTERVAL).await;
    }
    Ok(())
}
