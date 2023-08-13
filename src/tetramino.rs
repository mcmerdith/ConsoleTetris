use crate::{
    board::{BOARD_HEIGHT, BOARD_WIDTH},
    game::{TileGrid, TileGridMap},
    graphics::Tile,
    position_outside_bounds,
};
use grid::{grid, Grid};
use rand::{distributions::Standard, prelude::Distribution, random};
use ratatui::{style::Color, widgets::canvas::Shape};

const LINE_COLOR: Color = Color::Indexed(31);
const REVERSE_L_COLOR: Color = Color::Indexed(3);
const L_COLOR: Color = Color::Indexed(240);
const SQUARE_COLOR: Color = Color::Indexed(252);
const SQUIGGLE_COLOR: Color = Color::Indexed(28);
const T_COLOR: Color = Color::Indexed(131);
const REVERSE_SQUIGGLE_COLOR: Color = Color::Indexed(224);

pub enum TetraminoType {
    Line,
    ReverseL,
    L,
    Square,
    Squiggle,
    T,
    ReverseSquiggle,
}

impl Distribution<TetraminoType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TetraminoType {
        match rng.gen_range(0..7) {
            0 => TetraminoType::Line,
            1 => TetraminoType::ReverseL,
            2 => TetraminoType::L,
            3 => TetraminoType::Square,
            4 => TetraminoType::Squiggle,
            5 => TetraminoType::T,
            _ => TetraminoType::ReverseSquiggle,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tetramino {
    /// the grid of tiles making up the tetramino
    tile_grid: TileGrid,
    /// the relative x position
    x_pos: i32,
    /// the relative y position
    y_pos: i32,
}

impl TileGridMap for Tetramino {
    /// Return a [`Vec`] of the tetraminoes tiles
    ///
    /// ```
    /// let tile = (grid_x_position, grid_y_position, tile_color);
    /// ```
    fn get_tiles(&self) -> Vec<Tile> {
        self.tile_grid
            .get_tiles()
            .iter()
            .map(|tile| Tile {
                x: self.x_pos + tile.x,
                y: self.y_pos + tile.y,
                color: tile.color,
            })
            .collect()
    }
}

impl Default for Tetramino {
    fn default() -> Self {
        Tetramino::new(random())
    }
}

impl Tetramino {
    /// Create a new tetramino
    pub fn new(piece: TetraminoType) -> Tetramino {
        match piece {
            TetraminoType::Line => Tetramino {
                tile_grid: grid![
                    [None, None, None, None]
                    [Some(LINE_COLOR), Some(LINE_COLOR), Some(LINE_COLOR), Some(LINE_COLOR)]
                    [None, None, None, None]
                    [None, None, None, None]
                ],
                x_pos: 3,
                y_pos: 0,
            },
            TetraminoType::L => Tetramino {
                tile_grid: grid![
                    [None, None, Some(L_COLOR)]
                    [Some(L_COLOR), Some(L_COLOR), Some(L_COLOR)]
                    [None, None, None]
                ],
                x_pos: 3,
                y_pos: 0,
            },
            TetraminoType::ReverseL => Tetramino {
                tile_grid: grid![
                    [None, None, Some(REVERSE_L_COLOR)]
                    [Some(REVERSE_L_COLOR), Some(REVERSE_L_COLOR), Some(REVERSE_L_COLOR)]
                    [None, None, None]
                ],
                x_pos: 3,
                y_pos: 0,
            },
            TetraminoType::Square => Tetramino {
                tile_grid: grid![
                    [Some(SQUARE_COLOR), Some(SQUARE_COLOR)]
                    [Some(SQUARE_COLOR), Some(SQUARE_COLOR)]
                ],
                x_pos: 4,
                y_pos: 0,
            },
            TetraminoType::Squiggle => Tetramino {
                tile_grid: grid![
                    [None, Some(SQUIGGLE_COLOR), Some(SQUIGGLE_COLOR)]
                    [Some(SQUIGGLE_COLOR), Some(SQUIGGLE_COLOR), None]
                    [None, None, None]
                ],
                x_pos: 3,
                y_pos: 0,
            },
            TetraminoType::T => Tetramino {
                tile_grid: grid![
                    [None, Some(T_COLOR), None]
                    [Some(T_COLOR), Some(T_COLOR), Some(T_COLOR)]
                    [None, None, None]
                ],
                x_pos: 3,
                y_pos: 0,
            },
            TetraminoType::ReverseSquiggle => Tetramino {
                tile_grid: grid![
                    [Some(REVERSE_SQUIGGLE_COLOR), Some(REVERSE_SQUIGGLE_COLOR), None]
                    [None, Some(REVERSE_SQUIGGLE_COLOR), Some(REVERSE_SQUIGGLE_COLOR)]
                    [None, None, None]
                ],
                x_pos: 3,
                y_pos: 0,
            },
        }
    }

    /// Check if the tetramino is either:
    ///
    /// 1. outside the bounds of the board
    /// 2. colliding with another tile
    ///
    /// after moved by `x_offset` and `y_offset`
    ///
    /// [`None`] indicates the position is valid
    /// If [`Some`], there will always be at least one tile in the [`Vec`]
    ///
    /// ```
    /// let tile = (grid_x_position, grid_y_position, tile_color);
    /// ```
    pub fn position_invalid(
        &self,
        x_offset: i32,
        y_offset: i32,
        board_tile_grid: &TileGrid,
    ) -> Option<Vec<Tile>> {
        // check if tile is out of bounds
        let collision: Vec<Tile> = self
            .get_tiles()
            .iter()
            .filter_map(|tile| {
                let x = tile.x + x_offset;
                let y = tile.y + y_offset;

                let out_of_bounds = position_outside_bounds!(x, y);
                let board_collision = board_tile_grid
                    .get_tiles()
                    .iter()
                    .any(|board_tile| x == board_tile.x && y == board_tile.y);

                if out_of_bounds || board_collision {
                    // invalid tiles are returned
                    Some(tile.to_owned())
                } else {
                    // valid tiles are not
                    None
                }
            })
            .collect();

        // TODO check tile collision

        if collision.is_empty() {
            None
        } else {
            Some(collision)
        }
    }

    /// Move the tetramino by `x` and `y`
    ///
    /// Returns `true` if the move was successful,
    /// `false` if the position would be invalid after the move
    pub fn move_position(&mut self, x: i32, y: i32, board_tile_grid: &TileGrid) -> bool {
        // check if position would be invalid
        if self.position_invalid(x, y, board_tile_grid).is_some() {
            return false;
        }

        // move the piece if valid
        self.x_pos += x;
        self.y_pos += y;

        true
    }

    /// Rotate the tetramino clockwise
    ///
    /// Does nothing if the position would be invalid after the rotation
    pub fn rotate(&mut self, board_tile_grid: &TileGrid) {
        // store the previous position in case rotation is impossible
        let original_tiles = self.tile_grid.to_owned();

        // make the rotated grid
        let (rows, cols) = self.tile_grid.size();
        self.tile_grid = Grid::new(cols, rows);

        // map the original tiles to the new rotated grid
        original_tiles
            .iter_rows()
            .enumerate()
            .for_each(|(row, col_iter)| {
                col_iter
                    .enumerate()
                    .for_each(|(col, tile)| self.tile_grid[col][rows - row - 1] = *tile)
            });

        // check if new position is okay
        if self.position_invalid(0, 0, board_tile_grid).is_none() {
            return;
        };

        // try to wall kick if position is invalid
        for resolve in vec![-1, 1, -2, 2] {
            if self.move_position(resolve, 0, board_tile_grid)
                || self.move_position(0, resolve, board_tile_grid)
            {
                // if wall kick was successfull return
                return;
            }
        }

        // rotation is impossible, revert the tiles
        self.tile_grid = original_tiles;
    }
}

impl Shape for Tetramino {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for tile in self.get_tiles() {
            tile.draw(painter);
        }
    }
}
