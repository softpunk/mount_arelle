extern crate piston_window;
use piston_window::{PistonWindow, OpenGL, WindowSettings};

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

use std::env::args;

extern crate mount_arelle;
use mount_arelle::dungeon::Dungeon;
use mount_arelle::game::Game;

fn main() {
    let seed = args().nth(1).expect("No seed specified");
    let dungeon = Dungeon::new_from_seed(&seed);

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Mount Arelle", (640, 480))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut gl = GlGraphics::new(opengl);

    let mut game = Game::new(dungeon);
    game.run(&mut window, &mut gl);
}
