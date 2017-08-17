// extern crate cgmath;
// use cgmath::{Vector2, vec2};

pub struct Player {
    // pub pos: Vector2<f32>,
    pub x_pos: f64,
    pub y_pos: f64,
    pub dir_x: f64,
    pub dir_y: f64,
    pub plane_x: f64,
    pub plane_y: f64,
    // pub angle: f64,
    // pub fov: u32,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x_pos: x,
            y_pos: y,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
            // angle: 0.0,
            // fov: 75,
        }
    }
}
