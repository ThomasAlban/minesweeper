// all the game logic for minesweeper is in this file (completely independent of the graphics)

use rand::Rng;
use std::cmp;
use stopwatch::Stopwatch;

#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    Playing,
    Won,
    Lost,
}

#[derive(Copy, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(PartialEq)]
pub enum TileMines {
    // number of mines around the tile
    NoMine(u8),
    Mine,
}

#[derive(PartialEq)]
pub enum TileState {
    Closed,
    Flagged,
    Open,
}

// struct used throughout the program to store coordinates
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

// struct for storing each tile
pub struct Tile {
    pub coord: Coord,
    pub mines: TileMines,
    pub state: TileState,
}
impl Tile {
    // creates a new tile that is by default closed and has no mines around it
    fn new(coord: Coord) -> Self {
        Tile {
            coord,
            mines: TileMines::NoMine(0),
            state: TileState::Closed,
        }
    }
}

pub struct Minesweeper {
    // stores the whole game state
    pub dimensions: Coord,
    mines: u8,
    pub state: GameState,
    pub difficulty: Difficulty,
    pub tiles: Vec<Tile>,
    first_go: bool,
    pub stopwatch: Stopwatch,
    pub mines_left: i32,
}
impl Minesweeper {
    pub fn new(difficulty: Difficulty) -> Self {
        // this is where the dimensions and no of mines for each difficulty are defined
        let game_info: (Coord, u8) = match difficulty {
            Difficulty::Easy => (Coord { x: 9, y: 9 }, 10),
            Difficulty::Medium => (Coord { x: 16, y: 16 }, 40),
            Difficulty::Hard => (Coord { x: 30, y: 16 }, 100),
        };

        // construct the tiles vector, pushing in the correct number of tiles to fill the board
        let mut tiles = Vec::new();
        for x in 0..game_info.0.x {
            for y in 0..game_info.0.y {
                tiles.push(Tile::new(Coord { x, y }));
            }
        }

        Minesweeper {
            dimensions: game_info.0,
            mines: game_info.1,
            state: GameState::Playing,
            difficulty,
            tiles,
            first_go: true,
            stopwatch: Stopwatch::new(),
            mines_left: game_info.1 as i32,
        }
    }

    // helper functions to return a (mutable) reference to the tile at a given coord
    pub fn get_tile(&self, coord: Coord) -> &Tile {
        &self.tiles[coord.y as usize * self.dimensions.x as usize + coord.x as usize]
    }
    pub fn get_tile_mut(&mut self, coord: Coord) -> &mut Tile {
        &mut self.tiles[coord.y as usize * self.dimensions.x as usize + coord.x as usize]
    }

    // function which returns an iterator over each neighbour of a given coord
    pub fn iter_neighbours(&self, coord: Coord) -> impl Iterator<Item = Coord> {
        // variables to store the min and max values for the x and y coords which can be iterated over
        // coords are prevented from being out of bounds using the cmp::max and cmp::min functions
        let x = cmp::max(coord.x as i32 - 1, 0);
        let x2 = cmp::min(coord.x as i32 + 1, self.dimensions.x as i32 - 1);

        let y = cmp::max(coord.y as i32 - 1, 0);
        let y2 = cmp::min(coord.y as i32 + 1, self.dimensions.y as i32 - 1);

        let mut coords = Vec::new();
        for y in y as u8..=y2 as u8 {
            for x in x as u8..=x2 as u8 {
                if x == coord.x && y == coord.y {
                    continue;
                }
                // add each coord surrounding the inputted coord to the 'coords' vector
                coords.push(Coord { x, y });
            }
        }
        // return the coords vector as an iterator
        coords.into_iter()
    }

    fn gen_map(&mut self, start_coord: Coord) {
        for _ in 0..self.mines {
            let mut tile_coord: Coord;
            loop {
                // keep trying until we find a tile with no mine and not a neighbour of the start tile
                tile_coord = Coord {
                    x: rand::thread_rng().gen_range(0..=(self.dimensions.x - 1)),
                    y: rand::thread_rng().gen_range(0..=(self.dimensions.y - 1)),
                };
                if self.get_tile(tile_coord).mines == TileMines::Mine
                    || (tile_coord.x as i32 - start_coord.x as i32).abs() <= 1
                        && (tile_coord.y as i32 - start_coord.y as i32).abs() <= 1
                {
                    // if the tile is the start tile, then continue (this is needed bc iter_neighbours doesn't iter over the tile itself)
                    continue;
                }
                break;
            }
            // assign the mine
            self.get_tile_mut(tile_coord).mines = TileMines::Mine;

            for neighbour in self.iter_neighbours(tile_coord) {
                if let TileMines::NoMine(n) = self.get_tile(neighbour).mines {
                    if n < 8 {
                        // increment mines variables for tiles around each mine
                        self.get_tile_mut(neighbour).mines = TileMines::NoMine(n + 1);
                    }
                }
            }
        }
    }

    pub fn count_flagged_neighbours(&mut self, coord: Coord) -> u8 {
        let mut count = 0;
        for neighbour in self.iter_neighbours(coord) {
            if self.get_tile(neighbour).state == TileState::Flagged {
                count += 1;
            }
        }
        count
    }

    pub fn open(&mut self, coord: Coord) {
        // generate the map if it is the first go
        if self.first_go {
            self.gen_map(coord);
            self.stopwatch.start();
            self.first_go = false;
        }
        match self.get_tile(coord).state {
            TileState::Closed => {
                self.get_tile_mut(coord).state = TileState::Open;
                if self.get_tile(coord).mines == TileMines::Mine {
                    self.state = GameState::Lost;
                } else if self.get_tile(coord).mines == TileMines::NoMine(0) {
                    // if the tile has no mines around it, open all the tiles around it recursively
                    for neighbour in self.iter_neighbours(coord) {
                        if self.get_tile(neighbour).state == TileState::Closed {
                            self.open(neighbour);
                        }
                    }
                }
            }
            TileState::Flagged => {
                self.get_tile_mut(coord).state = TileState::Closed;
            }
            TileState::Open => {
                if let TileMines::NoMine(n) = self.get_tile(coord).mines {
                    if n == 0 {
                        return;
                    }
                    // count no of flagged neighbours
                    let count = self.count_flagged_neighbours(coord);
                    // if no of flagged neighbours matches the number of the tile,
                    // open all the non-flagged tiles around it (which may open a mine if the flags were wrong)
                    if count == n {
                        for neighbour in self.iter_neighbours(coord) {
                            if self.get_tile(neighbour).state == TileState::Closed {
                                self.open(neighbour);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn flag(&mut self, coord: Coord) {
        match self.get_tile(coord).state {
            TileState::Closed => {
                self.get_tile_mut(coord).state = TileState::Flagged;
                self.mines_left -= 1;
            }
            TileState::Flagged => {
                self.get_tile_mut(coord).state = TileState::Closed;
                self.mines_left += 1;
            }
            TileState::Open => {
                // if we flag an open tile, either unflag all the tiles around it or flag all the tiles around it
                // depeding on if all the closed tiles around it are flagged or not (as a shortcut to manually flagging tiles)
                if let TileMines::NoMine(n) = self.get_tile(coord).mines {
                    if n == 0 {
                        return;
                    }
                    let mut unflag_all: bool = true;
                    for neighbour in self.iter_neighbours(coord) {
                        if self.get_tile(neighbour).state == TileState::Closed {
                            // if we find a closed tile that is not flagged around it, don't unflag all the tiles around it
                            unflag_all = false;
                            break;
                        }
                    }

                    if unflag_all {
                        for neighbour in self.iter_neighbours(coord) {
                            // if all closed tiles around the tile are flagged, unflag all the closed tiles around it
                            if self.get_tile(neighbour).state == TileState::Flagged {
                                self.get_tile_mut(neighbour).state = TileState::Closed;
                                self.mines_left += 1;
                            }
                        }
                    } else {
                        for neighbour in self.iter_neighbours(coord) {
                            // if not, flag all the closed tiles around it
                            if self.get_tile(neighbour).state == TileState::Closed {
                                self.get_tile_mut(neighbour).state = TileState::Flagged;
                                self.mines_left -= 1;
                            }
                        }
                    }
                }
            }
        }
    }

    // more efficient if we check gamestate once in the main function rather than again and again in open function
    pub fn check_state(&mut self) {
        // check if the game has been won
        let mut game_won = true;
        for tile in self.tiles.iter() {
            if tile.state == TileState::Open && tile.mines == TileMines::Mine {
                self.stopwatch.stop();
                self.state = GameState::Lost;
                return;
            }
            if tile.state != TileState::Open && tile.mines != TileMines::Mine {
                game_won = false;
                // cannot break as there may be another mine that is open and so we may still have lost
            }
        }
        self.state = if game_won {
            self.mines_left = 0;
            self.stopwatch.stop();
            GameState::Won
        } else {
            GameState::Playing
        };
    }
}
