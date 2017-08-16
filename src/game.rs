extern crate piston_window;
use piston_window::PistonWindow;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

extern crate input;
use input::{Input, RenderArgs, UpdateArgs, Button};

use dungeon::Dungeon;
use player::Player;

pub struct Game {
    dungeon: Dungeon,
    player: Player,
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

impl Game {
    pub fn new(dungeon: Dungeon) -> Self {
        let (px, py) = dungeon.player_spawn();
        Game {
            dungeon: dungeon,
            player: Player::new(px, py),
            forward: false,
            back: false,
            left: false,
            right: false,
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
    }

    pub fn update(&mut self, args: UpdateArgs, mdx: f64, mdy: f64) {
        let dt = args.dt;

        if self.forward {
            self.player.y_pos += 1.0 * dt;
        }
        if self.back {
            self.player.y_pos -= 1.0 * dt;
        }
        if self.left {
            self.player.x_pos -= 1.0 * dt;
        }
        if self.right {
            self.player.x_pos += 1.0 * dt;
        }

        let mut new_angle = self.player.angle + (mdx * 30.0 * dt);
        if new_angle >= 360.0 {
            new_angle -= 360.0
        }
        if new_angle <= 0.0 {
            new_angle += 360.0
        }
        self.player.angle = new_angle;

        println!("({},{})", self.player.x_pos, self.player.y_pos);
        println!("{}", self.player.angle);
        println!();
    }
}
