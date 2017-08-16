// extern crate cgmath;
// use cgmath::{Vector2, vec2};

pub struct Player {
    // pub pos: Vector2<f32>,
    pub x_pos: f64,
    pub y_pos: f64,
    pub angle: f64,
    pub fov: u32,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x_pos: x,
            y_pos: y,
            angle: 0.0,
            fov: 75,
        }
    }
}
