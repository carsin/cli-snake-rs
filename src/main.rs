extern crate crossterm;

use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

const GAME_WIDTH: usize = 30;
const GAME_HEIGHT: usize = GAME_WIDTH;
const TICK_SPEED: u64 = 5;

mod game;
mod input;

fn main() {
    let mut stdout = stdout();

    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    // Set up input
    let input_receiver = input::start_input_receiver();

    // Set up game
    let snake = game::Snake::new(1, 4, GAME_HEIGHT / 2, game::Direction::East);
    let mut game = game::Game::new(GAME_WIDTH, GAME_HEIGHT, snake);

    // Game loop
    let update_speed = Duration::from_millis(1000 / TICK_SPEED);
    let mut past = Instant::now();

    game.playing = true;
    while game.playing {
        let now = Instant::now();
        let dt = now.duration_since(past);
        past = now;

        // Listen for input
        while let Ok(char) = input_receiver.try_recv() {
            game.handle_input(char);
        }

        // Update
        game.update();

        // Render
        // Clear terminal
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        game.render_map();
        stdout.flush().unwrap();

        // Limit rate at which game is updated
        if dt < update_speed {
            sleep(update_speed - dt);
            continue;
        }
    }

    // Restore terminal after game is finished
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited");
}
