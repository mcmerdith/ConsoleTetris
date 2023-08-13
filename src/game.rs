use crate::{
    constraint::{get_board_size, BOARD_HEIGHT, BOARD_WIDTH},
    game_io::Movement,
    tetramino::{Tetramino, Tile},
};
use grid::{grid, Grid};
use rand::random;
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::Color,
    widgets::{canvas::Canvas, Block, Borders, Paragraph, StatefulWidget, Widget},
};

pub type TileGrid = Grid<Option<Color>>;

pub trait TileGridMap {
    /// Returns a [`Vec`] of all non-empty tiles
    ///
    /// ```
    /// let tile = (grid_x_position, grid_y_position, tile_color);
    /// ```
    fn get_tiles(&self) -> Vec<Tile>;
}

impl TileGridMap for TileGrid {
    fn get_tiles(&self) -> Vec<Tile> {
        self.iter_rows()
            .enumerate()
            .flat_map(|(row, row_iter)| {
                row_iter.enumerate().filter_map(move |(col, tile)| {
                    return match tile {
                        Some(color) => Some((col as i32, row as i32, color.to_owned())),
                        None => None,
                    };
                })
            })
            .collect()
    }
}

pub struct GameState {
    pub running: bool,
    pub tetramino: Tetramino,
    blocks: TileGrid,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            running: true,
            tetramino: Tetramino::new(random(), 0, 0),
            blocks: grid![],
        }
    }
}

impl GameState {
    fn new_piece(&mut self) {
        // solidify the current piece

        // new tetramino
        self.tetramino = Tetramino::new(random(), 0, 0);
    }

    pub fn apply_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Rotate => {
                self.tetramino.rotate();
            }
            Movement::Left => {
                self.tetramino.move_position(-1, 0);
            }
            Movement::Right => {
                self.tetramino.move_position(1, 0);
            }
            Movement::Down => {
                self.tetramino.move_position(0, 1);
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
        let Some((board_width, board_height, margin)) = get_board_size(area.width, area.height)
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
            .x_bounds([0.0, BOARD_WIDTH.into()])
            .y_bounds([0.0, BOARD_HEIGHT.into()])
            .marker(ratatui::symbols::Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&state.tetramino);
            })
            .render(layout[1], buf);
    }
}
