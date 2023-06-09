// functions which return an iterator over a collection of sprites to be drawn
use piston_window::*;

use crate::consts::*;
use crate::game::*;
use crate::textures::*;

pub fn tile_sprites<'a>(
    game: &Minesweeper,
    textures: &'a GameTextures,
) -> impl Iterator<Item = (Image, &'a Texture<gfx_device_gl::Resources>)> {
    let mut sprites = Vec::with_capacity(game.dimensions.x as usize * game.dimensions.y as usize);
    for x in 0..game.dimensions.x {
        for y in 0..game.dimensions.y {
            let coord = Coord { x, y };

            let image = Image::new().rect([
                x as f64 * TILE_SIZE + BORDER_SIZE,
                y as f64 * TILE_SIZE + BORDER_MIDDLE_POS + BORDER_SIZE,
                TILE_SIZE,
                TILE_SIZE,
            ]);
            let mut texture = match game.get_tile(coord).state {
                TileState::Closed => &textures.tile.closed,
                TileState::Flagged => &textures.tile.flagged,
                TileState::Open => match game.get_tile(coord).mines {
                    TileMines::NoMine(n) => &textures.tile.number[n as usize],
                    TileMines::Mine => &textures.tile.mine,
                },
            };

            if game.state == GameState::Lost {
                if game.get_tile(coord).mines == TileMines::Mine {
                    match game.get_tile(coord).state {
                        // if we have lost, show any open mines as hit
                        TileState::Open => texture = &textures.tile.mine_hit,
                        // show all unopened mines as opened
                        _ => texture = &textures.tile.mine,
                    }
                } else if game.get_tile(coord).state == TileState::Flagged {
                    // show all flags that aren't mines as wrong
                    texture = &textures.tile.mine_wrong;
                }
            } else if game.state == GameState::Won && game.get_tile(coord).mines == TileMines::Mine
            {
                // if we have won, show all mines as flagged
                texture = &textures.tile.flagged;
            }
            sprites.push((image, texture));
        }
    }
    sprites.into_iter()
}

pub fn border_sprites<'a>(
    window_size: &Size,
    textures: &'a GameTextures,
) -> impl Iterator<Item = (Image, &'a Texture<gfx_device_gl::Resources>)> {
    vec![
        // top
        (
            Image::new().rect([0., 0., window_size.width, BORDER_SIZE]),
            &textures.border.horizontal,
        ),
        // bottom
        (
            Image::new().rect([
                0.,
                window_size.height - BORDER_SIZE * BORDER_WIDE_SCALE_FACTOR,
                window_size.width,
                BORDER_SIZE * BORDER_WIDE_SCALE_FACTOR,
            ]),
            &textures.border.horizontal_wide,
        ),
        // left
        (
            Image::new().rect([0., 0., BORDER_SIZE, window_size.height]),
            &textures.border.vertical,
        ),
        // right
        (
            Image::new().rect([
                window_size.width - BORDER_SIZE,
                0.,
                BORDER_SIZE,
                window_size.height,
            ]),
            &textures.border.vertical,
        ),
        // middle
        (
            Image::new().rect([0., BORDER_MIDDLE_POS, window_size.width, BORDER_SIZE]),
            &textures.border.horizontal,
        ),
        // top left
        (
            Image::new().rect([0., 0., BORDER_SIZE, BORDER_SIZE]),
            &textures.border.top_left,
        ),
        // top right
        (
            Image::new().rect([
                window_size.width - BORDER_SIZE,
                0.,
                BORDER_SIZE,
                BORDER_SIZE,
            ]),
            &textures.border.top_right,
        ),
        // bottom left
        (
            Image::new().rect([
                0.,
                window_size.height - BORDER_SIZE * BORDER_WIDE_SCALE_FACTOR,
                BORDER_SIZE,
                BORDER_SIZE * BORDER_WIDE_SCALE_FACTOR,
            ]),
            &textures.border.bottom_left_wide,
        ),
        // bottom right
        (
            Image::new().rect([
                window_size.width - BORDER_SIZE,
                window_size.height - BORDER_SIZE * BORDER_WIDE_SCALE_FACTOR,
                BORDER_SIZE,
                BORDER_SIZE * BORDER_WIDE_SCALE_FACTOR,
            ]),
            &textures.border.bottom_right_wide,
        ),
        // middle left
        (
            Image::new().rect([0., BORDER_MIDDLE_POS, BORDER_SIZE, BORDER_SIZE]),
            &textures.border.middle_left,
        ),
        // middle right
        (
            Image::new().rect([
                window_size.width - BORDER_SIZE,
                BORDER_MIDDLE_POS,
                BORDER_SIZE,
                BORDER_SIZE,
            ]),
            &textures.border.middle_right,
        ),
    ]
    .into_iter()
}

pub fn face_button_sprite<'a>(
    game: &Minesweeper,
    textures: &'a GameTextures,
    face_button_rect: [f64; 4],
) -> (Image, &'a Texture<gfx_device_gl::Resources>) {
    let image = Image::new().rect(face_button_rect);
    let texture = match game.state {
        GameState::Lost => &textures.face_buttons.lost,
        GameState::Won => &textures.face_buttons.won,
        GameState::Playing => &textures.face_buttons.normal,
    };
    (image, texture)
}

pub fn difficulty_buttons_sprites<'a>(
    game: &Minesweeper,
    textures: &'a GameTextures,
    difficulty_buttons_rects: [[f64; 4]; 3],
) -> impl Iterator<Item = (Image, &'a Texture<gfx_device_gl::Resources>)> {
    let mut sprites = Vec::new();
    for (i, button_rect) in difficulty_buttons_rects.iter().enumerate() {
        let image = Image::new().rect(*button_rect);
        let mut texture = match i {
            1 => &textures.difficulty_buttons.medium,
            2 => &textures.difficulty_buttons.hard,
            _ => &textures.difficulty_buttons.easy,
        };
        if game.difficulty as usize == i {
            texture = match i {
                1 => &textures.difficulty_buttons.medium_pressed,
                2 => &textures.difficulty_buttons.hard_pressed,
                _ => &textures.difficulty_buttons.easy_pressed,
            }
        }
        sprites.push((image, texture));
    }
    sprites.into_iter()
}

fn num_display_sprites(
    textures: &GameTextures,
    num_display_rect: [f64; 4],
    num: i32,
) -> impl Iterator<Item = (Image, &Texture<gfx_device_gl::Resources>)> {
    let mut sprites = Vec::new();
    let num = if num < 0 { 0 } else { num };
    // the background
    sprites.push((
        Image::new().rect(num_display_rect),
        &textures.number_display.background,
    ));

    let hundreds_rect = [
        num_display_rect[0] + (14. / 274.) * num_display_rect[2],
        num_display_rect[1] + (14. / 170.) * num_display_rect[3],
        (74. / 274.) * num_display_rect[2],
        (142. / 170.) * num_display_rect[3],
    ];
    sprites.push((
        Image::new().rect(hundreds_rect),
        &textures.number_display.numbers[((num / 100) % 10) as usize],
    ));

    let tens_rect = [
        num_display_rect[0] + (100. / 274.) * num_display_rect[2],
        num_display_rect[1] + (14. / 170.) * num_display_rect[3],
        (74. / 274.) * num_display_rect[2],
        (142. / 170.) * num_display_rect[3],
    ];
    sprites.push((
        Image::new().rect(tens_rect),
        &textures.number_display.numbers[((num / 10) % 10) as usize],
    ));

    let units_rect = [
        num_display_rect[0] + (186. / 274.) * num_display_rect[2],
        num_display_rect[1] + (14. / 170.) * num_display_rect[3],
        (74. / 274.) * num_display_rect[2],
        (142. / 170.) * num_display_rect[3],
    ];
    sprites.push((
        Image::new().rect(units_rect),
        &textures.number_display.numbers[(num % 10) as usize],
    ));

    sprites.into_iter()
}

pub fn stopwatch_sprites<'a>(
    game: &Minesweeper,
    textures: &'a GameTextures,
    stopwatch_rect: [f64; 4],
) -> impl Iterator<Item = (Image, &'a Texture<gfx_device_gl::Resources>)> {
    let time = game.stopwatch.elapsed().as_secs();
    num_display_sprites(textures, stopwatch_rect, time as i32)
}

pub fn mines_left_sprites<'a>(
    game: &Minesweeper,
    textures: &'a GameTextures,
    mines_left_rect: [f64; 4],
) -> impl Iterator<Item = (Image, &'a Texture<gfx_device_gl::Resources>)> {
    num_display_sprites(textures, mines_left_rect, game.mines_left)
}
