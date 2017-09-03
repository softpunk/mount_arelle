use ggez::{Context, timer};
use ggez::graphics::{self, Point, Rect, Color, DrawMode, GraphicsContext};
use ggez::event::{EventHandler, Keycode, Mod, MouseState};
use ggez::error::GameResult;

use picto::pixel::Read;
use picto::buffer::Rgba as RgbaImage;
use picto::color::Rgba;
use picto::write;

use std::f64;
use std::time::Duration;

use dungeon::Dungeon;
use grid::Tile;
use player::Player;

const TAU: f64 = 2.0 * f64::consts::PI;
const FPS: u32 = 24;

lazy_static! {
    static ref BLACK: Rgba = Rgba::new(0.0, 0.0, 0.0, 1.0);
    static ref WHITE: Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0);
    static ref RED: Rgba = Rgba::new(0.2, 0.0, 0.0, 1.0);
    static ref LIGHT_GRAY: Rgba = Rgba::new(0.7, 0.7, 0.7, 1.0);
    static ref DARK_GRAY: Rgba = Rgba::new(0.4, 0.4, 0.4, 1.0);
}

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

    pub fn software_render(&self, mut ctx: &mut Context) -> GameResult<()> {
        let (screen_w, screen_h) = ctx.gfx_context.get_drawable_size();

        let mut buffer = RgbaImage::from_pixel(screen_w, screen_h, &RED);

        let proj_dist =
            (screen_w as f64 / 2.0) / (self.player.fov / 2.0).tan();

        for (x, raycast) in self.cast_rays(screen_w).iter().enumerate() {
            let dist = raycast.distance;

            let line_height = (proj_dist / dist) as f32;
            let mut line_bottom = (screen_h as f32 / 2.0_f32) - (line_height / 2.0_f32);
            let mut line_top = line_bottom + line_height;

            if line_bottom < 0. { line_bottom = 0. };
            if line_top > screen_h as f32 {
                line_top = screen_h as f32;
            }

            let color = if raycast.cell_edge {
                *LIGHT_GRAY
            } else {
                *DARK_GRAY
            };

            for y in line_bottom as u32..line_top as u32 {
                buffer.set(x as u32, y as u32, &color);
            }
        }

        graphics::clear(ctx);
        let mut image = graphics::Image::from_rgba8(
            ctx,
            buffer.width() as u16,
            buffer.height() as u16,
            &buffer,
        )?;

        graphics::draw(
            ctx,
            &image,
            [screen_w as f32 / 2.0, screen_h as f32 / 2.0].into(),
            0.0,
        )?;

        graphics::circle(
            ctx,
            DrawMode::Line,
            [screen_w as f32 / 2.0, screen_h as f32 / 2.0].into(),
            3.0,
            0.0001,
        )?;

        graphics::present(&mut ctx);

        Ok(())
    }

    fn cast_rays(&self, screen_w: u32) -> Vec<Raycast> {
        let mut rays = Vec::new();

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
                            Some(&Tile::Wall(_)) | None => {
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
                        Some(&Tile::Wall(_)) | None => {
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

                let actual_distance = int_dist.sqrt() * (self.player.angle - ray_angle).cos();
                rays.push(
                    Raycast {
                        distance: actual_distance,
                        cell_edge: cell_edge,
                    }
                );
            }

        }
        rays
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let dt = timer::duration_to_f64(dt);
        let speed = 3.3;

        let cur_x = self.player.x_pos;
        let cur_y = self.player.y_pos;

        let angle_x = self.player.angle.cos();
        let angle_y = self.player.angle.sin();

        let mut x_dist = 0.0;
        let mut y_dist = 0.0;

        if self.forward {
            x_dist += angle_x;
            y_dist += angle_y;
        } else if self.back {
            x_dist -= angle_x;
            y_dist -= angle_y;
        }

        if self.left {
            x_dist += angle_y;
            y_dist -= angle_x;
        } else if self.right {
            x_dist -= angle_y;
            y_dist += angle_x;
        }

        let length = (x_dist.powi(2) + y_dist.powi(2)).sqrt();
        let mut nx: f64 = x_dist / length;
        let mut ny: f64 = y_dist / length;
        if nx.is_nan() {
            nx = 0.0;
        }

        if ny.is_nan() {
            ny = 0.0;
        }

        let mut dx = nx * speed * dt;
        let mut dy = ny * speed * dt;

        match self.dungeon.grid.get((cur_x + dx).floor() as u32, cur_y.floor() as u32) {
            Some(&Tile::Wall(_)) | None => {
                dx = 0.0;
            },
            _ => {},
        }

        match self.dungeon.grid.get(cur_x.floor() as u32, (cur_y + dy).floor() as u32) {
            Some(&Tile::Wall(_)) | None => {
                dy = 0.0;
            },
            _ => {},
        }

        self.player.x_pos += dx;
        self.player.y_pos += dy;

        Ok(())
    }

    fn draw(&mut self, mut ctx: &mut Context) -> GameResult<()> {
        let return_val = self.software_render(&mut ctx);
        timer::sleep_until_next_frame(&ctx, FPS);
        return_val
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

    // fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
    //     println!("{}x{}", width, height);
    //     let _ = graphics::set_screen_coordinates(ctx, 0.0, width as f32, 0.0, height as f32);
    // }
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

struct Raycast {
    distance: f64,
    cell_edge: bool,
}
