use crate::{
    board::{get_board_size, BOARD_HEIGHT, BOARD_WIDTH},
    game_io::Movement,
    graphics::Tile,
    new_board,
    tetramino::Tetramino,
};
use grid::Grid;
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
                        Some(color) => Some(Tile {
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
    pub tetramino: Tetramino,
    board_tile_grid: TileGrid,
}

impl Default for GameState {
    fn default() -> Self {
        let board_tiles = new_board!();
        GameState {
            running: true,
            tetramino: Tetramino::default(),
            board_tile_grid: board_tiles,
        }
    }
}

impl GameState {
    pub fn new_piece(&mut self) {
        // solidify the current piece
        self.tetramino.get_tiles().iter().for_each(|tile| {
            self.board_tile_grid[tile.y as usize][tile.x as usize] = Some(tile.color)
        });

        // new tetramino
        self.tetramino = Tetramino::default();
    }

    pub fn apply_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Rotate(rotation) => {
                self.tetramino.rotate(rotation, &self.board_tile_grid);
            }
            Movement::Left => {
                self.tetramino.move_position(-1, 0, &self.board_tile_grid);
            }
            Movement::Right => {
                self.tetramino.move_position(1, 0, &self.board_tile_grid);
            }
            Movement::Down => {
                self.tetramino.move_position(0, -1, &self.board_tile_grid);
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
            // .y_bounds([BOARD_HEIGHT.into(), 0.0])
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| {
                ctx.draw(&state.tetramino);
                state
                    .board_tile_grid
                    .get_tiles()
                    .iter()
                    .for_each(|tile| ctx.draw(tile));
            })
            .render(layout[1], buf);
    }
}
