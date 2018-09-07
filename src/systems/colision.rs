use std::ops::Deref;

use amethyst::ecs::prelude::{Join, Read, ReadStorage, ReadExpect, System, WriteStorage, Entities};
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::transform::components::Transform;
use amethyst::core::timing::Time;

use super::super::brickbreak::{Paddle, Ball, Brick};
use super::super::audio::{play_brick_broken, Sounds};

use super::super::constants::{
    ARENA_WIDTH, ARENA_HEIGHT,
    PADDLE_POS_Y, PADDLE_WIDTH, PADDLE_HEIGHT,
    BALL_RADIUS, BRICK_WIDTH, BRICK_HEIGHT,
    };


// A point is in a box when its coordinates are smaller or equal than the top
// right, but larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}


pub struct CollisionSystem;


impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Brick>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (entities, mut balls, paddles, bricks, transforms, time, storage, sounds, audio_output): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            // game is started
            if ball.velocity != [0.0, 0.0] {

                // collission with the arena
                let ball_x = transform.translation[0] + ball.velocity[0] * time.delta_seconds();
                if (ball_x + BALL_RADIUS > ARENA_WIDTH) || (ball_x - BALL_RADIUS < 0.0){
                    ball.velocity[0] *= -1.0;
                }
                let ball_y = transform.translation[1] + ball.velocity[1] * time.delta_seconds();
                if ball_y + BALL_RADIUS > ARENA_HEIGHT {
                    ball.velocity[1] *= -1.0;
                }

                if ball_y < 0.0 {
                    ball.velocity = [0.0, 0.0];
                }

                // collision with the paddle
                let half_pad = PADDLE_WIDTH * 0.5;
                if (ball_y - BALL_RADIUS < PADDLE_POS_Y) && (ball_y + BALL_RADIUS > (PADDLE_POS_Y - PADDLE_HEIGHT)) {
                    for (_, paddle_transform) in (&paddles, &transforms).join() {
                        let paddle_x = paddle_transform.translation[0];
                        if (ball_x + BALL_RADIUS > paddle_x - half_pad) &&
                           (ball_x - BALL_RADIUS < paddle_x + half_pad) {
                            ball.velocity[1] *= -1.0;
                            ball.velocity[0] = ((ball_x - paddle_x) / half_pad) * ball.velocity[1];
                        }
                    }
                }

                for (entity, _, brick_transform) in (&*entities, &bricks, &transforms).join() {
                    let brick_x = brick_transform.translation[0] - BRICK_WIDTH * 0.5;
                    let brick_y = brick_transform.translation[1] - BRICK_HEIGHT * 0.5;

                    if point_in_rect(
                        ball_x,
                        ball_y,
                        brick_x - BALL_RADIUS,
                        brick_y - BALL_RADIUS,
                        brick_x + PADDLE_WIDTH + BALL_RADIUS,
                        brick_y + PADDLE_HEIGHT + BALL_RADIUS,
                    ) {
                        ball.velocity[1] *= -1.0;
                        entities.delete(entity).unwrap();
                        play_brick_broken(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                        break;
                    }
                }
           }
        }
    }
}
