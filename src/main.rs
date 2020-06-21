extern crate crossterm;

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdin, stdout, Read, Write};
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
    let game = game::Game::new(30, 15);

    // Game loop
    let update_speed = Duration::from_millis(1000 / 60); // 60 TPS
    let mut past = Instant::now();

    let mut playing = true;
    while playing {
        let now = Instant::now();
        let dt = now.duration_since(past);
        past = now;

        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

        // Listen for input
        while let Ok(ctrl) = input_receiver.try_recv() {
            match ctrl {
                'q' => playing = false,
                _ => (),
            }
        }

        // Render
        stdout.queue(cursor::MoveTo(0, 0)).unwrap()
            .write("hello :D".as_bytes()).unwrap();

        game.render_map();

        stdout.flush().unwrap();

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
