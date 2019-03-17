extern crate piston_window;

use piston_window::*;

use breakout_piston::ball::Ball;
use breakout_piston::brick::Brick;
use breakout_piston::paddle::Paddle;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Breakout", [800, 600]).build().unwrap();
    let mut window_width = window.size().width;
    let mut window_height = window.size().height;
    let mut cursor = [0.0, 0.0];

    let mut ball = Ball::new(window_width / 2.0, window_height - 100.0, 11.0);
    let mut paddle = Paddle::new(window_width / 2.0, window_height - 50.0, 100.0, 20.0);
    let mut bricks = Brick::make_bricks(window_width);

    while let Some(event) = window.next() {
        window_width = window.size().width;
        window_height = window.size().height;

        event.mouse_cursor(|x, y| {
            cursor = [x, y];
        });

        window.draw_2d(&event, |context, graphics| {
            clear([0.5, 0.7, 0.7, 1.0], graphics);

            ball.draw(&context, graphics);
            ball.update();
            ball.edge_bounce(window_width, window_height);
            ball.hit_paddle(&paddle);
            ball.break_bricks(&mut bricks);

            paddle.draw(&context, graphics);
            paddle.update(cursor[0]);

            for b in bricks.iter() {
                b.draw(&context, graphics);
            }
        });
    }
}
