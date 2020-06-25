use std::io::stdout;
use crossterm::{cursor, QueueableCommand, style::Print};
use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    WallVert, WallHorz, Empty, Apple, Snake
}

pub enum Direction {
    North, South, East, West
}

pub struct Snake {
    pub length: usize,
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

impl Snake {
    pub fn new(init_length: usize, init_x: usize, init_y: usize, init_direction: Direction) -> Self {
        Snake {
            length: init_length,
            x: init_x,
            y: init_y,
            direction: init_direction,
        }
    }
}

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    pub snake: Snake,
    pub playing: bool,
}

impl Game {
    pub fn new(width: usize, height: usize, snake: Snake) -> Self {
        let mut new_game = Game {
            width,
            height,
            tiles: vec![],
            snake,
            playing: false,
        };

        new_game.init_map();
        new_game
    }

    fn init_map(&mut self) {
        self.tiles = vec![Tile::Empty; self.width * self.height];
        // Border the map with walls
        for x in 0..self.width {
            let top = self.get_index(x, 0);
            self.tiles[top] = Tile::WallHorz;
            let bot = self.get_index(x, self.height - 1);
            self.tiles[bot] = Tile::WallHorz;
        }

        for y in 0..self.height {
            let left = self.get_index(0, y);
            self.tiles[left] = Tile::WallVert;
            let right = self.get_index(self.width - 1, y);
            self.tiles[right] = Tile::WallVert;
        }

        self.place_apple();
    }

    pub fn place_apple(&mut self) {
        let mut placing = true;
        let mut rng = rand::thread_rng();
        while placing {
            let random_tile = self.get_index(rng.gen_range(1, self.width - 1), rng.gen_range(1, self.height - 1));
            if self.tiles[random_tile] == Tile::Empty {
                self.tiles[random_tile] = Tile::Apple;
                placing = false;
            }
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn update(&mut self) {
        // Update tail
        let index = self.get_index(self.snake.x, self.snake.y);
        self.tiles[index] = Tile::Empty;

        // Move snake
        // TODO: Implement checks for next movement (wall / snake = lose, apple = grow)
        match self.snake.direction {
            Direction::North => self.snake.y -= 1,
            Direction::South => self.snake.y += 1,
            Direction::East => self.snake.x += 1,
            Direction::West => self.snake.x -= 1,
            _ => (),
        }

        // Set snake on map
        let index = self.get_index(self.snake.x, self.snake.y);
        self.tiles[index] = Tile::Snake;
    }

    pub fn render_map(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let current_char;
                match self.tiles[self.get_index(x, y)] {
                    Tile::Empty => {
                        current_char = String::from("  ");
                    },
                    Tile::WallHorz => {
                        current_char = String::from("══");
                    },
                    Tile::WallVert => {
                        current_char = String::from("┃┃");
                    },
                    Tile::Apple => {
                        current_char = String::from("╪╪");
                    },
                    Tile::Snake => {
                        current_char = String::from("██");
                    },
                };

                stdout().queue(cursor::MoveTo((x * 2) as u16, y as u16)).unwrap()
                        .queue(Print(current_char)).unwrap();
            }
        }
    }

    pub fn handle_input(&mut self, input_char: char) {
        match input_char {
            'q' => self.playing = false,
            'w' => self.snake.direction = self::Direction::North,
            'a' => self.snake.direction = self::Direction::West,
            's' => self.snake.direction = self::Direction::South,
            'd' => self.snake.direction = self::Direction::East,
            'r' => self.place_apple(),
            _ => (),
        }
    }
}
