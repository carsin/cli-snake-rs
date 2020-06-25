extern crate crossterm;

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, style::Print};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};

const GAME_WIDTH: usize = 30;
const GAME_HEIGHT: usize = GAME_WIDTH;
const UPS: u64 = 10;
const UPDATE_SPEED: Duration = Duration::from_millis(1000 / UPS);

mod game;
mod input;

fn main() {
    stdout().execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().execute(cursor::Hide).unwrap();

    // Set up game
    let snake = game::Snake::new(1, 4, GAME_HEIGHT / 2, game::Direction::East);
    let game = game::Game::new(GAME_WIDTH, GAME_HEIGHT, snake);

    run(UPDATE_SPEED, game);

    // Restore terminal after game is finished
    stdout().execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited");
}

fn run(update_speed: Duration, mut game: game::Game) {
    let input_receiver = input::start_input_receiver();

    let mut next_time = Instant::now();
    println!("{:?}", next_time);

    game.running = true;
    while game.running {
        let current_time = Instant::now();
        if current_time >= next_time {
            next_time += update_speed;
            // Handle input
            while let Ok(char) = input_receiver.try_recv() {
                game.handle_input(char);
            }
            // Update
            game.update();

            // Render
            if current_time < next_time {
                stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
                game.render_map();
            }
        } else {
            let sleep_time = next_time.duration_since(current_time);
            if sleep_time > Duration::new(0, 0) {
                println!("sleeping for {:?}", sleep_time);
                sleep(sleep_time);
            }
        }
    }
}
