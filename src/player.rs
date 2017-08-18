pub struct Player {
    pub x_pos: f64,
    pub y_pos: f64,
    pub angle: f64,
    pub fov: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x_pos: x,
            y_pos: y,
            angle: 0.0f64.to_radians(),
            fov: 75.0f64.to_radians(),
        }
    }

    pub fn rotate(&mut self, degrees: f64) {
        let delta = degrees % 360.0;
        let mut new_degrees = self.angle.to_degrees() + delta;

        if new_degrees < 0.0 {
            new_degrees += 360.0;
        } else if new_degrees >= 360.0 {
            new_degrees -= 360.0;
        }

        self.angle = new_degrees.to_radians();
    }
}
