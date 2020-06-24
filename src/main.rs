extern crate crossterm;

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdin, stdout, Read, Write};
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::{Duration, Instant};

const GAME_WIDTH: usize = 30;
const GAME_HEIGHT: usize = GAME_WIDTH / 2;

mod game;

fn main() {
    let mut stdout = stdout();

    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    // Set up input
    let (input_sender, input_receiver) = channel::<char>();
    std::thread::spawn(move || {
        loop {
            let mut buf = [0u8; 1];
            stdin().read_exact(&mut buf).unwrap();
            input_sender.send(buf[0] as char).unwrap();
        }
    });

    // Set up game
    let snake = game::Snake::new(1, 4, GAME_HEIGHT / 2, game::Direction::East);
    let mut game = game::Game::new(GAME_WIDTH, GAME_HEIGHT, snake);

    // Game loop
    let update_speed = Duration::from_millis(1000 / 60); // 60 TPS
    let mut past = Instant::now();

    let mut playing = true;
    while playing {
        let now = Instant::now();
        let dt = now.duration_since(past);
        past = now;

        // Clear terminal
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

        // Listen for input
        while let Ok(char) = input_receiver.try_recv() {
            match char {
                'q' => playing = false,
                'a' => game.place_apple(),
                _ => (),
            }
        }

        // Update
        game.update();

        // Render
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
