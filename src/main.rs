extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [640, 480]).exit_on_esc(true).build().unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut x: f64 = 0.0;
    // Game loop
    while let Some(e) = window.next() {
        // Rendering logic
        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("{:?}", key);
        }

        x += 1 as f64;

        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [x, x, 100.0, 100.0],
                c.transform,
                g,
            );
        });

    }
}
