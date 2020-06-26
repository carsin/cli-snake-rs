extern crate crossterm;

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, style::Print};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};

const GAME_WIDTH: usize = 20;
const GAME_HEIGHT: usize = GAME_WIDTH;
const UPDATES_PER_SECONDS: u64 = 6;
const UPDATE_SPEED: Duration = Duration::from_millis(1000 / UPDATES_PER_SECONDS);

mod game;
mod input;

fn main() {
    // Set up terminal
    stdout().execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().execute(cursor::Hide).unwrap();
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    // Set up game
    let snake = game::Snake::new(1, 4, GAME_HEIGHT / 2, game::Direction::East);
    let game = game::Game::new(GAME_WIDTH, GAME_HEIGHT, snake);

    run(game);

    // Restore terminal after game is finished
    stdout().execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited");
}

fn run(mut game: game::Game) {
    let input_receiver = input::start_input_receiver();

    let mut next_time = Instant::now();
    game.running = true;
    while game.running {
        let current_time = Instant::now();
        if current_time >= next_time {
            next_time += UPDATE_SPEED;
            // Handle input
            while let Ok(char) = input_receiver.try_recv() {
                game.handle_input(char);
            }

            if game.snake.alive {
                // Update
                game.update_snake();

                // Render if we've updated
                if current_time < next_time {
                    game.render_map();
                }
            } else {
                stdout().queue(terminal::Clear(terminal::ClearType::All)).unwrap()
                        .queue(cursor::MoveTo(0, 0)).unwrap()
                        .queue(Print("GAME OVER")).unwrap()
                        .queue(cursor::MoveTo(0, 1)).unwrap()
                        .execute(Print("Press 'q' to exit")).unwrap();
            }
        } else {
            let sleep_time = next_time.duration_since(current_time);
            if sleep_time > Duration::new(0, 0) {
                sleep(sleep_time);
            }
        }
    }
}
