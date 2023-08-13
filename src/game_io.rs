use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use crossterm::event::{self, KeyCode};

/// Controls for the game

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Movement {
    Rotate,
    Left,
    Right,
    Down,
    Drop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// Close the game
    QuitGame,
    /// Move the piece
    Move(Movement),
    /// Debug
    Debug,
}

pub fn start_io_handler() -> (Receiver<Message>, JoinHandle<()>) {
    let (io_tx, io_rx) = mpsc::channel();
    (
        io_rx,
        thread::spawn(move || loop {
            let _ = io_tx.send(match event::read() {
                Ok(event::Event::Key(key)) => match key.code {
                    KeyCode::Char(c) => {
                        if c == 'q' {
                            let _ = io_tx.send(Message::QuitGame);
                            break;
                        } else if c == 'r' {
                            Message::Debug
                        } else {
                            continue;
                        }
                    }
                    KeyCode::Up => Message::Move(Movement::Rotate),
                    KeyCode::Left => Message::Move(Movement::Left),
                    KeyCode::Right => Message::Move(Movement::Right),
                    KeyCode::Down => Message::Move(Movement::Down),
                    KeyCode::Enter => Message::Move(Movement::Drop),
                    _ => continue,
                },
                _ => continue,
            });
        }),
    )
}
