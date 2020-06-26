use std::io::stdout;
use crossterm::{cursor, ExecutableCommand, QueueableCommand, style::Print};
use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall, Empty, Apple, Snake
}

pub enum Direction {
    North, South, East, West
}

struct Position {
    x: usize,
    y: usize,
}

pub struct Snake {
    pub length: usize,
    tail: Vec<Position>,
    pub alive: bool,
    x: usize,
    y: usize,
    direction: Direction,

}

impl Snake {
    pub fn new(init_length: usize, init_x: usize, init_y: usize, init_direction: Direction) -> Self {
        Snake {
            length: init_length,
            tail: vec![],
            alive: true,
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
    pub running: bool,
}

impl Game {
    pub fn new(width: usize, height: usize, snake: Snake) -> Self {
        let mut new_game = Game {
            width,
            height,
            tiles: vec![],
            snake,
            running: false,
        };

        new_game.init_map();
        new_game
    }

    pub fn update_snake(&mut self) {
        // Insert latest snake position into front of tail
        self.snake.tail.insert(0, Position { x: self.snake.x, y: self.snake.y });
        // Get position of last tail tile
        let dead_tail_index = self.get_index(self.snake.tail[self.snake.length - 1].x, self.snake.tail[self.snake.length - 1].y);
        // Set last tail tile to empty
        self.tiles[dead_tail_index] = Tile::Empty;

        // Update snake based on direction and next tile
        match self.snake.direction {
            Direction::North => {
                match self.tiles[self.get_index(self.snake.x, self.snake.y - 1)] {
                    Tile::Wall | Tile::Snake => self.snake.alive = false,
                    Tile::Empty => self.snake.y -= 1,
                    Tile::Apple => {
                        self.snake.length += 1;
                        self.snake.y -= 1;
                        self.place_apple();
                    },
                }
            },

            Direction::South => {
                match self.tiles[self.get_index(self.snake.x, self.snake.y + 1)] {
                    Tile::Wall | Tile::Snake => self.snake.alive = false,
                    Tile::Empty => self.snake.y += 1,
                    Tile::Apple => {
                        self.snake.length += 1;
                        self.snake.y += 1;
                        self.place_apple();
                    },
                }
            },

            Direction::West => {
                match self.tiles[self.get_index(self.snake.x - 1, self.snake.y)] {
                    Tile::Wall | Tile::Snake => self.snake.alive = false,
                    Tile::Empty => self.snake.x -= 1,
                    Tile::Apple => {
                        self.snake.length += 1;
                        self.snake.x -= 1;
                        self.place_apple();
                    },
                }
            },

            Direction::East => {
                match self.tiles[self.get_index(self.snake.x + 1, self.snake.y)] {
                    Tile::Wall | Tile::Snake => self.snake.alive = false,
                    Tile::Empty => self.snake.x += 1,
                    Tile::Apple => {
                        self.snake.length += 1;
                        self.snake.x += 1;
                        self.place_apple();
                    },
                }
            },
        }


        // Truncate tail to length
        self.snake.tail.truncate(self.snake.length - 1);
        // Set snake on map
        let head_index = self.get_index(self.snake.x, self.snake.y);
        self.tiles[head_index] = Tile::Snake;
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

    pub fn render_map(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let current_char;
                match self.tiles[self.get_index(x, y)] {
                    Tile::Empty => {
                        current_char = String::from("  ");
                    },
                    Tile::Wall => {
                        current_char = String::from("▒▒");
                    },
                    Tile::Apple => {
                        current_char = String::from("╪╪");
                    },
                    Tile::Snake => {
                        current_char = String::from("██");
                    },
                };

                stdout().queue(cursor::MoveTo((x * 2) as u16, y as u16)).unwrap()
                        .execute(Print(current_char)).unwrap();
            }
        }
    }


    fn init_map(&mut self) {
        self.tiles = vec![Tile::Empty; self.width * self.height];
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

        self.place_apple();
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn handle_input(&mut self, input_char: char) {
        match input_char {
            'q' => self.running = false,
            'w' => self.snake.direction = self::Direction::North,
            'a' => self.snake.direction = self::Direction::West,
            's' => self.snake.direction = self::Direction::South,
            'd' => self.snake.direction = self::Direction::East,
            'r' => self.place_apple(),
            _ => (),
        }
    }
}
