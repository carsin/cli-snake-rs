extern crate crossterm;

use std::io::{stdout, Write};
use crossterm::{
    execute, queue, ExecutableCommand, QueueableCommand, cursor,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    Result,
};


mod map;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    stdout().execute(Clear(ClearType::All))?;
    let game = map::Map::initialize(20, 20);
    game.render();
    terminal::disable_raw_mode()?;

    Ok(())
}


