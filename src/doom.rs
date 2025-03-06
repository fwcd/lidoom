use doomgeneric::game::DoomGeneric;
use tokio::sync::mpsc;
use tracing::info;

use crate::message::{ControllerMessage, UpdaterMessage};

pub struct LighthouseDoom {
    updater_tx: mpsc::Sender<UpdaterMessage>,
    controller_tx: mpsc::Receiver<ControllerMessage>,
}

impl LighthouseDoom {
    pub fn new(
        updater_tx: mpsc::Sender<UpdaterMessage>,
        controller_tx: mpsc::Receiver<ControllerMessage>,
    ) -> Self {
        Self { updater_tx, controller_tx }
    }

    pub fn run(self) {
        doomgeneric::game::init(self);
    }
}

impl DoomGeneric for LighthouseDoom {
    fn draw_frame(&mut self, screen_buffer: &[u32], xres: usize, yres: usize) {
        // TODO
    }

    fn get_key(&mut self) -> Option<doomgeneric::input::KeyData> {
        // TODO
        None
    }

    fn set_window_title(&mut self, title: &str) {
        info!("Window title: {title}");
    }
}
