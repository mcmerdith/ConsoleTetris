use crate::{
    game_handler::RotationDirection,
    matrix::{get_spawn_point, GridRotation, Matrix, MinoGrid},
    position_outside_bounds,
};
use grid::grid;
use rand::{distributions::Standard, prelude::Distribution};
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
    minos: Matrix,
    /// the index of the preview
    index: usize,
}

impl MinoGrid for TetriminoPreview {
    fn get_minos(&self) -> Vec<Mino> {
        self.minos
            .get_minos()
            .iter()
            .map(|mino| Mino {
                col: mino.col,
                row: mino.row + 3 * self.index as i32,
                color: mino.color,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mino {
    pub col: i32,
    pub row: i32,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tetrimino {
    // the type of the Tetrimino
    tetrimino_type: TetriminoType,
    /// the grid of minos making up the Tetrimino
    minos: Matrix,
    /// the column of the top-left corner of the bound-box
    col: i32,
    /// the row of the top-left corner of the bound-box
    row: i32,
}

impl MinoGrid for Tetrimino {
    fn get_minos(&self) -> Vec<Mino> {
        self.minos
            .get_minos()
            .iter()
            .map(|mino| Mino {
                col: self.col + mino.col,
                row: self.row - mino.row,
                color: mino.color,
            })
            .collect()
    }
}

impl Tetrimino {
    /// Create a new Tetrimino
    pub fn new(tetrimino_type: TetriminoType) -> Tetrimino {
        let (col, row) = get_spawn_point();

        Tetrimino {
            tetrimino_type,
            minos: Matrix::from(match tetrimino_type {
                TetriminoType::O => grid![
                    [None, None, None, None, None]
                    [None, None, Some(O_COLOR), Some(O_COLOR), None]
                    [None, None, Some(O_COLOR), Some(O_COLOR), None]
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                ],
                TetriminoType::I => grid![
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                    [None, Some(I_COLOR), Some(I_COLOR), Some(I_COLOR), Some(I_COLOR)]
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                ],
                TetriminoType::T => grid![
                    [None, None, None, None, None]
                    [None, None, Some(T_COLOR), None, None]
                    [None, Some(T_COLOR), Some(T_COLOR), Some(T_COLOR), None]
                    [None, None, None, None, None]
                ],
                TetriminoType::L => grid![
                    [None, None, None, None, None]
                    [None, None, None, Some(L_COLOR), None]
                    [None, Some(L_COLOR), Some(L_COLOR), Some(L_COLOR), None]
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                ],
                TetriminoType::J => grid![
                    [None, None, None, None, None]
                    [None, Some(J_COLOR), None, None, None]
                    [None, Some(J_COLOR), Some(J_COLOR), Some(J_COLOR), None]
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                ],
                TetriminoType::S => grid![
                    [None, None, None, None, None]
                    [None, None, Some(S_COLOR), Some(S_COLOR), None]
                    [None, Some(S_COLOR), Some(S_COLOR), None, None]
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                ],
                TetriminoType::Z => grid![
                    [None, None, None, None, None]
                    [None, Some(Z_COLOR), Some(Z_COLOR), None, None]
                    [None, None, Some(Z_COLOR), Some(Z_COLOR), None]
                    [None, None, None, None, None]
                    [None, None, None, None, None]
                ],
            }),
            col,
            row,
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
        col_offset: i32,
        row_offset: i32,
        matrix: &Matrix,
    ) -> Option<Vec<Mino>> {
        let invalid: Vec<Mino> = self
            .get_minos()
            .iter()
            .filter_map(|mino| {
                let col = mino.col + col_offset;
                let row = mino.row + row_offset;

                let out_of_bounds = position_outside_bounds!(col, row);
                let board_collision = matrix
                    .get_minos()
                    .iter()
                    .any(|block| col == block.col && row == block.row);

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
    pub fn move_position(&mut self, col: i32, row: i32, matrix: &Matrix) -> bool {
        // check if position would be invalid
        if self.position_invalid(col, row, matrix).is_some() {
            return false;
        }

        // move the piece if valid
        self.col += col;
        self.row += row;

        true
    }

    /// Rotate the Tetrimino
    ///
    /// Does nothing if the position would be invalid after the rotation
    pub fn rotate(&mut self, rotation_direction: RotationDirection, matrix: &Matrix) -> bool {
        // store the previous state in case rotation is impossible
        let original_minos = self.minos.to_owned();

        // make the rotated grid
        self.minos = self.minos.rotated(rotation_direction);

        // Super-Rotation-System uses an offset table to try and place Tetrimino
        for (x, y) in self
            .tetrimino_type
            .get_offset_data(original_minos.rotation, self.minos.rotation)
        {
            if self.move_position(x, y, matrix) {
                // position is okay
                return true;
            }
        }

        // rotation is impossible, revert the state
        self.minos = original_minos;
        false
    }
}
