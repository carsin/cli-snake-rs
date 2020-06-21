extern crate crossterm;

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdin, stdout, Read, Write};
use std::sync::mpsc::channel;

fn main() {
    // Set up terminal
    stdout().execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().execute(cursor::Hide).unwrap();

    // Set up input
    let (ctrls_sender, ctrls_receiver) = channel::<char>();
    std::thread::spawn(move || {
        loop {
            let mut buf = [0u8; 1]; // create a buffer for a single byte
            stdin().read_exact(&mut buf).unwrap(); // read byte into the buffer
            ctrls_sender.send(buf[0] as char).unwrap(); // send char on channel
        }
    });

    // Game loop
    let mut playing = true;
    while playing {
        stdout().queue(terminal::Clear(terminal::ClearType::All)).unwrap();

        while let Ok(ctrl) = ctrls_receiver.try_recv() {
            match ctrl {
                'q' => playing = false,
                _ => (),
            }
        }

        // Render
        stdout()
            .queue(cursor::MoveTo(0, 0)).unwrap()
            .write("move with wasd, press q to exit".as_bytes()).unwrap();
        stdout().flush().unwrap();
    }

    // Restore terminal after game is finished
    stdout().execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited");
}
