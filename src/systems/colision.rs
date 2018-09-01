

use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::transform::components::Transform;
use amethyst::core::timing::Time;

use super::super::brickbreak::{
    Paddle, Ball,
    ARENA_WIDTH, ARENA_HEIGHT, PADDLE_POS_Y, PADDLE_WIDTH, PADDLE_HEIGHT};


pub struct CollisionSystem;


impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, paddles, transforms, time): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            // game is started
            if ball.velocity != [0.0, 0.0] {

                // collission with the arena
                let x = transform.translation[0] + ball.velocity[0] * time.delta_seconds();
                if (x > ARENA_WIDTH) || (x < 0.0){
                    ball.velocity[0] *= -1.0;
                }
                let y = transform.translation[1] + ball.velocity[1] * time.delta_seconds();
                if y > ARENA_HEIGHT {
                    ball.velocity[1] *= -1.0;
                }

                // collision with the paddle
                let half_pad = PADDLE_WIDTH * 0.5;
                if (y < PADDLE_POS_Y) && (y > (PADDLE_POS_Y - PADDLE_HEIGHT)) {
                    for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                        let paddle_x = paddle_transform.translation[0] - paddle.width * 0.5;
                        if (x > paddle_x - half_pad) &&
                           (x < paddle_x + half_pad) {
                            ball.velocity[1] *= -1.0;
                            ball.velocity[0] = ((x - paddle_x) / half_pad) * ball.velocity[1];
                        }
                    }
                }
           }
        }
    }
}
