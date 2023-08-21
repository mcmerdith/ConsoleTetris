use ratatui::{style::Color, widgets::canvas::Shape};

use crate::{
    game::{Game, NextQueue},
    matrix::{MinoGrid, MATRIX_HEIGHT, MATRIX_WIDTH, PREVIEW_MATRIX_WIDTH},
    position_outside_render_bounds,
    tetramino::{Mino, Tetrimino, TetriminoPreview},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderMino {
    col: i32,
    row: i32,
    cols: usize,
    rows: usize,
    color: Color,
}

impl Shape for RenderMino {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        if position_outside_render_bounds!(self.col, self.row) {
            return;
        }

        // get explicit bounds of the board
        let Some((x1, y1)) =
            painter.get_point(0.0,0.0)
            else { return; };
        let Some((x2, y2)) =
            painter.get_point(self.cols as f64,self.rows as f64)
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
        let block_x_size = width / self.cols;
        let block_y_size = height / self.rows;

        // starting corner
        let x_start_pos = start_x + (self.col as usize) * block_x_size;
        let y_start_pos = start_y + (self.row as usize) * block_y_size;

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

fn draw_minos(
    painter: &mut ratatui::widgets::canvas::Painter,
    minos: &Vec<Mino>,
    matrix_width: usize,
    matrix_height: usize,
) {
    for mino in minos.iter().map(|mino| RenderMino {
        col: mino.col,
        row: mino.row,
        cols: matrix_width,
        rows: matrix_height,
        color: mino.color,
    }) {
        mino.draw(painter);
    }
}

impl Shape for TetriminoPreview {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        draw_minos(
            painter,
            &self.get_minos(),
            PREVIEW_MATRIX_WIDTH.into(),
            MATRIX_HEIGHT.into(),
        );
    }
}

impl Shape for Tetrimino {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        draw_minos(
            painter,
            &self
                .get_minos()
                .iter()
                .map(|mino| Mino {
                    col: mino.col,
                    row: MATRIX_HEIGHT as i32 - mino.row - 1,
                    color: mino.color,
                })
                .collect(),
            MATRIX_WIDTH.into(),
            MATRIX_HEIGHT.into(),
        );
    }
}

impl Shape for Game {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        let (rows, cols) = self.matrix.size();

        draw_minos(
            painter,
            &self
                .matrix
                .get_minos()
                .iter()
                .map(|mino| Mino {
                    col: mino.col,
                    row: rows as i32 - mino.row - 1,
                    color: mino.color,
                })
                .collect(),
            cols,
            rows,
        );

        self.tetrimino.draw(painter);
    }
}

impl Shape for NextQueue {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        if let Some(tetraminos) = self.get_queue().chunks(6).next() {
            for (index, tetramino) in tetraminos.iter().enumerate() {
                tetramino.preview(index).draw(painter);
            }
        };
    }
}
