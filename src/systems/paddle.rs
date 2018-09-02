use amethyst::core::transform::components::Transform;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use super::super::brickbreak::{Paddle, ARENA_WIDTH, PADDLE_WIDTH};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (_, mut transform) in (&paddles, &mut transforms).join() {
            let movement = input.axis_value("paddle");
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = 1.7 * mv_amount as f32;
                    let new_val = (transform.translation[0] + scaled_amount)
                        .min(ARENA_WIDTH - PADDLE_WIDTH * 0.5)
                        .max(PADDLE_WIDTH * 0.5);
                    transform.translation[0] = new_val;
                }
            }
        }
    }
}
