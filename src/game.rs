use std::collections::VecDeque;

use crate::{
    game_handler::Movement,
    matrix::{
        get_matrix_size, Matrix, MinoGrid, MATRIX_HEIGHT, MATRIX_WIDTH, PREVIEW_MATRIX_WIDTH,
    },
    tetramino::{Facing, Tetrimino},
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    widgets::{canvas::Canvas, Block, Borders, Paragraph, StatefulWidget, Widget},
};

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
    pub fn get_queue(&self) -> Vec<Tetrimino> {
        Vec::from(self.queue.to_owned())
    }

    fn next_bag(&mut self) -> Tetrimino {
        // fill and shuffle if empty
        if self.bag.is_empty() {
            self.bag = Tetrimino::all();
            self.bag.shuffle(&mut self.rng);
        }

        // bag is fed back to front but that order doesn't matter
        self.bag.pop().expect("Bag was empty!")
    }

    pub fn next(&mut self) -> Tetrimino {
        // move a bag tetrimino into the queue
        let next = self.next_bag();
        self.queue.push_back(next);
        // provide an element of the queue
        self.queue.pop_front().expect("Queue was empty!")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub tetrimino: Tetrimino,
    pub matrix: Matrix,
}

impl Game {
    pub fn next_tetrimino(&mut self, tetrimino: Tetrimino) {
        // lock the current Tetrimino
        for mino in self.tetrimino.get_minos() {
            self.matrix.set_mino(mino.to_owned());
        }

        // new Tetrimino
        self.tetrimino = tetrimino;
    }

    pub fn apply_movement(&mut self, movement: Movement) -> bool {
        match movement {
            Movement::Rotate(rotation) => self.tetrimino.rotate(rotation, &self.matrix),
            Movement::Left => self.tetrimino.move_position(-1, 0, &self.matrix),
            Movement::Right => self.tetrimino.move_position(1, 0, &self.matrix),
            Movement::Down => self.tetrimino.move_position(0, -1, &self.matrix),
            Movement::Drop => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub running: bool,
    pub next_queue: NextQueue,
    pub game: Game,
}

impl Default for GameState {
    fn default() -> Self {
        let mut next_queue = NextQueue::default();
        let tetrimino = next_queue.next();

        Self {
            running: true,
            next_queue,
            game: Game {
                tetrimino,
                matrix: Matrix::new(MATRIX_HEIGHT.into(), MATRIX_WIDTH.into(), Facing::North),
            },
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
        let Some((board_width, board_height, preview_width, margin)) =
            get_matrix_size(area.width, area.height)
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
                Constraint::Length(preview_width),
                Constraint::Length(margin),
            ])
            .split(vertical_layout[0]);

        Canvas::default()
            .block(Block::default().title("TETRIS").borders(Borders::ALL))
            .x_bounds([0.0, MATRIX_WIDTH.into()])
            .y_bounds([0.0, MATRIX_HEIGHT.into()])
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| ctx.draw(&state.game))
            .render(layout[1], buf);

        Canvas::default()
            .block(Block::default().title("TETRIS").borders(Borders::ALL))
            .x_bounds([0.0, PREVIEW_MATRIX_WIDTH.into()])
            .y_bounds([0.0, MATRIX_HEIGHT.into()])
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| ctx.draw(&state.next_queue))
            .render(layout[2], buf);
    }
}
