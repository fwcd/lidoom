use std::time::Duration;

use doomgeneric::game::{DOOMGENERIC_RESX, DOOMGENERIC_RESY};

pub const DOOM_WIDTH: usize = DOOMGENERIC_RESX;
pub const DOOM_HEIGHT: usize = DOOMGENERIC_RESY;

pub const UPDATE_INTERVAL: Duration = Duration::from_millis(200);
