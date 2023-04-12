// hide console window on Windows (x86_64-pc-windows-gnu) in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use piston_window::*;

mod game;
use game::*;

mod textures;
use textures::*;

mod sprites;
use sprites::*;

mod rects;
use rects::*;

mod consts;
use consts::*;

fn main() {
    let mut game = Minesweeper::new(Difficulty::Easy);

    let mut window: PistonWindow = WindowSettings::new("Minesweeper", get_window_size(&game))
        .resizable(false)
        .fullscreen(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut face_button_rect = get_face_button_rect(&window);
    let mut difficulty_buttons_rects = get_difficulty_buttons_rects(&window);

    let textures = GameTextures::new(&mut window);

    let mut mouse_pos = [0., 0.];

    // the tile that the mouse is currently over
    let mut coord: Option<Coord> = None;

    window.set_max_fps(8);

    // the main game loop
    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| {
            mouse_pos = pos;
            // if the mouse is over the game area, set the coord to the tile that the mouse is over
            if mouse_intersects_rect(
                mouse_pos,
                [
                    BORDER_SIZE,
                    BORDER_SIZE + BORDER_MIDDLE_POS,
                    (game.dimensions.x as f64) * TILE_SIZE,
                    (game.dimensions.y as f64) * TILE_SIZE,
                ],
            ) {
                coord = Some(Coord {
                    x: ((pos[0] - BORDER_SIZE) / TILE_SIZE) as u8,
                    y: ((pos[1] - BORDER_MIDDLE_POS - BORDER_SIZE) / TILE_SIZE) as u8,
                });
            } else {
                coord = None;
            }
        });

        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == MouseButton::Left {
                if mouse_intersects_rect(mouse_pos, face_button_rect) {
                    game = game::Minesweeper::new(game.difficulty);
                } else {
                    for (i, button_rect) in difficulty_buttons_rects.iter().enumerate() {
                        if mouse_intersects_rect(mouse_pos, *button_rect) {
                            game = match i {
                                1 => game::Minesweeper::new(Difficulty::Medium),
                                2 => game::Minesweeper::new(Difficulty::Hard),
                                _ => game::Minesweeper::new(Difficulty::Easy),
                            };
                        }
                    }
                    window.set_size(get_window_size(&game));
                    face_button_rect = get_face_button_rect(&window);
                    difficulty_buttons_rects = get_difficulty_buttons_rects(&window);
                }
            }
            if game.state == GameState::Playing {
                if let Some(coord) = coord {
                    if button == MouseButton::Left {
                        game.open(coord);
                    } else if button == MouseButton::Right {
                        game.flag(coord);
                    }
                    game.check_state();
                }
            }
        }

        let window_size = window.size();

        let border_sprites = border_sprites(&window_size, &textures);
        let difficulty_buttons_sprites =
            difficulty_buttons_sprites(&game, &textures, difficulty_buttons_rects);
        let stopwatch_sprites = stopwatch_sprites(&game, &textures, get_stopwatch_rect(&window));
        let mines_left_sprites = mines_left_sprites(&game, &textures, get_mines_left_rect());

        window.draw_2d(&e, |c, g, _| {
            clear([0.75; 4], g);
            // draw the tiles
            for (image, texture) in tile_sprites(&game, &textures) {
                image.draw(texture, &Default::default(), c.transform, g);
            }
            // draw the border
            for (image, texture) in border_sprites {
                image.draw(texture, &Default::default(), c.transform, g);
            }
            // draw the face button
            let (image, texture) = face_button_sprite(&game, &textures, face_button_rect);
            image.draw(texture, &Default::default(), c.transform, g);
            // draw the difficulty buttons
            for (image, texture) in difficulty_buttons_sprites {
                image.draw(texture, &Default::default(), c.transform, g);
            }
            // draw the stopwatch
            for (image, texture) in stopwatch_sprites {
                image.draw(texture, &Default::default(), c.transform, g);
            }
            // draw the mines left display
            for (image, texture) in mines_left_sprites {
                image.draw(texture, &Default::default(), c.transform, g);
            }
        });
    }
}
