pub const TILE_SIZE: f64 = 32.;

pub const BORDER_SIZE: f64 = TILE_SIZE * 0.6;
// how far down the window the middle border starts
pub const BORDER_MIDDLE_POS: f64 = TILE_SIZE * 2.5;
// how much wider the bottom border is than the normal borders
pub const BORDER_WIDE_SCALE_FACTOR: f64 = 2.5;

pub const FACE_BUTTON_SIZE: f64 = (BORDER_MIDDLE_POS - BORDER_SIZE) * 0.9;

pub const DIFFICULTY_BUTTON_HEIGHT: f64 = TILE_SIZE;
pub const DIFFICULTY_BUTTON_SPACING: f64 = TILE_SIZE * 0.5;

pub const NUM_DISPLAY_WIDTH: f64 = (274. / 170.) * FACE_BUTTON_SIZE;
pub const NUM_DISPLAY_HEIGHT: f64 = FACE_BUTTON_SIZE;
