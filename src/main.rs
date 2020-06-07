extern crate termion;

use std::io::{stdout, Write};
use termion::raw::IntoRawMode;


mod game;
// mod input;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    // Clear terminal
    write!(stdout, "{}{}{}", termion::cursor::Goto(1,1), termion::clear::All, termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    let mut game = game::Game {
        width: 30,
        height: 20,
        tiles: vec![],
    };

    game.tiles = game.init_map();
    // let input = input::spawn_input_channel();

    game.render_map(&mut stdout);

    // stdout().execute(LeaveAlternateScreen)?;
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
