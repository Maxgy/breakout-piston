use graphics::{Context, Graphics};
use piston_window::rectangle;

pub struct Paddle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Paddle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    pub fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        rectangle(
            [1.0, 0.0, 1.0, 1.0],
            [self.x - self.w / 2.0, self.y, self.w, self.h],
            context.transform,
            graphics,
        )
    }

    pub fn update(&mut self, mouse_x: f64) {
        self.x = mouse_x;
    }
}
