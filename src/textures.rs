// structs to contain all the textures used in the game

use piston_window::*;
use std::path::PathBuf;

pub struct GameTextures {
    pub tile: TileTextures,
    pub border: BorderTextures,
    pub face_buttons: FaceButtonTextures,
    pub difficulty_buttons: DifficultyButtonTextures,
    pub number_display: NumberDisplayTextures,
}

impl GameTextures {
    pub fn new(window: &mut PistonWindow) -> Self {
        // get the folder of the executable
        let mut exe_folder = std::env::current_exe().unwrap();
        // remove the executable's name, leaving the path to the containing folder
        exe_folder.pop();
        // find the assets folder starting from the executable's folder
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .of(exe_folder)
            .for_folder("assets")
            .unwrap();
        // return the struct containing all the textures
        GameTextures {
            tile: TileTextures::new(window, &assets),
            border: BorderTextures::new(window, &assets),
            face_buttons: FaceButtonTextures::new(window, &assets),
            difficulty_buttons: DifficultyButtonTextures::new(window, &assets),
            number_display: NumberDisplayTextures::new(window, &assets),
        }
    }
}

trait MinesweeperTexture {
    fn new(window: &mut PistonWindow, assets: &PathBuf) -> Self;

    fn load_texture(window: &mut PistonWindow, assets: &PathBuf, path: &str) -> G2dTexture {
        Texture::from_path(
            &mut window.create_texture_context(),
            assets.join(path),
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap()
    }
}

pub struct TileTextures {
    pub closed: G2dTexture,
    pub flagged: G2dTexture,

    pub mine: G2dTexture,
    pub mine_hit: G2dTexture,
    pub mine_wrong: G2dTexture,

    pub number: [G2dTexture; 9],
}

impl MinesweeperTexture for TileTextures {
    fn new(window: &mut PistonWindow, assets: &PathBuf) -> Self {
        TileTextures {
            closed: Self::load_texture(window, &assets, "tiles/tile_closed.png"),
            flagged: Self::load_texture(window, &assets, "tiles/tile_flagged.png"),

            mine: Self::load_texture(window, &assets, "tiles/tile_mine.png"),
            mine_hit: Self::load_texture(window, &assets, "tiles/tile_mine_hit.png"),
            mine_wrong: Self::load_texture(window, &assets, "tiles/tile_mine_wrong.png"),

            number: [
                Self::load_texture(window, &assets, "tiles/tile_0.png"),
                Self::load_texture(window, &assets, "tiles/tile_1.png"),
                Self::load_texture(window, &assets, "tiles/tile_2.png"),
                Self::load_texture(window, &assets, "tiles/tile_3.png"),
                Self::load_texture(window, &assets, "tiles/tile_4.png"),
                Self::load_texture(window, &assets, "tiles/tile_5.png"),
                Self::load_texture(window, &assets, "tiles/tile_6.png"),
                Self::load_texture(window, &assets, "tiles/tile_7.png"),
                Self::load_texture(window, &assets, "tiles/tile_8.png"),
            ],
        }
    }
}

pub struct BorderTextures {
    pub top_left: G2dTexture,
    pub top_right: G2dTexture,

    pub middle_left: G2dTexture,
    pub middle_right: G2dTexture,

    pub bottom_left_wide: G2dTexture,
    pub bottom_right_wide: G2dTexture,

    pub horizontal: G2dTexture,
    pub horizontal_wide: G2dTexture,

    pub vertical: G2dTexture,
}

impl MinesweeperTexture for BorderTextures {
    fn new(window: &mut PistonWindow, assets: &PathBuf) -> Self {
        BorderTextures {
            top_left: Self::load_texture(window, &assets, "border/top_left.png"),
            top_right: Self::load_texture(window, &assets, "border/top_right.png"),

            middle_left: Self::load_texture(window, &assets, "border/middle_left.png"),
            middle_right: Self::load_texture(window, &assets, "border/middle_right.png"),

            bottom_left_wide: Self::load_texture(window, &assets, "border/bottom_left_wide.png"),
            bottom_right_wide: Self::load_texture(window, &assets, "border/bottom_right_wide.png"),

            horizontal: Self::load_texture(window, &assets, "border/horizontal.png"),
            horizontal_wide: Self::load_texture(window, &assets, "border/horizontal_wide.png"),

            vertical: Self::load_texture(window, &assets, "border/vertical.png"),
        }
    }
}

pub struct FaceButtonTextures {
    pub normal: G2dTexture,
    pub won: G2dTexture,
    pub lost: G2dTexture,
}

impl MinesweeperTexture for FaceButtonTextures {
    fn new(window: &mut PistonWindow, assets: &PathBuf) -> Self {
        FaceButtonTextures {
            normal: Self::load_texture(window, &assets, "buttons/face_normal.jpg"),
            won: Self::load_texture(window, &assets, "buttons/face_won.jpg"),
            lost: Self::load_texture(window, &assets, "buttons/face_lost.jpg"),
        }
    }
}

pub struct DifficultyButtonTextures {
    pub easy: G2dTexture,
    pub easy_pressed: G2dTexture,

    pub medium: G2dTexture,
    pub medium_pressed: G2dTexture,

    pub hard: G2dTexture,
    pub hard_pressed: G2dTexture,
}

impl MinesweeperTexture for DifficultyButtonTextures {
    fn new(window: &mut PistonWindow, assets: &PathBuf) -> Self {
        DifficultyButtonTextures {
            easy: Self::load_texture(window, &assets, "buttons/easy.jpg"),
            easy_pressed: Self::load_texture(window, &assets, "buttons/easy_pressed.jpg"),

            medium: Self::load_texture(window, &assets, "buttons/medium.jpg"),
            medium_pressed: Self::load_texture(window, &assets, "buttons/medium_pressed.jpg"),

            hard: Self::load_texture(window, &assets, "buttons/hard.jpg"),
            hard_pressed: Self::load_texture(window, &assets, "buttons/hard_pressed.jpg"),
        }
    }
}

pub struct NumberDisplayTextures {
    pub background: G2dTexture,
    pub numbers: [G2dTexture; 10],
}

impl MinesweeperTexture for NumberDisplayTextures {
    fn new(window: &mut PistonWindow, assets: &PathBuf) -> Self {
        NumberDisplayTextures {
            background: Self::load_texture(window, &assets, "number_display/background.jpg"),
            numbers: [
                Self::load_texture(window, &assets, "number_display/0.jpg"),
                Self::load_texture(window, &assets, "number_display/1.jpg"),
                Self::load_texture(window, &assets, "number_display/2.jpg"),
                Self::load_texture(window, &assets, "number_display/3.jpg"),
                Self::load_texture(window, &assets, "number_display/4.jpg"),
                Self::load_texture(window, &assets, "number_display/5.jpg"),
                Self::load_texture(window, &assets, "number_display/6.jpg"),
                Self::load_texture(window, &assets, "number_display/7.jpg"),
                Self::load_texture(window, &assets, "number_display/8.jpg"),
                Self::load_texture(window, &assets, "number_display/9.jpg"),
            ],
        }
    }
}
