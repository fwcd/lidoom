use std::process;

use doomgeneric::{game::DoomGeneric, input::{keys::{self, KEY_DOWN, KEY_ENTER, KEY_ESCAPE, KEY_FIRE, KEY_LEFT, KEY_RIGHT, KEY_SPEED, KEY_STRAFELEFT, KEY_STRAFERIGHT, KEY_UP, KEY_USE}, KeyData}};
use lighthouse_client::protocol::{Color, Frame, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS};
use tokio::sync::mpsc;
use tracing::info;

use crate::{constants::{DOOM_HEIGHT, DOOM_WIDTH}, message::{Action, GUIMessage, MapperMessage, UpdaterMessage}};

pub struct LighthouseDoom {
    #[cfg(feature = "gui")]
    gui_tx: mpsc::Sender<GUIMessage>,
    updater_tx: mpsc::Sender<UpdaterMessage>,
    mapper_tx: mpsc::Receiver<MapperMessage>,
}

impl LighthouseDoom {
    pub fn new(
        #[cfg(feature = "gui")]
        gui_tx: mpsc::Sender<GUIMessage>,
        updater_tx: mpsc::Sender<UpdaterMessage>,
        mapper_tx: mpsc::Receiver<MapperMessage>,
    ) -> Self {
        Self {
            #[cfg(feature = "gui")]
            gui_tx,
            updater_tx,
            mapper_tx
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
        self.mapper_tx.try_recv().ok().and_then(|message| match message {
            MapperMessage::Action { action, down } => {
                convert_action(action).map(|code| {
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

fn convert_action(action: Action) -> Option<u8> {
    match action {
        Action::Right => Some(*KEY_RIGHT),
        Action::Left => Some(*KEY_LEFT),
        Action::Up => Some(*KEY_UP),
        Action::Down => Some(*KEY_DOWN),
        Action::StrafeLeft => Some(*KEY_STRAFELEFT),
        Action::StrafeRight => Some(*KEY_STRAFERIGHT),
        Action::Use => Some(*KEY_USE),
        Action::Fire => Some(*KEY_FIRE),
        Action::Speed => Some(*KEY_SPEED),
        Action::Escape => Some(KEY_ESCAPE),
        Action::Enter => Some(KEY_ENTER),
        Action::KeyLetter(c) => keys::from_char(c.to_ascii_lowercase()), // TODO: Is this correct?
    }
}
