use std::io::{Write};

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall, Empty
}

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl Game {
    pub fn init_map(&self) -> Vec<Tile> {
        let mut map = vec![Tile::Empty; self.width * self.height];

        // Border the map with walls
        for x in 0..self.width {
            map[self.get_index(x, 0)] = Tile::Wall;
            map[self.get_index(x, self.height - 1)] = Tile::Wall;
        }

        for y in 0..self.height {
            map[self.get_index(0, y)] = Tile::Wall;
            map[self.get_index(self.width - 1, y)] = Tile::Wall;
        }

        map
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn render_map(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>){
        for x in 0..self.width {
            for y in 0..self.height {
                let tile_char = match self.tiles[self.get_index(x, y)] {
                    Tile::Empty => ".",
                    Tile::Wall => "#",
                };

                write!(stdout, "{}{}", termion::cursor::Goto((x + 1) as u16, (y + 1) as u16), tile_char).unwrap();
            }
        }
    }
}


