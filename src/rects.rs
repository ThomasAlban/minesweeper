// functions which return rectangles to be either drawn or used in the application logic

use crate::consts::*;
use crate::game::*;
use piston_window::*;

pub fn mouse_intersects_rect(mouse_pos: [f64; 2], rect: [f64; 4]) -> bool {
    mouse_pos[0] >= rect[0]
        && mouse_pos[0] <= rect[0] + rect[2]
        && mouse_pos[1] >= rect[1]
        && mouse_pos[1] <= rect[1] + rect[3]
}

pub fn get_window_size(game: &Minesweeper) -> [f64; 2] {
    [
        game.dimensions.x as f64 * TILE_SIZE as f64 + BORDER_SIZE * 2.,
        game.dimensions.y as f64 * TILE_SIZE as f64
            + BORDER_MIDDLE_POS
            + BORDER_SIZE
            + BORDER_SIZE * BORDER_WIDE_SCALE_FACTOR,
    ]
}

pub fn get_face_button_rect(window: &PistonWindow) -> [f64; 4] {
    [
        (window.size().width / 2.) - (FACE_BUTTON_SIZE / 2.),
        (BORDER_MIDDLE_POS + BORDER_SIZE) / 2. - (FACE_BUTTON_SIZE / 2.),
        FACE_BUTTON_SIZE,
        FACE_BUTTON_SIZE,
    ]
}

pub fn get_difficulty_buttons_rects(window: &PistonWindow) -> [[f64; 4]; 3] {
    let easy_button_width = DIFFICULTY_BUTTON_HEIGHT * 2.27;
    let medium_button_width = DIFFICULTY_BUTTON_HEIGHT * 3.;
    let hard_button_width = DIFFICULTY_BUTTON_HEIGHT * 2.27;

    let button_y = window.size().height
        - ((BORDER_SIZE * BORDER_WIDE_SCALE_FACTOR) / 2.)
        - (DIFFICULTY_BUTTON_HEIGHT / 2.);
    [
        // easy button
        [
            (window.size().width / 2.)
                - (medium_button_width / 2.)
                - DIFFICULTY_BUTTON_SPACING
                - easy_button_width,
            button_y,
            easy_button_width,
            DIFFICULTY_BUTTON_HEIGHT,
        ],
        // medium button
        [
            (window.size().width / 2.) - (medium_button_width / 2.),
            button_y,
            medium_button_width,
            DIFFICULTY_BUTTON_HEIGHT,
        ],
        // hard button
        [
            (window.size().width / 2.) + (medium_button_width / 2.) + DIFFICULTY_BUTTON_SPACING,
            button_y,
            hard_button_width,
            DIFFICULTY_BUTTON_HEIGHT,
        ],
    ]
}

pub fn get_mines_left_rect() -> [f64; 4] {
    [
        BORDER_SIZE + 3.,
        (BORDER_MIDDLE_POS + BORDER_SIZE) / 2. - (NUM_DISPLAY_HEIGHT / 2.),
        NUM_DISPLAY_WIDTH,
        NUM_DISPLAY_HEIGHT,
    ]
}

pub fn get_stopwatch_rect(window: &PistonWindow) -> [f64; 4] {
    [
        window.size().width - BORDER_SIZE - 3. - NUM_DISPLAY_WIDTH,
        (BORDER_MIDDLE_POS + BORDER_SIZE) / 2. - (NUM_DISPLAY_HEIGHT / 2.),
        NUM_DISPLAY_WIDTH,
        NUM_DISPLAY_HEIGHT,
    ]
}
