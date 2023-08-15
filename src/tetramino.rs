use crate::{
    board::{BOARD_HEIGHT, BOARD_WIDTH},
    game::{TileGrid, TileGridMap},
    game_io::RotationDirection,
    graphics::Tile,
    position_outside_bounds,
};
use grid::{grid, Grid};
use rand::{distributions::Standard, prelude::Distribution, random};
use ratatui::style::Color;

// const I_COLOR: Color = Color::Indexed(31);
// const J_COLOR: Color = Color::Indexed(3);
// const L_COLOR: Color = Color::Indexed(240);
// const O_COLOR: Color = Color::Indexed(252);
// const S_COLOR: Color = Color::Indexed(28);
// const T_COLOR: Color = Color::Indexed(131);
// const Z_COLOR: Color = Color::Indexed(224);

const I_COLOR: Color = Color::Indexed(51);
const J_COLOR: Color = Color::Indexed(33);
const L_COLOR: Color = Color::Indexed(208);
const O_COLOR: Color = Color::Indexed(226);
const S_COLOR: Color = Color::Indexed(40);
const T_COLOR: Color = Color::Indexed(128);
const Z_COLOR: Color = Color::Indexed(160);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    pub fn rotated(&self, rotation_direction: RotationDirection) -> Rotation {
        match rotation_direction {
            RotationDirection::Clockwise => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
            },
            RotationDirection::Counterclockwise => match self {
                Self::North => Self::West,
                Self::East => Self::North,
                Self::South => Self::East,
                Self::West => Self::South,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetraminoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetraminoType {
    /// Returns a [`Vec`] of kick offsets for the type of tetramino
    ///
    /// Offsets should be tried sequentially
    pub fn get_kick_offset_data(
        &self,
        origin_rotation: Rotation,
        target_rotation: Rotation,
    ) -> Vec<(i32, i32)> {
        // All y-values are inverted
        let offset_table = match self {
            Self::J | Self::L | Self::S | Self::T | Self::Z => grid![
                [(0, 0), ( 0, 0), ( 0,  0), (0,  0), ( 0,  0)]
                [(0, 0), ( 1, 0), ( 1,  1), (0, -2), ( 1, -2)]
                [(0, 0), ( 0, 0), ( 0,  0), (0,  0), ( 0,  0)]
                [(0, 0), (-1, 0), (-1,  1), (0, -2), (-1, -2)]
            ],
            Self::I => grid![
                [( 0,  0), (-1,  0), ( 2,  0), (-1,  0), ( 2,  0)]
                [(-1,  0), ( 0,  0), ( 0,  0), ( 0, -1), ( 0,  2)]
                [(-1, -1), ( 1, -1), (-2, -1), ( 1,  0), (-2,  0)]
                [( 0, -1), ( 0, -1), ( 0, -1), ( 0,  1), ( 0, -2)]
            ],
            Self::O => grid![[(0, 0)][(0, 1)][(-1, 1)][(-1, 0)]],
        };

        offset_table
            .iter_row(origin_rotation as usize)
            .zip(offset_table.iter_row(target_rotation as usize))
            .map(|((origin_x, origin_y), (target_x, target_y))| {
                // (target_x - origin_x, target_y - origin_y)
                (
                    origin_x - target_x, // + true_rotation_offset_x,
                    origin_y - target_y, // + true_rotation_offset_y,
                )
            })
            .collect()
    }
}

impl Distribution<TetraminoType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TetraminoType {
        match rng.gen_range(0..7) {
            0 => TetraminoType::I,
            1 => TetraminoType::J,
            2 => TetraminoType::L,
            3 => TetraminoType::O,
            4 => TetraminoType::S,
            5 => TetraminoType::T,
            _ => TetraminoType::Z,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tetramino {
    tetramino_type: TetraminoType,
    /// the grid of tiles making up the tetramino
    tile_grid: TileGrid,
    /// the current tetramino rotation
    rotation: Rotation,
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
        Tetramino {
            tetramino_type: piece,
            tile_grid: match piece {
                TetraminoType::I => grid![
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                    [None, Some(I_COLOR), Some(I_COLOR), Some(I_COLOR), Some(I_COLOR)]
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                ],
                TetraminoType::L => grid![
                    [None, None, Some(L_COLOR)]
                    [Some(L_COLOR), Some(L_COLOR), Some(L_COLOR)]
                    [None, None, None]
                ],
                TetraminoType::J => grid![
                    [Some(J_COLOR), None, None]
                    [Some(J_COLOR), Some(J_COLOR), Some(J_COLOR)]
                    [None, None, None]
                ],
                TetraminoType::O => grid![
                    [None, Some(O_COLOR), Some(O_COLOR)]
                    [None, Some(O_COLOR), Some(O_COLOR)]
                    [None, None, None]
                ],
                TetraminoType::S => grid![
                    [None, Some(S_COLOR), Some(S_COLOR)]
                    [Some(S_COLOR), Some(S_COLOR), None]
                    [None, None, None]
                ],
                TetraminoType::T => grid![
                    [None, Some(T_COLOR), None]
                    [Some(T_COLOR), Some(T_COLOR), Some(T_COLOR)]
                    [None, None, None]
                ],
                TetraminoType::Z => grid![
                    [Some(Z_COLOR), Some(Z_COLOR), None]
                    [None, Some(Z_COLOR), Some(Z_COLOR)]
                    [None, None, None]
                ],
            },
            rotation: Rotation::North,
            x_pos: match piece {
                TetraminoType::I => 3,
                TetraminoType::J => 3,
                TetraminoType::L => 3,
                TetraminoType::O => 4,
                TetraminoType::S => 3,
                TetraminoType::T => 3,
                TetraminoType::Z => 3,
            },
            y_pos: (BOARD_HEIGHT - 1).into(),
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
    pub fn rotate(&mut self, rotation_direction: RotationDirection, board_tile_grid: &TileGrid) {
        // store the previous position in case rotation is impossible
        let original_tiles = self.tile_grid.to_owned();
        let original_rotation = self.rotation;

        // make the rotated grid
        let (rows, cols) = self.tile_grid.size();
        self.tile_grid = Grid::new(cols, rows);
        self.rotation = self.rotation.rotated(rotation_direction);

        // map the original tiles to the new rotated grid
        original_tiles
            .iter_rows()
            .enumerate()
            .for_each(|(row, col_iter)| {
                col_iter.enumerate().for_each(|(col, tile)| {
                    self.tile_grid[match rotation_direction {
                        RotationDirection::Clockwise => col,
                        RotationDirection::Counterclockwise => cols - col - 1,
                    }][match rotation_direction {
                        RotationDirection::Clockwise => rows - row - 1,
                        RotationDirection::Counterclockwise => row,
                    }] = *tile
                })
            });

        // Super-Rotation-System uses an offset table to try and place tetraminoa
        for (x, y) in self
            .tetramino_type
            .get_kick_offset_data(original_rotation, self.rotation)
        {
            if self.move_position(x, y, board_tile_grid) {
                // position is okay
                return;
            }
        }

        // rotation is impossible, revert the tiles
        self.tile_grid = original_tiles;
        self.rotation = original_rotation;
    }
}
