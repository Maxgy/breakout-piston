use graphics::{Context, Graphics};
use piston_window::ellipse;

pub struct Ball {
    x: f64,
    y: f64,
    r: f64,
    dx: f64,
    dy: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self {
            x,
            y,
            r,
            dx: 4.0,
            dy: 4.0,
        }
    }

    pub fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        ellipse(
            [1.0, 0.0, 1.0, 1.0],
            [self.x - self.r, self.y - self.r, self.r * 2.0, self.r * 2.0],
            context.transform,
            graphics,
        )
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn bounce(&mut self, window_width: f64, window_height: f64) {
        if self.x > window_width - self.r || self.x < self.r {
            self.dx = -self.dx;
        }
        if self.y > window_height - self.r || self.y < self.r {
            self.dy = -self.dy;
        }
    }
}
