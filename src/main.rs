use anyhow::Result;
use clap::Parser;
use doom::LighthouseDoom;
use lighthouse_client::{protocol::Authentication, Lighthouse, LIGHTHOUSE_URL};
use tracing::info;
use tokio::{runtime::Runtime, sync::mpsc, task};
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

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);

    #[cfg(feature = "gui")]
    let (gui_tx, gui_rx) = mpsc::channel(8);
    let (updater_tx, updater_rx) = mpsc::channel(8);
    let (controller_tx, controller_rx) = mpsc::channel(8);

    let doom = LighthouseDoom::new(
        #[cfg(feature = "gui")]
        gui_tx,
        updater_tx,
        controller_rx,
    );

    let tokio_handle = thread::Builder::new().name("Tokio".into()).spawn(move || {
        let rt = Runtime::new().unwrap();

        rt.block_on(async move {
            let lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await.unwrap();
            info!("Connected to the Lighthouse server");

            let input = lh.stream_input().await.unwrap();

            let updater_handle = task::spawn(updater::run(lh, updater_rx));
            let controller_handle = task::spawn(controller::run(input, controller_tx));

            updater_handle.await.unwrap().unwrap();
            controller_handle.await.unwrap().unwrap();
        });
    })?;

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

    tokio_handle.join().unwrap();
    doom_handle.join().unwrap();

    Ok(())
}
