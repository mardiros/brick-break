use amethyst::ecs::prelude::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;
use super::super::brickbreak::{Ball, BALL_VELOCITY_X, BALL_VELOCITY_Y};

pub struct StartSystem;

impl<'s> System<'s> for StartSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut ball, input): Self::SystemData) {
        for mut ball in (&mut ball).join() {
            let start = input.action_is_down("start");
            if start.is_some() && start.unwrap() {
                ball.velocity = [BALL_VELOCITY_X, BALL_VELOCITY_Y];
            }
        }
    }
}
