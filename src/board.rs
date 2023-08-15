use std::cmp::min;

use crate::tetramino::TetriminoType;

pub const MATRIX_WIDTH: u16 = 10;
pub const MATRIX_HEIGHT: u16 = 20;

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
        $x >= MATRIX_WIDTH.into()
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
        $y >= MATRIX_HEIGHT.into()
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

/// Create a new game matrix
///
/// ```
/// let game_board = new_matrix!();
/// ```
#[macro_export]
macro_rules! new_matrix {
    () => {
        Grid::new(MATRIX_HEIGHT.into(), MATRIX_WIDTH.into())
    };
}

/// Get the `(width, height, horizontal margin)` required for the matrix
///
/// Returns [`None`] if the screen is too small
pub fn get_matrix_size(vw_width: u16, vw_height: u16) -> Option<(u16, u16, u16)> {
    if vw_width < (MATRIX_WIDTH * 2 + 2) || vw_height < (MATRIX_HEIGHT + 2) {
        return None;
    }

    let canvas_height = vw_height - 2;
    let board_height = canvas_height - (canvas_height % MATRIX_HEIGHT);
    let board_width = (board_height * MATRIX_WIDTH / MATRIX_HEIGHT) * 2;
    let margin = (vw_width - board_width) / 2;

    Some((
        min(board_width + 2, vw_width),
        min(board_height + 2, vw_height),
        margin,
    ))
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
