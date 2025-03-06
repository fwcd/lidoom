use std::process;

use doomgeneric::{game::DoomGeneric, input::{keys::{self, KEY_DOWN, KEY_ENTER, KEY_ESCAPE, KEY_FIRE, KEY_LEFT, KEY_RIGHT, KEY_SPEED, KEY_STRAFELEFT, KEY_STRAFERIGHT, KEY_UP, KEY_USE}, KeyData}};
use lighthouse_client::protocol::{Color, Frame, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS};
use tokio::sync::mpsc;
use tracing::info;

use crate::{constants::{DOOM_HEIGHT, DOOM_WIDTH}, message::{ControllerMessage, GUIMessage, Key, UpdaterMessage}};

pub struct LighthouseDoom {
    #[cfg(feature = "gui")]
    gui_tx: mpsc::Sender<GUIMessage>,
    updater_tx: mpsc::Sender<UpdaterMessage>,
    controller_tx: mpsc::Receiver<ControllerMessage>,
}

impl LighthouseDoom {
    pub fn new(
        #[cfg(feature = "gui")]
        gui_tx: mpsc::Sender<GUIMessage>,
        updater_tx: mpsc::Sender<UpdaterMessage>,
        controller_tx: mpsc::Receiver<ControllerMessage>,
    ) -> Self {
        Self {
            #[cfg(feature = "gui")]
            gui_tx,
            updater_tx,
            controller_tx
        }
    }

    pub fn run(self) {
        doomgeneric::game::init(self);

        loop {
            doomgeneric::game::tick();
        }
    }
}

impl DoomGeneric for LighthouseDoom {
    fn draw_frame(&mut self, screen_buffer: &[u32], xres: usize, yres: usize) {
        assert!(xres == DOOM_WIDTH);
        assert!(yres == DOOM_HEIGHT);

        #[cfg(feature = "gui")]
        {
            // Send frame to GUI
            let mut screen_frame = vec![0u8; 3 * DOOM_WIDTH * DOOM_HEIGHT];
            for y in 0..DOOM_HEIGHT {
                for x in 0..DOOM_WIDTH {
                    let pixel_idx = y * DOOM_WIDTH + x;
                    let rgb_idx = 3 * pixel_idx;
                    let pixel = screen_buffer[pixel_idx];
                    screen_frame[rgb_idx] = ((pixel >> 16) & 0xFF) as u8; // red
                    screen_frame[rgb_idx + 1] = ((pixel >> 8) & 0xFF) as u8; // green
                    screen_frame[rgb_idx + 2] = (pixel & 0xFF) as u8; // blue
                }
            }
            self.gui_tx.blocking_send(GUIMessage::Frame(screen_frame)).unwrap_or_else(|_| quit_upon_channel_close());
        }

        // Send frame to updater (i.e. lighthouse)
        let mut frame = Frame::empty();
        for i in 0..LIGHTHOUSE_ROWS {
            for j in 0..LIGHTHOUSE_COLS {
                let y = (i * DOOM_HEIGHT) / LIGHTHOUSE_ROWS;
                let x = (j * DOOM_WIDTH) / LIGHTHOUSE_COLS;
                let pixel = screen_buffer[y * DOOM_WIDTH + x];
                let color = Color::new(((pixel >> 16) & 0xFF) as u8, ((pixel >> 8) & 0xFF) as u8, (pixel & 0xFF) as u8);
                frame.set(j, i, color);
            }
        }
        self.updater_tx.blocking_send(UpdaterMessage::Frame(frame)).unwrap_or_else(|_| quit_upon_channel_close());
    }

    fn get_key(&mut self) -> Option<KeyData> {
        self.controller_tx.try_recv().ok().and_then(|message| match message {
            ControllerMessage::Key { key, down } => {
                map_key(key).map(|code| {
                    let key_data = KeyData { pressed: down, key: code };
                    info!("{:?}", key_data);
                    key_data
                })
            }
        })
    }

    fn set_window_title(&mut self, title: &str) {
        info!("Window title: {title}");
        #[cfg(feature = "gui")]
        self.gui_tx.blocking_send(GUIMessage::UpdateTitle(title.into())).unwrap_or_else(|_| quit_upon_channel_close());
    }
}

fn quit_upon_channel_close() {
    // When one of the channels close, this means one of the other threads
    // (Tokio or the GUI/main thread) have finished, indicating that the
    // user wishes to quit the game. Since we cannot panic/unwind through
    // C functions, we'll just exit the process from here to avoid a
    // crash message.
    info!("Quitting upong channel close...");
    process::exit(0);
}

fn map_key(key: Key) -> Option<u8> {
    match key {
        Key::ArrowRight => Some(*KEY_RIGHT),
        Key::ArrowLeft => Some(*KEY_LEFT),
        Key::ArrowUp => Some(*KEY_UP),
        Key::ArrowDown => Some(*KEY_DOWN),
        Key::Letter('W') => Some(*KEY_UP),
        Key::Letter('S') => Some(*KEY_DOWN),
        Key::Letter('A') => Some(*KEY_STRAFELEFT),
        Key::Letter('D') => Some(*KEY_STRAFERIGHT),
        Key::Ctrl => Some(*KEY_USE),
        Key::Space => Some(*KEY_FIRE),
        Key::Shift => Some(*KEY_SPEED),
        Key::Escape => Some(KEY_ESCAPE),
        Key::Enter => Some(KEY_ENTER),
        Key::Letter(c) => keys::from_char(c.to_ascii_lowercase()), // TODO: Is this correct?
    }
}
