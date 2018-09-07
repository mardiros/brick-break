use amethyst::core::transform::components::Transform;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::input::InputHandler;
use super::super::brickbreak::Ball;
use super::super::constants::{PADDLE_MOVE_FACTOR, ARENA_WIDTH, PADDLE_WIDTH, ARENA_HEIGHT, PADDLE_HEIGHT};

pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, ball, input, time): Self::SystemData) {
        for (ball, mut transform) in (&ball, &mut transforms).join() {
            let movement = input.axis_value("paddle");
            if ball.velocity == [0.0, 0.0] {
                if let Some(mv_amount) = movement {
                    // the game is not started, the ball is glued to the paddle
                    let scaled_amount = PADDLE_MOVE_FACTOR * mv_amount as f32;
                    let new_val = (transform.translation[0] + scaled_amount)
                        .min(ARENA_WIDTH - PADDLE_WIDTH * 0.5)
                        .max(PADDLE_WIDTH * 0.5);
                    transform.translation[0] = new_val;
                    transform.translation[1] = ARENA_HEIGHT * 0.05 + PADDLE_HEIGHT;
                }
            }
            else {
                transform.translation[0] += ball.velocity[0] * time.delta_seconds();
                transform.translation[1] += ball.velocity[1] * time.delta_seconds();
            }

        }
    }
}
