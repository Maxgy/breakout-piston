use graphics::{Context, Graphics};
use piston_window::ellipse;

use crate::brick::Brick;
use crate::paddle::Paddle;

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
            dy: -4.0,
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

    pub fn edge_bounce(&mut self, window_width: f64, window_height: f64) -> bool {
        if self.x > window_width - self.r || self.x < self.r {
            self.dx = -self.dx;
            false
        } else if self.y < self.r {
            self.dy = -self.dy;
            false
        } else {
            self.y > window_height - self.r
        }
    }

    pub fn hit_paddle(&mut self, paddle: &Paddle) {
        if self.y > paddle.y - self.r && (self.x > paddle.x && self.x < paddle.x + paddle.w) {
            self.y -= 1.0;
            if self.x < paddle.x + paddle.w / 2.0 {
                self.dx = -(self.dx.abs());
                self.dy = -self.dy;
            } else {
                self.dx = self.dx.abs();
                self.dy = -self.dy;
            }
        }
    }

    pub fn break_bricks(&mut self, bricks: &mut Vec<Brick>) {
        for (i, b) in bricks.iter().enumerate() {
            if self.x > b.x
                && self.x < b.x + b.w
                && (self.y < b.y + b.h + self.r && self.y > b.y + b.h / 2.0
                    || self.y > b.y - self.r && self.y < b.y + b.h / 2.0)
            {
                self.dy = -self.dy;
                bricks.remove(i);
                break;
            }
            if self.y > b.y
                && self.y < b.y + b.h
                && (self.x < b.x + b.w + self.r && self.x > b.x + b.w / 2.0
                    || self.x > b.x - self.r && self.x < b.x + b.w / 2.0)
            {
                self.dx = -self.dx;
                bricks.remove(i);
                break;
            }
        }
    }
}
