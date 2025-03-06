use anyhow::{anyhow, Result};
use sdl2::{event::Event, pixels::PixelFormatEnum, render::Texture, surface::Surface};
use tokio::sync::mpsc;

use crate::{constants::{DOOM_HEIGHT, DOOM_WIDTH}, message::GUIMessage};

pub fn run(mut rx: mpsc::Receiver<GUIMessage>) -> Result<()> {
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
