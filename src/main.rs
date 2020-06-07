extern crate crossterm;

use std::io::{stdout, Write};
use crossterm::{
    execute, queue, ExecutableCommand, QueueableCommand, cursor,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};


mod game;
mod input;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut game = game::Game {
        width: 30,
        height: 20,
        tiles: vec![],
    };

    game.tiles = game.init_map();

    async_std::task::block_on(input::print_events());
    game.render_map().expect("Failed to render game.");

    // stdout().execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
