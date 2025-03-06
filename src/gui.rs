use anyhow::{anyhow, Result};
use sdl2::{event::Event, keyboard::{Keycode, Mod}, pixels::PixelFormatEnum, render::Texture, surface::Surface};
use tokio::sync::mpsc;

use crate::{constants::{DOOM_HEIGHT, DOOM_WIDTH}, message::{ControllerMessage, GUIMessage, Key}};

pub fn run(
    mut rx: mpsc::Receiver<GUIMessage>,
    tx: mpsc::Sender<ControllerMessage>,
) -> Result<()> {
    let sdl_context = sdl2::init().map_err(|e| anyhow!("{e}"))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!("{e}"))?;
    
    let window = video_subsystem
        .window("DOOM", DOOM_WIDTH as u32, DOOM_HEIGHT as u32) // TODO: Receive titles
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().map_err(|e| anyhow!("{e}"))?;

    'running: loop {
        if let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, keymod, .. } => {
                    if let Some(key) = convert_key(keycode, keymod) {
                        tx.blocking_send(ControllerMessage::Key { key, down: true })?;
                    }
                },
                Event::KeyUp { keycode, keymod, .. } => {
                    if let Some(key) = convert_key(keycode, keymod) {
                        tx.blocking_send(ControllerMessage::Key { key, down: false })?;
                    }
                },
                // TODO: Gamepad events
                _ => {},
            }
        }

        if let Ok(message) = rx.try_recv() {
            match message {
                GUIMessage::Frame(mut frame) => {
                    let surface = Surface::from_data(
                        &mut frame,
                        DOOM_WIDTH as u32,
                        DOOM_HEIGHT as u32,
                        (DOOM_WIDTH * 3) as u32,
                        PixelFormatEnum::RGB24
                    ).map_err(|e| anyhow!("{e}"))?;

                    let texture = Texture::from_surface(&surface, &texture_creator)?;
                    canvas.copy(&texture, None, None).map_err(|e| anyhow!("{e}"))?;
                    canvas.present();
                },
                GUIMessage::UpdateTitle(title) => {
                    canvas.window_mut().set_title(&title)?;
                },
            }
        }
    }

    Ok(())
}

fn convert_key(sdl_key: Option<Keycode>, sdl_mod: Mod) -> Option<Key> {
    match sdl_key {
        Some(Keycode::Left) => Some(Key::ArrowLeft),
        Some(Keycode::Right) => Some(Key::ArrowRight),
        Some(Keycode::Up) => Some(Key::ArrowUp),
        Some(Keycode::Down) => Some(Key::ArrowDown),
        Some(Keycode::Escape) => Some(Key::Escape),
        Some(Keycode::Return) => Some(Key::Enter),
        Some(Keycode::Space) => Some(Key::Space),
        Some(Keycode::A) => Some(Key::Letter('A')),
        Some(Keycode::B) => Some(Key::Letter('B')),
        Some(Keycode::C) => Some(Key::Letter('C')),
        Some(Keycode::D) => Some(Key::Letter('D')),
        Some(Keycode::E) => Some(Key::Letter('E')),
        Some(Keycode::F) => Some(Key::Letter('F')),
        Some(Keycode::G) => Some(Key::Letter('G')),
        Some(Keycode::H) => Some(Key::Letter('H')),
        Some(Keycode::I) => Some(Key::Letter('I')),
        Some(Keycode::J) => Some(Key::Letter('J')),
        Some(Keycode::K) => Some(Key::Letter('K')),
        Some(Keycode::L) => Some(Key::Letter('L')),
        Some(Keycode::M) => Some(Key::Letter('M')),
        Some(Keycode::N) => Some(Key::Letter('N')),
        Some(Keycode::O) => Some(Key::Letter('O')),
        Some(Keycode::P) => Some(Key::Letter('P')),
        Some(Keycode::Q) => Some(Key::Letter('Q')),
        Some(Keycode::R) => Some(Key::Letter('R')),
        Some(Keycode::S) => Some(Key::Letter('S')),
        Some(Keycode::T) => Some(Key::Letter('T')),
        Some(Keycode::U) => Some(Key::Letter('U')),
        Some(Keycode::V) => Some(Key::Letter('V')),
        Some(Keycode::W) => Some(Key::Letter('W')),
        Some(Keycode::X) => Some(Key::Letter('X')),
        Some(Keycode::Y) => Some(Key::Letter('Y')),
        Some(Keycode::Z) => Some(Key::Letter('Z')),
        None => {
            if sdl_mod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) {
                Some(Key::Ctrl)
            } else if sdl_mod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD) {
                Some(Key::Shift)
            } else {
                None
            }
        },
        _ => None,
    }
}
