use doomgeneric::{game::DoomGeneric, input::{keys::{self, KEY_DOWN, KEY_ENTER, KEY_ESCAPE, KEY_FIRE, KEY_LEFT, KEY_RIGHT, KEY_SPEED, KEY_STRAFE, KEY_STRAFELEFT, KEY_STRAFERIGHT, KEY_UP, KEY_USE}, KeyData}};
use lighthouse_client::protocol::{Color, Frame, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS};
use tokio::sync::mpsc;
use tracing::info;

use crate::message::{ControllerMessage, Key, UpdaterMessage};

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
        let mut frame = Frame::empty();
        for i in 0..LIGHTHOUSE_ROWS {
            for j in 0..LIGHTHOUSE_COLS {
                let y = (i * yres) / LIGHTHOUSE_ROWS;
                let x = (j * xres) / LIGHTHOUSE_COLS;
                let pixel = screen_buffer[y * xres + x];
                let color = Color::new(((pixel >> 16) & 0xFF) as u8, ((pixel >> 8) & 0xFF) as u8, (pixel & 0xFF) as u8);
                frame.set(x, y, color);
            }
        }
        self.updater_tx.blocking_send(UpdaterMessage::Frame(frame)).unwrap();
    }

    fn get_key(&mut self) -> Option<KeyData> {
        self.controller_tx.try_recv().ok().and_then(|message| match message {
            ControllerMessage::Key { key, down } => {
                convert_key(key).map(|code| {
                    KeyData { pressed: down, key: code }
                })
            }
        })
    }

    fn set_window_title(&mut self, title: &str) {
        info!("Window title: {title}");
    }
}

fn convert_key(key: Key) -> Option<u8> {
    match key {
        Key::Right => Some(*KEY_RIGHT),
        Key::Left => Some(*KEY_LEFT),
        Key::Up => Some(*KEY_UP),
        Key::Down => Some(*KEY_DOWN),
        Key::StrafeLeft => Some(*KEY_STRAFELEFT),
        Key::StrafeRight => Some(*KEY_STRAFERIGHT),
        Key::Fire => Some(*KEY_FIRE),
        Key::Use => Some(*KEY_USE),
        Key::Strafe => Some(*KEY_STRAFE),
        Key::Speed => Some(*KEY_SPEED),
        Key::Escape => Some(KEY_ESCAPE),
        Key::Enter => Some(KEY_ENTER),
        Key::Letter(c) => keys::from_char(c),
    }
}
