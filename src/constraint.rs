use std::cmp::min;

pub const BOARD_WIDTH: u16 = 10;
pub const BOARD_HEIGHT: u16 = 20;

/// Expands to a boolean expression evaluating if `x` is outside the left bound of the board
///
/// ```
/// let x: i32 = ..;
/// if x_position_outside_left_bound!(x) ..
/// ```
#[macro_export]
macro_rules! x_position_outside_left_bound {
    ($x: expr) => {
        $x < 0 || $x >= BOARD_WIDTH.into()
    };
}

/// Expands to a boolean expression evaluating if `x` and `y` is outside the right bound of the board
///
/// ```
/// let x: i32 = ..;
/// if x_position_outside_right_bound!(x) ..
/// ```
#[macro_export]
macro_rules! x_position_outside_right_bound {
    ($x: expr) => {
        $x < 0 || $x >= BOARD_WIDTH.into()
    };
}

/// Expands to a boolean expression evaluating if `x` si outside the bounds of the board
///
/// ```
/// let x: i32 = ..;
/// if x_position_outside_bounds!(x, y) ..
#[macro_export]
macro_rules! x_position_outside_bounds {
    ($x: expr) => {
        $x < 0 || $x >= BOARD_WIDTH.into()
    };
}

/// Expands to a boolean expression evaluating if `y` is outside the bounds of the board
///
/// ```
/// let y: i32 = ..;
/// if y_position_outside_bounds!(y) ..
#[macro_export]
macro_rules! y_position_outside_bounds {
    ($y: expr) => {
        $y < 0 || $y >= BOARD_HEIGHT.into()
    };
}

/// Expands to a boolean expression evaluating if `x` and `y` are outside the bounds of the board
///
/// ```
/// let x: i32 = ..;
/// let y: i32 = ..;
/// if position_outside_bounds!(x, y) ..
#[macro_export]
macro_rules! position_outside_bounds {
    ($x: expr, $y: expr) => {
        $crate::x_position_outside_bounds!($x) || $crate::y_position_outside_bounds!($y)
    };
}

/// Get the `(width, height, horizontal margin)` required for the board
///
/// Returns [`None`] if the screen is too small
pub fn get_board_size(vw_width: u16, vw_height: u16) -> Option<(u16, u16, u16)> {
    if vw_width < 12 || vw_height < 22 {
        return None;
    }

    let canvas_height = vw_height - 2;
    let board_height = canvas_height - (canvas_height % BOARD_HEIGHT);
    let board_width = board_height * BOARD_WIDTH / BOARD_HEIGHT;
    let margin = (vw_width - board_width) / 2;

    Some((
        min(board_width + 2, vw_width),
        min(board_height + 2, vw_height),
        margin,
    ))
}
