use grid::Grid;
use ratatui::style::Color;

use crate::{
    game_io::RotationDirection,
    tetramino::{Facing, Mino, TetriminoType},
};

pub const MATRIX_WIDTH: u16 = 10;
pub const MATRIX_HEIGHT: u16 = 20;

pub const PREVIEW_MATRIX_WIDTH: u16 = 6;

/// Check if `x` is outside the left bound of the matrix
///
/// ```
/// let x: i32 = ..;
/// if x_position_outside_left_bound!(x) ..
/// ```
#[macro_export]
macro_rules! x_position_outside_left_bound {
    ($x: expr) => {
        $x < 0
    };
}

/// Check if `x` is outside the right bound of the matrix
///
/// ```
/// let x: i32 = ..;
/// if x_position_outside_right_bound!(x) ..
/// ```
#[macro_export]
macro_rules! x_position_outside_right_bound {
    ($x: expr) => {
        $x >= $crate::matrix::MATRIX_WIDTH.into()
    };
}

/// Check if `x` is outside the bounds of the matrix
///
/// ```
/// let x: i32 = ..;
/// if x_position_outside_bounds!(x) ..
/// ```
#[macro_export]
macro_rules! x_position_outside_bounds {
    ($x: expr) => {
        $crate::x_position_outside_left_bound!($x) || $crate::x_position_outside_right_bound!($x)
    };
}

/// Check if `y` is outside the bottom bound of the matrix
///
/// ```
/// let y: i32 = ..;
/// if y_position_outside_bottom_bound!(y) ..
/// ```
#[macro_export]
macro_rules! y_position_outside_bottom_bound {
    ($y: expr) => {
        $y < 0
    };
}

/// Check if `y` is outside the top bound of the matrix
///
/// ```
/// let y: i32 = ..;
/// if y_position_outside_top_bound!(y) ..
/// ```
#[macro_export]
macro_rules! y_position_outside_top_bound {
    ($y: expr) => {
        $y >= $crate::matrix::MATRIX_HEIGHT.into()
    };
}

/// Check if `y` is outside the bounds of the matrix
///
/// ```
/// let y: i32 = ..;
/// if y_position_outside_bounds!(y) ..
/// ```
#[macro_export]
macro_rules! y_position_outside_bounds {
    ($y: expr) => {
        $crate::y_position_outside_top_bound!($y) || $crate::y_position_outside_bottom_bound!($y)
    };
}

/// Check if `x` and `y` are outside the bounds of the matrix (excluding the top)
///
/// ```
/// let x: i32 = ..;
/// let y: i32 = ..;
/// if position_outside_bounds!(x, y) ..
/// ```
#[macro_export]
macro_rules! position_outside_bounds {
    ($x: expr, $y: expr) => {
        $crate::x_position_outside_bounds!($x) || $crate::y_position_outside_bottom_bound!($y)
    };
}

/// Check if `x` and `y` are outside the bounds of the matrix
///
/// ```
/// let x: i32 = ..;
/// let y: i32 = ..;
/// if position_outside_render_bounds!(x, y) ..
/// ```
#[macro_export]
macro_rules! position_outside_render_bounds {
    ($x: expr, $y: expr) => {
        $crate::x_position_outside_bounds!($x) || $crate::y_position_outside_bounds!($y)
    };
}

/// Get the `(width, height, preview_width, horizontal margin)` required for the matrix
///
/// Returns [`None`] if the screen is too small
pub fn get_matrix_size(vw_width: u16, vw_height: u16) -> Option<(u16, u16, u16, u16)> {
    let required_width = MATRIX_WIDTH * 2 + PREVIEW_MATRIX_WIDTH * 2 + 4;
    let required_height = MATRIX_HEIGHT + 2;

    if vw_width < required_width || vw_height < required_height {
        return None;
    }

    let canvas_height = vw_height - 2;
    let board_height = canvas_height - (canvas_height % MATRIX_HEIGHT);

    let board_width = (board_height * MATRIX_WIDTH / MATRIX_HEIGHT) * 2;
    let preview_width = (board_height * PREVIEW_MATRIX_WIDTH / MATRIX_HEIGHT) * 2;

    let margin = (vw_width - board_width - preview_width) / 2;

    Some((board_width + 2, board_height + 2, preview_width + 2, margin))
}

/// Get the spawn point of a tetramino
///
/// Returns `(x, y)`
pub fn get_spawn_point(piece: TetriminoType) -> (i32, i32) {
    (
        match piece {
            TetriminoType::I => 3,
            TetriminoType::J => 3,
            TetriminoType::L => 3,
            TetriminoType::O => 4,
            TetriminoType::S => 3,
            TetriminoType::T => 3,
            TetriminoType::Z => 3,
        },
        (MATRIX_HEIGHT - 1).into(),
    )
}

pub trait MinoGrid {
    /// Returns a [`Vec`] of all non-empty minos
    fn get_minos(&self) -> Vec<Mino>;
}

pub trait GridRotation {
    fn rotated(&self, direction: RotationDirection) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    matrix: Grid<Option<Color>>,
    pub rotation: Facing,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, rotation: Facing) -> Self {
        Self {
            rows,
            cols,
            matrix: Grid::new(rows, cols),
            rotation,
        }
    }

    /// rows, cols
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get_mino(&self, row: usize, col: usize) -> Option<Mino> {
        match self.matrix.get(row, col) {
            Some(mino) => mino.map(|color| Mino {
                col: col as i32,
                row: row as i32,
                color: color,
            }),
            None => None,
        }
    }

    pub fn get_matrix(&self) -> Grid<Option<Color>> {
        self.matrix.to_owned()
    }

    pub fn set_mino(&mut self, mino: Mino) {
        if position_outside_render_bounds!(mino.col, mino.row) {
            return;
        }

        self.matrix[mino.row as usize][mino.col as usize] = Some(mino.color);
    }
}

impl From<Grid<Option<Color>>> for Matrix {
    fn from(value: Grid<Option<Color>>) -> Self {
        Self {
            rows: value.rows(),
            cols: value.cols(),
            matrix: value,
            rotation: Facing::North,
        }
    }
}

impl MinoGrid for Matrix {
    fn get_minos(&self) -> Vec<Mino> {
        self.matrix
            .iter_rows()
            .enumerate()
            .flat_map(|(row, row_iter)| {
                row_iter.enumerate().filter_map(move |(col, tile)| {
                    return match tile {
                        Some(color) => Some(Mino {
                            col: col as i32,
                            row: row as i32,
                            color: *color,
                        }),
                        None => None,
                    };
                })
            })
            .collect()
    }
}

impl GridRotation for Matrix {
    fn rotated(&self, direction: RotationDirection) -> Self {
        let (rows, cols) = self.size();
        let mut rotated = Matrix::new(cols, rows, self.rotation.rotated(direction));

        // map the original minos to the new rotated grid
        for (row, row_iter) in self.matrix.iter_rows().enumerate() {
            for (col, mino) in row_iter.enumerate() {
                rotated.matrix[match direction {
                    RotationDirection::Clockwise => col,
                    RotationDirection::Counterclockwise => cols - col - 1,
                }][match direction {
                    RotationDirection::Clockwise => rows - row - 1,
                    RotationDirection::Counterclockwise => row,
                }] = *mino
            }
        }

        rotated
    }
}
