extern crate piston_window;
use piston_window::PistonWindow;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

extern crate input;
use input::{Input, RenderArgs, UpdateArgs};

use dungeon::Dungeon;

pub struct Game {
    dungeon: Dungeon,
}

impl Game {
    pub fn new(dungeon: Dungeon) -> Self {
        Game { dungeon: dungeon }
    }

    pub fn run(&mut self, window: &mut PistonWindow, graphics: &mut GlGraphics) {
        while let Some(event) = window.next() {
            match event {
                Input::Press(_button) => {

                },
                Input::Release(_button) => {

                },
                Input::Update(args) => {
                    self.update(args);
                },
                Input::Render(args) => {
                    self.render(args);
                },
                _ => {},
            }
        }
    }

    fn render(&mut self, args: RenderArgs) {
    }

    fn update(&mut self, args: UpdateArgs) {
    }
}
