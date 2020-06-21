extern crate crossterm;

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdin, stdout, Read, Write};
use std::sync::mpsc::channel;

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
    let mut game = game::Game {
        width: 30,
        height: 20,
        tiles: vec![],
    };

    game.tiles = game.init_map();

    // Game loop
    let mut playing = true;
    while playing {
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
            .write("move with wasd, press q to exit".as_bytes()).unwrap();

        game.render_map();

        stdout.flush().unwrap();
    }

    // Restore terminal after game is finished
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited");
}
