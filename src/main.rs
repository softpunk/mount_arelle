extern crate ggez;

extern crate piston_window;
use piston_window::{AdvancedWindow, PistonWindow, OpenGL, WindowSettings};

extern crate window;
// use window::AdvancedWindow;

extern crate input;
use input::{Input, RenderArgs, UpdateArgs, Button, Motion};
use input::keyboard::Key;

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

    let mut window: PistonWindow = WindowSettings::new("Mount Arelle", (800, 600))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    window.set_capture_cursor(true);

    let mut glgraphics = GlGraphics::new(opengl);
    let mut game = Game::new(dungeon);

    let mut mouse_dx = 0.0;
    let mut mouse_dy = 0.0;

    while let Some(event) = window.next() {
        match event {
            Input::Move(Motion::MouseRelative(x, y)) => {
                mouse_dx = x;
                mouse_dy = y;
            },
            Input::Render(args) => {
                game.render(args, &mut glgraphics);
            },
            _ => {},
        }
    }
}
