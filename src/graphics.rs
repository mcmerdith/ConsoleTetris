use ratatui::{style::Color, widgets::canvas::Shape};

use crate::{
    board::{BOARD_HEIGHT, BOARD_WIDTH},
    position_outside_bounds,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Shape for Tile {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        if position_outside_bounds!(self.x, self.y) {
            return;
        }

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

        // starting corner
        let x_start_pos = start_x + (self.x as usize) * block_x_size;
        let y_start_pos = start_y + (self.y as usize) * block_y_size;

        // ending corner
        let x_end_pos = x_start_pos + block_x_size;
        let y_end_pos = y_start_pos + block_y_size;

        // paint tile
        for x in x_start_pos..x_end_pos {
            for y in y_start_pos..y_end_pos {
                painter.paint(x, y, self.color);
            }
        }
    }
}
