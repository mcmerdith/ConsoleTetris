use ratatui::{style::Color, widgets::canvas::Shape};

use crate::{
    board::{MATRIX_HEIGHT, MATRIX_WIDTH},
    game::MinoGridMap,
    position_outside_render_bounds,
    tetramino::{Tetrimino, TetriminoPreview},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mino {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Mino {
    pub fn get_render_position(&self) -> (i32, i32) {
        (self.x, MATRIX_HEIGHT as i32 - self.y - 1)
    }
}

impl Shape for Mino {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        if position_outside_render_bounds!(self.x, self.y) {
            return;
        }

        // get explicit bounds of the board
        let Some((x1, y1)) =
            painter.get_point(0.0,0.0)
            else { return; };
        let Some((x2, y2)) =
            painter.get_point(MATRIX_WIDTH.into(),MATRIX_HEIGHT.into())
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
        let block_x_size = width / MATRIX_WIDTH as usize;
        let block_y_size = height / MATRIX_HEIGHT as usize;

        // starting corner
        let x_start_pos = start_x + (self.x as usize) * block_x_size;
        let y_start_pos = start_y + (self.y as usize) * block_y_size;

        // ending corner
        let x_end_pos = x_start_pos + block_x_size;
        let y_end_pos = y_start_pos + block_y_size;

        // paint mino
        for x in x_start_pos..x_end_pos {
            for y in y_start_pos..y_end_pos {
                painter.paint(x, y, self.color);
            }
        }
    }
}

impl Shape for Tetrimino {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for mino in self.get_minos().iter().map(|mino| {
            let (x, y) = mino.get_render_position();
            Mino {
                x,
                y,
                color: mino.color,
            }
        }) {
            mino.draw(painter);
        }
    }
}

impl Shape for TetriminoPreview {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for mino in self.get_minos() {
            mino.draw(painter);
            //print!(" {},{}", mino.x, -mino.y);
        }
        //print!(";");
    }
}
