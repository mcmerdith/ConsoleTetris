use std::{
    sync::mpsc::{self, Receiver},
    thread::{self},
};

use crossterm::event::{self, KeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Rotate(RotationDirection),
    Left,
    Right,
    Down,
    Drop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// Close the game
    QuitGame,
    /// Move the Tetrimino
    Move(Movement),
    /// New Tetrimino
    NewTetrimino,
}

pub fn start_io_handler() -> Receiver<Message> {
    let (io_tx, io_rx) = mpsc::channel();
    thread::spawn(move || loop {
        let _ = io_tx.send(match event::read() {
            Ok(event::Event::Key(key)) => match key.code {
                KeyCode::Char(c) => match c {
                    'q' => {
                        let _ = io_tx.send(Message::QuitGame);
                        break;
                    }
                    'n' => Message::NewTetrimino,
                    'z' => Message::Move(Movement::Rotate(RotationDirection::Counterclockwise)),
                    _ => continue,
                },
                KeyCode::Up => Message::Move(Movement::Rotate(RotationDirection::Clockwise)),
                KeyCode::Left => Message::Move(Movement::Left),
                KeyCode::Right => Message::Move(Movement::Right),
                KeyCode::Down => Message::Move(Movement::Down),
                KeyCode::Enter => Message::Move(Movement::Drop),
                _ => continue,
            },
            _ => continue,
        });
    });
    io_rx
}
