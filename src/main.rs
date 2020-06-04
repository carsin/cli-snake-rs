extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    // Initialize IO
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    // Clear terminal
    write!(stdout, "{}{}{}Press WASD or q to quit", termion::cursor::Goto(1,1), termion::clear::All, termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    // Key Handler
    for c in stdin.keys() {
        // Clear last input
         write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 2),
               termion::clear::CurrentLine)
                .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('w') | Key::Up => {
                write!(stdout, "^").unwrap();
            },
            Key::Char('a') | Key::Left => {
                write!(stdout, "<").unwrap();
            },
            Key::Char('s') | Key::Down => {
                write!(stdout, "v").unwrap();
            },
            Key::Char('d') | Key::Right => {
                write!(stdout, ">").unwrap();
            },
            Key::Char(c) => {
                write!(stdout, "Unknown input: {}", c).unwrap();
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
