use anyhow::Result;
use clap::Parser;
use doom::LighthouseDoom;
use lighthouse_client::{protocol::Authentication, Lighthouse, LIGHTHOUSE_URL};
use tracing::info;
use tokio::{sync::mpsc, task};
use std::thread;

mod constants;
mod controller;
mod doom;
#[cfg(feature = "gui")]
mod gui;
mod message;
mod updater;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// The username.
    #[arg(short, long, env = "LIGHTHOUSE_USER")]
    username: String,
    /// The API token.
    #[arg(short, long, env = "LIGHTHOUSE_TOKEN")]
    token: String,
    /// The server URL.
    #[arg(long, env = "LIGHTHOUSE_URL", default_value = LIGHTHOUSE_URL)]
    url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);

    let (gui_tx, gui_rx) = mpsc::channel(8);
    let (updater_tx, updater_rx) = mpsc::channel(8);
    let (controller_tx, controller_rx) = mpsc::channel(8);

    let doom = LighthouseDoom::new(gui_tx, updater_tx, controller_rx);

    let lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await?;
    info!("Connected to the Lighthouse server");

    let input = lh.stream_input().await?;

    let updater_handle = task::spawn(updater::run(lh, updater_rx));
    let controller_handle = task::spawn(controller::run(input, controller_tx));
    let doom_handle = thread::Builder::new().name("DOOM".into()).spawn(move || {
        info!("Running DOOM...");
        doom.run();
    })?;

    #[cfg(feature = "gui")]
    {
        // NOTE: The GUI must run on the main thread
        info!("Running GUI...");
        gui::run(gui_rx).unwrap();
    }

    updater_handle.await??;
    controller_handle.await??;
    doom_handle.join().unwrap();

    Ok(())
}
