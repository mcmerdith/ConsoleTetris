mod board;
mod game;
mod game_io;
mod graphics;
mod tetramino;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::{GameState, Tetris};
use game_io::{start_io_handler, Message};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use std::{io, panic};
use tetramino::Tetramino;

fn main() -> Result<(), io::Error> {
    // emergency handlers
    ctrlc::set_handler(|| println!("no")).expect("Error setting Ctrl-C handler");
    let old_panic = panic::take_hook();
    panic::set_hook(Box::new(move |v| {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen, DisableMouseCapture);
        old_panic(v);
    }));

    // create term
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run the game
    game_loop(&mut terminal)?;

    // cleanup term
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn game_loop(terminal: &mut Terminal<impl Backend>) -> Result<(), io::Error> {
    let mut gamestate = GameState::default();

    let (io_rx, io_thread) = start_io_handler();

    while gamestate.running {
        match io_rx.try_recv() {
            Ok(v) => match v {
                Message::QuitGame => gamestate.running = false,
                Message::Move(control) => gamestate.apply_movement(control),
                Message::Debug => gamestate.tetramino = Tetramino::default(),
                Message::NewPiece => gamestate.new_piece(),
            },
            Err(_) => (),
        };

        terminal.draw(|f| {
            f.render_stateful_widget(Tetris {}, f.size(), &mut gamestate);
        })?;
    }

    // wait for the IO thread to terminate
    let _ = io_thread.join();

    Ok(())
}
