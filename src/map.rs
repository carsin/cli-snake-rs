use std::io::{stdout, Write};

use crossterm::{
    execute, queue, ExecutableCommand, QueueableCommand, cursor,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall, Empty
}

pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn initialize(width: usize, height: usize) -> Self {
        let mut this = Self {
            width: width,
            height: height,
            tiles: vec![Tile::Empty; width * height],
        };

        this
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn render(&self) -> Result<()> {
        for x in 0..self.width {
            for y in 0..self.height {
                let tile_char = match self.tiles[self.get_index(x, y)] {
                    Tile::Empty => "_",
                    Tile::Wall => "#",
                };
                stdout().queue(cursor::MoveTo(x as u16, y as u16))?
                        .queue(Print(tile_char))?;

            }
        }
        stdout().flush().unwrap();
        Ok(())
    }
}


