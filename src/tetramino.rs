use crate::{
    constraint::{BOARD_HEIGHT, BOARD_WIDTH},
    game::{TileGrid, TileGridMap},
    position_outside_bounds,
};
use grid::{grid, Grid};
use rand::{distributions::Standard, prelude::Distribution};
use ratatui::{style::Color, widgets::canvas::Shape};

const LINE_COLOR: Color = Color::Indexed(31);
const REVERSE_L_COLOR: Color = Color::Indexed(3);
const L_COLOR: Color = Color::Indexed(240);
const SQUARE_COLOR: Color = Color::Indexed(252);
const SQUIGGLE_COLOR: Color = Color::Indexed(28);
const T_COLOR: Color = Color::Indexed(131);
const REVERSE_SQUIGGLE_COLOR: Color = Color::Indexed(224);

pub type Tile = (i32, i32, Color);

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

#[derive(Debug, Clone, PartialEq)]
pub struct Tetramino {
    /// the grid of tiles making up the tetramino
    tiles: TileGrid,
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
        self.tiles
            .get_tiles()
            .iter()
            .map(|(x, y, color)| (self.x_pos + x, self.y_pos + y, color.to_owned()))
            .collect()
    }
}

impl Tetramino {
    /// Create a new tetramino
    pub fn new(piece: TetraminoType, x_pos: i32, y_pos: i32) -> Tetramino {
        match piece {
            TetraminoType::Line => Tetramino {
                tiles: grid![
                    [None, None, None, None]
                    [Some(LINE_COLOR), Some(LINE_COLOR), Some(LINE_COLOR), Some(LINE_COLOR)]
                    [None, None, None, None]
                    [None, None, None, None]
                ],
                x_pos,
                y_pos,
            },
            TetraminoType::L => Tetramino {
                tiles: grid![
                    [None, None, Some(L_COLOR)]
                    [Some(L_COLOR), Some(L_COLOR), Some(L_COLOR)]
                    [None, None, None]
                ],
                x_pos,
                y_pos,
            },
            TetraminoType::ReverseL => Tetramino {
                tiles: grid![
                    [None, None, Some(REVERSE_L_COLOR)]
                    [Some(REVERSE_L_COLOR), Some(REVERSE_L_COLOR), Some(REVERSE_L_COLOR)]
                    [None, None, None]
                ],
                x_pos,
                y_pos,
            },
            TetraminoType::Square => Tetramino {
                tiles: grid![
                    [Some(SQUARE_COLOR), Some(SQUARE_COLOR)]
                    [Some(SQUARE_COLOR), Some(SQUARE_COLOR)]
                ],
                x_pos,
                y_pos,
            },
            TetraminoType::Squiggle => Tetramino {
                tiles: grid![
                    [None, Some(SQUIGGLE_COLOR), Some(SQUIGGLE_COLOR)]
                    [Some(SQUIGGLE_COLOR), Some(SQUIGGLE_COLOR), None]
                    [None, None, None]
                ],
                x_pos,
                y_pos,
            },
            TetraminoType::T => Tetramino {
                tiles: grid![
                    [None, Some(T_COLOR), None]
                    [Some(T_COLOR), Some(T_COLOR), Some(T_COLOR)]
                    [None, None, None]
                ],
                x_pos,
                y_pos,
            },
            TetraminoType::ReverseSquiggle => Tetramino {
                tiles: grid![
                    [Some(REVERSE_SQUIGGLE_COLOR), Some(REVERSE_SQUIGGLE_COLOR), None]
                    [None, Some(REVERSE_SQUIGGLE_COLOR), Some(REVERSE_SQUIGGLE_COLOR)]
                    [None, None, None]
                ],
                x_pos,
                y_pos,
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
    pub fn position_invalid(&self, x_offset: i32, y_offset: i32) -> Option<Vec<Tile>> {
        // check if tile is out of bounds
        let outside: Vec<Tile> = self
            .get_tiles()
            .iter()
            .filter_map(|t| {
                if position_outside_bounds!(t.0 + x_offset, t.1 + y_offset) {
                    // If the tile is out of bounds it should be returned
                    Some(t.to_owned())
                } else {
                    // In bound tiles are not returned
                    None
                }
            })
            .collect();

        // TODO check tile collision

        if outside.is_empty() {
            None
        } else {
            Some(outside)
        }
    }

    /// Move the tetramino by `x` and `y`
    ///
    /// Returns `true` if the move was successful,
    /// `false` if the position would be invalid after the move
    pub fn move_position(&mut self, x: i32, y: i32) -> bool {
        if self.position_invalid(x, y).is_some() {
            return false;
        }

        self.x_pos += x;
        self.y_pos += y;

        true
    }

    /// Rotate the tetramino clockwise
    ///
    /// Does nothing if the position would be invalid after the rotation
    pub fn rotate(&mut self) {
        let (rows, cols) = self.tiles.size();
        let old_blocks = self.tiles.clone();

        self.tiles = Grid::new(cols, rows);
        old_blocks
            .iter_rows()
            .enumerate()
            .for_each(|(row, col_iter)| {
                col_iter
                    .enumerate()
                    .for_each(|(col, tile)| self.tiles[col][rows - row - 1] = tile.clone())
            });

        // check if position is okay
        if self.position_invalid(0, 0).is_none() {
            return;
        };

        // try to wall kick
        for resolve in vec![-1, 1, -2, 2] {
            if self.move_position(resolve, 0) || self.move_position(0, resolve) {
                return;
            }
        }

        // rotation is impossible
        self.tiles = old_blocks;
    }
}

impl Shape for Tetramino {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        // get explicit bounds of the board
        let Some((x1, y1)) =
            painter.get_point(0.0,0.0)
            else { return; };
        let Some((x2, y2)) =
            painter.get_point(BOARD_WIDTH.into(),BOARD_HEIGHT.into())
            else { return; };

        // get starting and ending points from the bounds
        let start_x = if x1 < x2 { x1 } else { x2 };
        let end_x = if x1 < x2 { x2 } else { x1 };

        let start_y = if y1 < y2 { y1 } else { y2 };
        let end_y = if y1 < y2 { y2 } else { y1 };

        // get size of board
        let width = end_x - start_x + 1;
        let height = end_y - start_y + 1;

        // get size of each block
        let block_x_size = width / BOARD_WIDTH as usize;
        let block_y_size = height / BOARD_HEIGHT as usize;

        for (raw_x, raw_y, color) in self.get_tiles() {
            // don't draw the tile if its outside of bounds somehow
            if position_outside_bounds!(raw_x, raw_y) {
                continue;
            }

            // starting corner
            let x_start_pos = start_x + (raw_x as usize) * block_x_size;
            let y_start_pos = start_y + (raw_y as usize) * block_y_size;

            // ending corner
            let x_end_pos = x_start_pos + block_x_size;
            let y_end_pos = y_start_pos + block_y_size;

            // paint each tile
            for x in x_start_pos..x_end_pos {
                for y in y_start_pos..y_end_pos {
                    painter.paint(x, y, color);
                }
            }
        }
    }
}
