extern crate piston_window;
use piston_window::PistonWindow;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

extern crate graphics;
use graphics::{Graphics, clear};
use graphics::rectangle::Rectangle;
use graphics::math::identity;

extern crate input;
use input::{Input, RenderArgs, UpdateArgs, Button};

use dungeon::Dungeon;
use grid::Tile;
use player::Player;

pub struct Game {
    dungeon: Dungeon,
    player: Player,
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const DARK_RED: [f32; 4] = [0.5, 0.0, 0.0, 1.0];

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

    pub fn render(&mut self, args: RenderArgs, gl: &mut GlGraphics) {
        let w = args.draw_width;
        let h = args.draw_height;

        let red = Rectangle::new(RED);
        let dark_red = Rectangle::new(DARK_RED);

        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
        });

        for x in 0..w {
             let cam_x = 2.0 * x as f64 / w as f64 - 1.0;
             let ray_pos_x = self.player.x_pos;
             let ray_pos_y = self.player.y_pos;
             let ray_dir_x = self.player.dir_x + self.player.plane_x * cam_x;
             let ray_dir_y = self.player.dir_y + self.player.plane_y * cam_x;

             let mut map_x = ray_pos_x as i64;
             let mut map_y = ray_pos_y as i64;

             let mut side_dist_x: f64;
             let mut side_dist_y: f64;

             let delta_dist_x = (1.0 + ray_dir_y.powi(2) / ray_dir_x.powi(2));
             let delta_dist_y = (1.0 + ray_dir_x.powi(2) / ray_dir_y.powi(2));

             let step_x: i64;
             let step_y: i64;

             let mut side;

             if ray_dir_x < 0.0 {
                 step_x = -1;
                 side_dist_x = (ray_pos_x - map_x as f64) * delta_dist_x;
             } else {
                 step_x = 1;
                 side_dist_x = (map_x as f64 + 1.0 - ray_pos_x) * delta_dist_x;
             }

             if ray_dir_y < 0.0 {
                 step_y = -1;
                 side_dist_y = (ray_pos_y - map_y as f64) * delta_dist_y;
             } else {
                 step_y = 1;
                 side_dist_y = (map_y as f64 + 1.0 - ray_pos_y) * delta_dist_y;
             }

             loop {
                 if side_dist_x < side_dist_y {
                     side_dist_x += delta_dist_x;
                     map_x += step_x;
                     side = 0;
                 } else {
                     side_dist_y += delta_dist_y;
                     map_y += step_y;
                     side = 1;
                 }

                 match self.dungeon.grid.get(map_x as u32, map_y as u32) {
                     Some(&Tile::Wall) => {
                         break;
                     }
                     None => {
                         break;
                     },
                     _ => {},
                 }
             }

             let mut perp_wall_dist: f64;

             if side == 0 {
                 perp_wall_dist = (map_x as f64 - ray_pos_x + (1.0 - step_x as f64) / 2.0) / ray_dir_x;
             } else {
                 perp_wall_dist = (map_y as f64 - ray_pos_y + (1.0 - step_y as f64) / 2.0) / ray_dir_y;
             }

             println!("Side is {}", side);
             println!("Map: ({},{})", map_x, map_y);
             println!("Ray: ({},{})", ray_pos_x, ray_pos_y);
             println!("Step: ({},{})", step_x, step_y);
             println!("PWD: {}", perp_wall_dist);
             println!();

             let line_height = h as i32 / perp_wall_dist as i32;

             let mut draw_start = -line_height / 2 + h as i32 / 2;
             if draw_start < 0 { draw_start = 0; }
             let mut draw_end = line_height / 2 + h as i32 / 2;
             if draw_start >= h as i32 { draw_end = h as i32 - 1; }

             gl.draw(args.viewport(), |c, gl| {
                 let rect = [x as f64, draw_start as f64, 1.0, line_height as f64];

                 if side == 1 {
                     &red.draw(rect, &c.draw_state, c.transform, gl);
                 } else {
                     &dark_red.draw(rect, &c.draw_state, c.transform, gl);
                 }
             });
        }
    }

    pub fn update(&mut self, args: UpdateArgs, mdx: f64, mdy: f64) {
        let dt = args.dt;
        let mut new_x = self.player.x_pos;
        let mut new_y = self.player.y_pos;

        if self.forward {
            new_y = self.player.y_pos + (5.0 * dt);
        }
        if self.back {
            new_y = self.player.y_pos - (5.0 * dt);
        }
        if self.left {
            new_x = self.player.y_pos - (5.0 * dt);
        }
        if self.right {
            new_x = self.player.y_pos + (5.0 * dt);
        }

        if new_x < 0.0 { new_x = 0.0; }
        if new_x > self.dungeon.grid.width() as f64 { new_x = self.dungeon.grid.width() as f64; }

        if new_y < 0.0 { new_y = 0.0; }
        if new_y > self.dungeon.grid.height() as f64 { new_y = self.dungeon.grid.height() as f64; }

        self.player.x_pos = new_x;
        self.player.y_pos = new_y;

        // let mut new_angle = self.player.angle + (mdx * 30.0 * dt);
        // if new_angle >= 360.0 {
        //     new_angle -= 360.0
        // }
        // if new_angle <= 0.0 {
        //     new_angle += 360.0
        // }
        // self.player.angle = new_angle;
    }
}
