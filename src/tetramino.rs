use crate::{
    board::{get_spawn_point, MATRIX_WIDTH},
    game::{MinoGrid, MinoGridMap},
    game_io::RotationDirection,
    graphics::Mino,
    position_outside_bounds,
};
use grid::{grid, Grid};
use rand::{distributions::Standard, prelude::Distribution, random};
use ratatui::style::Color;

const I_COLOR: Color = Color::Indexed(51);
const J_COLOR: Color = Color::Indexed(33);
const L_COLOR: Color = Color::Indexed(208);
const O_COLOR: Color = Color::Indexed(226);
const S_COLOR: Color = Color::Indexed(40);
const T_COLOR: Color = Color::Indexed(128);
const Z_COLOR: Color = Color::Indexed(160);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    pub fn rotated(&self, rotation_direction: RotationDirection) -> Facing {
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
pub enum TetriminoType {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl TetriminoType {
    /// Returns a [`Vec`] of offsets for the type of Tetrimino
    ///
    /// Offsets should be tried sequentially
    pub fn get_offset_data(
        &self,
        origin_rotation: Facing,
        target_rotation: Facing,
    ) -> Vec<(i32, i32)> {
        let offset_table = match self {
            Self::J | Self::L | Self::S | Self::T | Self::Z => grid![
                [(0, 0), ( 0, 0), ( 0,  0), (0,  0), ( 0,  0)]
                [(0, 0), ( 1, 0), ( 1,  -1), (0, 2), ( 1, 2)]
                [(0, 0), ( 0, 0), ( 0,  0), (0,  0), ( 0,  0)]
                [(0, 0), (-1, 0), (-1,  -1), (0, 2), (-1, 2)]
            ],
            Self::I => grid![
                [( 0, 0), (-1, 0), ( 2, 0), (-1,  0), ( 2,  0)]
                [(-1, 0), ( 0, 0), ( 0, 0), ( 0,  1), ( 0, -2)]
                [(-1, 1), ( 1, 1), (-2, 1), ( 1,  0), (-2,  0)]
                [( 0, 1), ( 0, 1), ( 0, 1), ( 0, -1), ( 0,  2)]
            ],
            Self::O => grid![[(0, 0)][(0, -1)][(-1, -1)][(-1, 0)]],
        };

        offset_table
            .iter_row(origin_rotation as usize)
            .zip(offset_table.iter_row(target_rotation as usize))
            .map(|((origin_x, origin_y), (target_x, target_y))| {
                (origin_x - target_x, origin_y - target_y)
            })
            .collect()
    }
}

impl Distribution<TetriminoType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TetriminoType {
        match rng.gen_range(0..7) {
            0 => TetriminoType::O,
            1 => TetriminoType::I,
            2 => TetriminoType::T,
            3 => TetriminoType::L,
            4 => TetriminoType::J,
            5 => TetriminoType::S,
            _ => TetriminoType::Z,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TetriminoPreview {
    /// the grid of minos making up the Tetrimino
    minos: MinoGrid,
    /// the index of the preview
    index: usize,
}

impl MinoGridMap for TetriminoPreview {
    fn get_minos(&self) -> Vec<Mino> {
        self.minos
            .get_minos()
            .iter()
            .map(|mino| Mino {
                x: mino.x,
                y: mino.y - 3 * self.index as i32,
                color: mino.color,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tetrimino {
    // the type of the Tetrimino
    tetrimino_type: TetriminoType,
    /// the grid of minos making up the Tetrimino
    minos: MinoGrid,
    /// the current rotation
    rotation: Facing,
    /// the relative x position
    x_pos: i32,
    /// the relative y position
    y_pos: i32,
}

impl MinoGridMap for Tetrimino {
    fn get_minos(&self) -> Vec<Mino> {
        self.minos
            .get_minos()
            .iter()
            .map(|mino| Mino {
                x: self.x_pos + mino.x,
                y: self.y_pos + mino.y,
                color: mino.color,
            })
            .collect()
    }
}

impl Default for Tetrimino {
    fn default() -> Self {
        Tetrimino::new(random())
    }
}

impl Tetrimino {
    /// Create a new Tetrimino
    pub fn new(tetrimino_type: TetriminoType) -> Tetrimino {
        let (x_pos, y_pos) = get_spawn_point(tetrimino_type);
        Tetrimino {
            tetrimino_type,
            minos: match tetrimino_type {
                TetriminoType::O => grid![
                    [None, Some(O_COLOR), Some(O_COLOR)]
                    [None, Some(O_COLOR), Some(O_COLOR)]
                    [None, None, None]
                ],
                TetriminoType::I => grid![
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                    [None, Some(I_COLOR), Some(I_COLOR), Some(I_COLOR), Some(I_COLOR)]
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                ],
                TetriminoType::T => grid![
                    [None, Some(T_COLOR), None]
                    [Some(T_COLOR), Some(T_COLOR), Some(T_COLOR)]
                    [None, None, None]
                ],
                TetriminoType::L => grid![
                    [None, None, Some(L_COLOR)]
                    [Some(L_COLOR), Some(L_COLOR), Some(L_COLOR)]
                    [None, None, None]
                ],
                TetriminoType::J => grid![
                    [Some(J_COLOR), None, None]
                    [Some(J_COLOR), Some(J_COLOR), Some(J_COLOR)]
                    [None, None, None]
                ],
                TetriminoType::S => grid![
                    [None, Some(S_COLOR), Some(S_COLOR)]
                    [Some(S_COLOR), Some(S_COLOR), None]
                    [None, None, None]
                ],
                TetriminoType::Z => grid![
                    [Some(Z_COLOR), Some(Z_COLOR), None]
                    [None, Some(Z_COLOR), Some(Z_COLOR)]
                    [None, None, None]
                ],
            },
            rotation: Facing::North,
            x_pos,
            y_pos,
        }
    }

    /// Return a [`Vec`] of all Tetriminos
    pub fn all() -> Vec<Tetrimino> {
        vec![
            Tetrimino::new(TetriminoType::O),
            Tetrimino::new(TetriminoType::I),
            Tetrimino::new(TetriminoType::T),
            Tetrimino::new(TetriminoType::L),
            Tetrimino::new(TetriminoType::J),
            Tetrimino::new(TetriminoType::S),
            Tetrimino::new(TetriminoType::Z),
        ]
    }

    pub fn preview(&self, index: usize) -> TetriminoPreview {
        TetriminoPreview {
            minos: self.minos.clone(),
            index,
        }
    }

    /// Check if the any mino is either:
    ///
    /// 1. outside the bounds of the board
    /// 2. colliding with another mino
    ///
    /// after moved by `x_offset` and `y_offset`
    ///
    /// [`None`] indicates the position is valid
    /// If [`Some`], there will always be at least one mino in the [`Vec`]
    ///
    /// ```
    /// let mino = (grid_x_position, grid_y_position, mino_color);
    /// ```
    pub fn position_invalid(
        &self,
        x_offset: i32,
        y_offset: i32,
        matrix: &MinoGrid,
    ) -> Option<Vec<Mino>> {
        let invalid: Vec<Mino> = self
            .get_minos()
            .iter()
            .filter_map(|mino| {
                let x = mino.x + x_offset;
                let y = mino.y + y_offset;

                let out_of_bounds = position_outside_bounds!(x, y);
                let board_collision = matrix
                    .get_minos()
                    .iter()
                    .any(|block| x == block.x && y == block.y);

                if out_of_bounds || board_collision {
                    // invalid minos are returned
                    Some(mino.to_owned())
                } else {
                    // valid minos are not
                    None
                }
            })
            .collect();

        if invalid.is_empty() {
            None
        } else {
            Some(invalid)
        }
    }

    /// Move the Tetrimino by `x` and `y`
    ///
    /// Returns `true` if the move was successful,
    /// `false` if the position would be invalid after the move
    pub fn move_position(&mut self, x: i32, y: i32, matrix: &MinoGrid) -> bool {
        // check if position would be invalid
        if self.position_invalid(x, y, matrix).is_some() {
            return false;
        }

        // move the piece if valid
        self.x_pos += x;
        self.y_pos += y;

        true
    }

    /// Rotate the Tetrimino
    ///
    /// Does nothing if the position would be invalid after the rotation
    pub fn rotate(&mut self, rotation_direction: RotationDirection, matrix: &MinoGrid) {
        // store the previous state in case rotation is impossible
        let original_minos = self.minos.to_owned();
        let original_rotation = self.rotation;

        // make the rotated grid
        let (rows, cols) = self.minos.size();
        self.minos = Grid::new(cols, rows);
        self.rotation = self.rotation.rotated(rotation_direction);

        // map the original minos to the new rotated grid
        original_minos
            .iter_rows()
            .enumerate()
            .for_each(|(row, col_iter)| {
                col_iter.enumerate().for_each(|(col, mino)| {
                    self.minos[match rotation_direction {
                        RotationDirection::Clockwise => col,
                        RotationDirection::Counterclockwise => cols - col - 1,
                    }][match rotation_direction {
                        RotationDirection::Clockwise => rows - row - 1,
                        RotationDirection::Counterclockwise => row,
                    }] = *mino
                })
            });

        // Super-Rotation-System uses an offset table to try and place Tetrimino
        for (x, y) in self
            .tetrimino_type
            .get_offset_data(original_rotation, self.rotation)
        {
            if self.move_position(x, y, matrix) {
                // position is okay
                return;
            }
        }

        // rotation is impossible, revert the state
        self.minos = original_minos;
        self.rotation = original_rotation;
    }
}
