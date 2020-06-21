use std::io::{stdout, Write};
use crossterm::{cursor, QueueableCommand};
use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall, Empty, Apple
}

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut new_game = Game {
            width,
            height,
            tiles: vec![],
        };

        new_game.init_map();
        new_game
    }

    fn init_map(&mut self) {
        self.tiles = vec![Tile::Empty; self.width * self.height];
        // TODO: Refactor
        // Border the map with walls
        for x in 0..self.width {
            let top = self.get_index(x, 0);
            self.tiles[top] = Tile::Wall;
            let bot = self.get_index(x, self.height - 1);
            self.tiles[bot] = Tile::Wall;
        }

        for y in 0..self.height {
            let left = self.get_index(0, y);
            self.tiles[left] = Tile::Wall;
            let right = self.get_index(self.width - 1, y);
            self.tiles[right] = Tile::Wall;
        }

        let mut rng = rand::thread_rng();
        let random_tile = self.get_index(rng.gen_range(1, self.width - 1), rng.gen_range(1, self.height - 1));
        self.tiles[random_tile] = Tile::Apple;

    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn render_map(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let current_char = match self.tiles[self.get_index(x, y)] {
                    Tile::Empty => ".",
                    Tile::Wall => "#",
                    Tile::Apple => "$",
                };

                stdout().queue(cursor::MoveTo(x as u16, (y + 1) as u16)).unwrap()
                    .write(current_char.as_bytes()).unwrap();
            }
        }
    }
}
