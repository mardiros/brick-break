use amethyst::core::transform::components::Transform;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use super::super::brickbreak::{Ball, ARENA_WIDTH, PADDLE_WIDTH};

pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, ball, input): Self::SystemData) {
        for (ball, mut transform) in (&ball, &mut transforms).join() {
            let movement = input.axis_value("paddle");
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    if ball.velocity == [0.0, 0.0] {
                        let scaled_amount = 1.5 * mv_amount as f32;
                        let new_val = (transform.translation[0] + scaled_amount)
                            .min(ARENA_WIDTH - PADDLE_WIDTH * 0.5)
                            .max(PADDLE_WIDTH * 0.5);
                        transform.translation[0] = new_val;
                    }
                }
            }
        }
    }
}