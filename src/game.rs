use std::collections::VecDeque;

use crate::{
    board::{get_matrix_size, MATRIX_HEIGHT, MATRIX_WIDTH},
    game_io::Movement,
    graphics::Mino,
    new_matrix,
    tetramino::Tetrimino,
};
use grid::Grid;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
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

/// A [`Bag`] is a self-filling [`Vec<Tetrimino>`]s
///
/// Calls to `next()` will yield shuffled sequences
#[derive(Debug, Clone)]
pub struct NextQueue {
    queue: VecDeque<Tetrimino>,
    bag: Vec<Tetrimino>,
    rng: ThreadRng,
}

impl PartialEq for NextQueue {
    fn eq(&self, other: &Self) -> bool {
        self.bag == other.bag
    }
}

impl Eq for NextQueue {}

impl Default for NextQueue {
    fn default() -> Self {
        let mut queue = Self {
            queue: VecDeque::new(),
            bag: vec![],
            rng: thread_rng(),
        };

        let mut next = (0..6).map(|_| queue.next_bag()).collect();
        queue.queue.append(&mut next);

        queue
    }
}

impl NextQueue {
    fn get_queue(&self) -> Vec<Tetrimino> {
        Vec::from(self.queue.to_owned())
    }

    fn next_bag(&mut self) -> Tetrimino {
        // fill and shuffle if empty
        if self.bag.is_empty() {
            self.bag = Tetrimino::all();
            self.bag.shuffle(&mut self.rng);
        }

        // bag is fed back to front but that order doesn't matter
        self.bag.pop().expect("Bag was empty!") //.unwrap_or_default()
    }

    fn next(&mut self) -> Tetrimino {
        // move a bag tetrimino into the queue
        let next = self.next_bag();
        self.queue.push_back(next);
        // provide an element of the queue
        self.queue.pop_front().expect("Queue was empty!") //.unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub running: bool,
    pub tetrimino: Tetrimino,
    pub next_queue: NextQueue,
    matrix: MinoGrid,
}

impl Default for GameState {
    fn default() -> Self {
        let mut next_queue = NextQueue::default();

        Self {
            running: true,
            tetrimino: next_queue.next(),
            next_queue,
            matrix: new_matrix!(),
        }
    }
}

impl GameState {
    pub fn next_tetrimino(&mut self) {
        // lock the current Tetrimino
        self.tetrimino
            .get_minos()
            .iter()
            .for_each(|mino| self.matrix[mino.y as usize][mino.x as usize] = Some(mino.color));

        // new Tetrimino
        self.tetrimino = self.next_queue.next();
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

        Canvas::default()
            .block(Block::default().title("TETRIS").borders(Borders::ALL))
            .x_bounds([0.0, 4.0])
            .y_bounds([0.0, MATRIX_HEIGHT.into()])
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| {
                if let Some(tetraminos) = state.next_queue.get_queue().chunks(6).next() {
                    for (index, tetramino) in tetraminos.iter().enumerate() {
                        ctx.draw(&tetramino.preview(index));
                    }
                } else {
                    println!("OH NO!");
                };
            })
            .render(layout[2], buf);
    }
}
