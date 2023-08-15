use crate::{
    board::{get_matrix_size, MATRIX_HEIGHT, MATRIX_WIDTH},
    game_io::Movement,
    graphics::Mino,
    new_matrix,
    tetramino::Tetrimino,
};
use grid::Grid;
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::Color,
    widgets::{canvas::Canvas, Block, Borders, Paragraph, StatefulWidget, Widget},
};

pub type MinoGrid = Grid<Option<Color>>;

pub trait MinoGridMap {
    /// Returns a [`Vec`] of all non-empty minos
    ///
    /// ```
    /// let mino = (grid_x_position, grid_y_position, tile_color);
    /// ```
    fn get_minos(&self) -> Vec<Mino>;
}

impl MinoGridMap for MinoGrid {
    fn get_minos(&self) -> Vec<Mino> {
        self.iter_rows()
            .enumerate()
            .flat_map(|(row, row_iter)| {
                row_iter.enumerate().filter_map(move |(col, tile)| {
                    return match tile {
                        Some(color) => Some(Mino {
                            x: col as i32,
                            y: -(row as i32),
                            color: *color,
                        }),
                        None => None,
                    };
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub running: bool,
    pub tetrimino: Tetrimino,
    matrix: MinoGrid,
}

impl Default for GameState {
    fn default() -> Self {
        let matrix = new_matrix!();
        GameState {
            running: true,
            tetrimino: Tetrimino::default(),
            matrix,
        }
    }
}

impl GameState {
    pub fn new_tetrimino(&mut self) {
        // lock the current tetrimino
        self.tetrimino
            .get_minos()
            .iter()
            .for_each(|tile| self.matrix[tile.y as usize][tile.x as usize] = Some(tile.color));

        // new tetrimino
        self.tetrimino = Tetrimino::default();
    }

    pub fn apply_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Rotate(rotation) => {
                self.tetrimino.rotate(rotation, &self.matrix);
            }
            Movement::Left => {
                self.tetrimino.move_position(-1, 0, &self.matrix);
            }
            Movement::Right => {
                self.tetrimino.move_position(1, 0, &self.matrix);
            }
            Movement::Down => {
                self.tetrimino.move_position(0, -1, &self.matrix);
            }
            Movement::Drop => return,
        }
    }
}

pub struct Tetris;

impl StatefulWidget for Tetris {
    type State = GameState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let Some((board_width, board_height, margin)) = get_matrix_size(area.width, area.height)
        else {
            Paragraph::new("This terminal is too small to play Tetris!").render(area, buf);
            return;
        };

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(board_height),
                Constraint::Length(area.height - board_height),
            ])
            .split(area);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(margin),
                Constraint::Length(board_width),
                Constraint::Length(margin),
            ])
            .split(vertical_layout[0]);

        Canvas::default()
            .block(Block::default().title("TETRIS").borders(Borders::ALL))
            .x_bounds([0.0, MATRIX_WIDTH.into()])
            .y_bounds([0.0, MATRIX_HEIGHT.into()])
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| {
                ctx.draw(&state.tetrimino);
                state
                    .matrix
                    .get_minos()
                    .iter()
                    .for_each(|mino| ctx.draw(mino));
            })
            .render(layout[1], buf);
    }
}
