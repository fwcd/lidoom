use anyhow::{anyhow, Result};
use doomgeneric::game::{DOOMGENERIC_RESX, DOOMGENERIC_RESY};
use sdl2::event::Event;

pub fn run() -> Result<()> {
    let sdl_context = sdl2::init().map_err(|e| anyhow!("{e}"))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!("{e}"))?;
    
    let window = video_subsystem
        .window("DOOM", DOOMGENERIC_RESX as u32, DOOMGENERIC_RESY as u32) // TODO: Receive titles
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().map_err(|e| anyhow!("{e}"))?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {},
            }
        }
    }

    Ok(())
}
