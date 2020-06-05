extern crate crossterm;

use std::io::{stdout, Write};
use crossterm::{
    execute, queue, ExecutableCommand, QueueableCommand, cursor,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};


mod game;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut game = game::Game {
        width: 30,
        height: 20,
        tiles: vec![],
    };

    game.tiles = game.init_map();

    game.render_map().expect("Failed to render game.");

    terminal::disable_raw_mode()?;
    Ok(())
}


