use std::cell::Cell;

use anyhow::{anyhow, Result};
use lighthouse_client::protocol::{Delta, Pos, Zero, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS};
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton as SDLMouseButton, pixels::PixelFormatEnum, render::Texture, surface::Surface};
use tokio::sync::mpsc;
use tracing::info;

use crate::{constants::{DOOM_HEIGHT, DOOM_WIDTH}, message::{ControllerMessage, GUIMessage, Key, MouseButton}};

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
    let mut last_pos: Option<Pos<f64>> = None;

    let mouse_down: Cell<bool> = Cell::new(false);

    let mut handle_mouse_event = |sdl_button: Option<SDLMouseButton>, x: i32, y: i32, relative: bool, pointer_locked: bool| {
        let mut pos = Pos::new(
            x as f64 / DOOM_WIDTH as f64 * LIGHTHOUSE_COLS as f64,
            y as f64 / DOOM_HEIGHT as f64 * LIGHTHOUSE_ROWS as f64,
        );
        if relative {
            pos = last_pos.unwrap_or(Pos::ZERO) + pos;
        }
        let movement: Delta<f64> = pos - last_pos.unwrap_or(pos);
        last_pos = Some(pos);
        let button = sdl_button.and_then(convert_mouse_button).unwrap_or(MouseButton::Left);
        tx.blocking_send(ControllerMessage::Mouse { button, movement, down: mouse_down.get(), pointer_locked })?;
        anyhow::Ok(())
    };

    'running: loop {
        if let Some(event) = event_pump.poll_event() {
            let pointer_locked = canvas.window().grab();
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    // Lock pointer on click
                    if mouse_btn == SDLMouseButton::Left && !pointer_locked {
                        info!("Locking pointer (press escape to unlock)");
                        canvas.window_mut().set_grab(true);
                        sdl_context.mouse().set_relative_mouse_mode(true);
                    }

                    mouse_down.set(true);
                    handle_mouse_event(Some(mouse_btn), x, y, false, pointer_locked)?;
                },
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    mouse_down.set(false);
                    handle_mouse_event(Some(mouse_btn), x, y, false, pointer_locked)?;
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    handle_mouse_event(None, xrel, yrel, true, pointer_locked)?;
                },
                Event::KeyDown { keycode, .. } => {
                    // Unlock pointer on escape
                    if keycode == Some(Keycode::Escape) && pointer_locked {
                        info!("Unlocking pointer");
                        canvas.window_mut().set_grab(false);
                        sdl_context.mouse().set_relative_mouse_mode(false);
                    }

                    if let Some(key) = convert_key(keycode) {
                        tx.blocking_send(ControllerMessage::Key { key, down: true })?;
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = convert_key(keycode) {
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

fn convert_mouse_button(sdl_button: SDLMouseButton) -> Option<MouseButton> {
    match sdl_button {
        SDLMouseButton::Left => Some(MouseButton::Left),
        SDLMouseButton::Middle => Some(MouseButton::Middle),
        SDLMouseButton::Right => Some(MouseButton::Right),
        _ => None,
    }
}

fn convert_key(sdl_key: Option<Keycode>) -> Option<Key> {
    match sdl_key {
        Some(Keycode::Left) => Some(Key::ArrowLeft),
        Some(Keycode::Right) => Some(Key::ArrowRight),
        Some(Keycode::Up) => Some(Key::ArrowUp),
        Some(Keycode::Down) => Some(Key::ArrowDown),
        Some(Keycode::Escape) => Some(Key::Escape),
        Some(Keycode::Return) => Some(Key::Enter),
        Some(Keycode::Space) => Some(Key::Space),
        Some(Keycode::LCtrl) => Some(Key::Ctrl),
        Some(Keycode::RCtrl) => Some(Key::Ctrl),
        Some(Keycode::LShift) => Some(Key::Shift),
        Some(Keycode::RShift) => Some(Key::Shift),
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
        _ => None,
    }
}
