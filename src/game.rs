extern crate ggez;
use ggez::{Context, timer};
use ggez::graphics::{self, Rect, Color, DrawMode};
use ggez::event::{EventHandler, Keycode, Mod, MouseState};
use ggez::error::GameResult;

// extern crate sdl2;

use std::f64;
use std::time::Duration;

use dungeon::Dungeon;
use grid::Tile;
use player::Player;

pub const TAU: f64 = 2.0 * f64::consts::PI;

pub struct Game {
    dungeon: Dungeon,
    player: Player,
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
const LIGHT_GRAY: Color = Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 };
const DARK_GRAY: Color = Color { r: 0.4, g: 0.4, b: 0.4, a: 1.0 };

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
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let dt = timer::duration_to_f64(dt);

        let mut new_x = self.player.x_pos;
        let mut new_y = self.player.y_pos;

        if self.forward {
            new_y = self.player.y_pos + (3.0 * dt);
        }
        if self.back {
            new_y = self.player.y_pos - (3.0 * dt);
        }
        if self.left {
            new_x = self.player.x_pos + (3.0 * dt);
        }
        if self.right {
            new_x = self.player.x_pos - (3.0 * dt);
        }

        if new_x < 0.0 {
            new_x = 0.0;
        }
        if new_x > self.dungeon.grid.width() as f64 {
            new_x = self.dungeon.grid.width() as f64;
        }

        if new_y < 0.0 {
            new_y = 0.0;
        }
        if new_y > self.dungeon.grid.height() as f64 {
            new_y = self.dungeon.grid.height() as f64;
        }

        self.player.x_pos = new_x;
        self.player.y_pos = new_y;

        // self.player.rotate(mdx * dt);

        Ok(())
    }

    fn draw(&mut self, mut ctx: &mut Context) -> GameResult<()> {
        let screen_w: u32;
        let screen_h: u32;

        {
            let window = ctx.gfx_context.get_window();
            let (w, h) = window.drawable_size();
            screen_w = w;
            screen_h = h;
        }

        graphics::clear(ctx);
        let buffer = vec![0; (screen_h * screen_w) as usize];

        let mut image = graphics::Image::from_rgba8(
            ctx,
            screen_w as u16,
            screen_h as u16,
            &buffer,
        );

        for x in 0..screen_w {
            let ray_screen_x = x as f64 - screen_w as f64 / 2.0;

            let proj_dist =
                (screen_w as f64 / 2.0) / (self.player.fov / 2.0).tan();
            let ray_view_dist =
                (ray_screen_x.powi(2) + proj_dist.powi(2)).sqrt();
            let ray_angle: f64 =
                (ray_screen_x / ray_view_dist).asin()+self.player.angle;

            let mut int_dist: f64 = 0.0;
            let mut int_x: f64 = 0.0;
            let mut int_y: f64 = 0.0;
            let mut cell_x: u32 = 0;
            let mut cell_y: u32 = 0;
            let mut cell_edge = false; // false for y, true for x

            let cell_size: f64 = 1.0;
            let angle = wrap_angle(ray_angle);
            let is_ray_right: bool =
                angle > (TAU * 0.75) || angle < (TAU * 0.25);
            let is_ray_up: bool =
                angle < 0.0 || angle > f64::consts::PI;

            {
                let mut slope = angle.sin() / angle.cos();
                let mut dx =
                    if is_ray_right { cell_size } else { -cell_size};
                let mut dy = dx * slope;

                let mut ray_position_x =
                    if is_ray_right {
                        f64::ceil(self.player.x_pos)
                    } else {
                        f64::floor(self.player.x_pos)
                    };
                let mut ray_position_y =
                    self.player.y_pos
                    + (ray_position_x - self.player.x_pos) * slope;

                while ray_position_x >= 0.0
                    && ray_position_x <= self.dungeon.grid.width() as f64
                    && ray_position_y >= 0.0
                    && ray_position_y <= self.dungeon.grid.height()
                    as f64 {
                        let tile_map_x =
                            f64::floor(ray_position_x + (if is_ray_right { 0.0 } else { -cell_size })) as u32;
                        let tile_map_y =
                            f64::floor(ray_position_y) as u32;

                        match self.dungeon.grid.get(tile_map_x, tile_map_y) {
                            Some(&Tile::Wall) | None => {
                                let dist_x = ray_position_x - self.player.x_pos;
                                let dist_y = ray_position_y - self.player.y_pos;
                                int_dist = dist_x.powi(2) + dist_y.powi(2);

                                cell_edge = false;

                                cell_x = tile_map_x;
                                cell_y = tile_map_y;

                                int_x = ray_position_x;
                                int_y = ray_position_y;

                                break;
                            },
                            _ => {},
                        }

                        ray_position_x += dx;
                        ray_position_y += dy;
                }
            }

            {
                let slope = angle.cos() / angle.sin();
                let delta_y = if is_ray_up { -cell_size } else { cell_size };
                let delta_x = delta_y * slope;

                let mut ray_position_y = if is_ray_up { f64::floor(self.player.y_pos) } else { f64::ceil(self.player.y_pos) };
                let mut ray_position_x = self.player.x_pos + (ray_position_y - self.player.y_pos) * slope;

                while (ray_position_x >= 0.0) && (ray_position_x < self.dungeon.grid.width() as f64) && (ray_position_y >= 0.0) && (ray_position_y < self.dungeon.grid.height() as f64) {
                    let tile_map_x: u32 = f64::floor(ray_position_x) as u32;
                    let tile_map_y: u32 = f64::floor(ray_position_y + (if is_ray_up { -cell_size } else { 0.0 })) as u32;

                    match self.dungeon.grid.get(tile_map_x, tile_map_y) {
                        Some(&Tile::Wall) | None => {
                            let distance_x: f64 = ray_position_x - self.player.x_pos;
                            let distance_y: f64 = ray_position_y - self.player.y_pos;
                            let x_intersection_distance = distance_x.powi(2) + distance_y.powi(2);
                            if (int_dist == 0.0) || (x_intersection_distance < int_dist) {
                                int_dist = x_intersection_distance;
                                cell_edge = true;
                                cell_x = tile_map_x;
                                cell_y = tile_map_y;
                                int_x = ray_position_x;
                                int_y = ray_position_y;
                            }

                            break;
                        },
                        _ => {},
                    }

                    ray_position_x += delta_x;
                    ray_position_y += delta_y;
                }
            }

            let actual_distance = int_dist.sqrt() * (self.player.angle - ray_angle).cos();


            let line_height: i32 = (proj_dist / actual_distance) as i32;
            let line_bottom: i32 = (screen_h as i32 / 2) - (line_height / 2);

            let rect = Rect {
                x: x as f32,
                y: (line_bottom + (line_height / 2)) as f32,
                w: 1.0,
                h: line_height as f32,
            };

            if cell_edge {
                graphics::set_color(ctx, LIGHT_GRAY);
            } else {
                graphics::set_color(ctx, DARK_GRAY);
            }
            graphics::rectangle(&mut ctx, DrawMode::Fill, rect);
        }
        graphics::present(&mut ctx);

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => {
                self.forward = true;
            },
            Keycode::A => {
                self.left = true;
            },
            Keycode::S => {
                self.back = true;
            },
            Keycode::D => {
                self.right = true;
            },
            _ => {},
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => {
                self.forward = false;
            },
            Keycode::A => {
                self.left = false;
            },
            Keycode::S => {
                self.back = false;
            },
            Keycode::D => {
                self.right = false;
            },
            _ => {},
        }
    }

    fn mouse_motion_event(
        &mut self,
        _state: MouseState,
        _x: i32,
        _y: i32,
        xrel: i32,
        _yrel: i32,
    ) {
        self.player.rotate(xrel as f64 * 0.5);
    }
}

fn wrap_angle(angle: f64) -> f64 {
    if angle < 0.0 {
        return angle + TAU;
    }
    else if angle >= TAU {
        return angle - TAU;
    }

    angle
}
