use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};

use super::constants::*;
use super::audio::initialise_audio;

pub struct BrickBreak;

impl<'a, 'b> State<GameData<'a, 'b>> for BrickBreak {

    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let spritesheet = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "textures/pong_spritesheet.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage,
            )
        };
        world.register::<Paddle>();
        world.register::<Brick>();
        world.register::<Ball>();

        initialise_paddle(world, &spritesheet);
        initialise_bricks(world, &spritesheet);
        initialise_ball(world, &spritesheet);
        initialise_camera(world);
        initialise_audio(world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}


fn initialise_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}

pub struct Paddle {
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new() -> Paddle {
        Paddle {
            width: 1.0,
            height: 1.0,
        }   
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}


fn initialise_paddle(world: &mut World, spritesheet: &TextureHandle) {
    let mut left_transform = Transform::default();

    // Correctly position the paddles.
    let x = ARENA_WIDTH * 0.5;
    left_transform.translation = Vector3::new(x, PADDLE_POS_Y, 0.0);

    // Build the sprite for the paddles.
    let sprite = Sprite {
        left: 0.0,
        right: PADDLE_WIDTH,
        top: 0.0,
        bottom: PADDLE_HEIGHT,
    };

    // Create a bric entity.
    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on brick")
        .with(Paddle::new())
        .with(GlobalTransform::default())
        .with(left_transform)
        .build();
}

pub struct Brick {
}

impl Brick {
    fn new() -> Brick {
        Brick {}
    }
}

impl Component for Brick {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_bricks(world: &mut World, spritesheet: &TextureHandle) {

    // Correctly position the paddles.
    let mut x = ARENA_WIDTH * 0.05;
    let mut y = ARENA_HEIGHT * 0.95;


    // Build the sprite for the paddles.
    let sprite = Sprite {
        left: 0.0,
        right: BRICK_WIDTH,
        top: 8.0,
        bottom: 8.0 + BRICK_HEIGHT,
    };

    for _ in 0..8 {

        for _ in 0..30 {

            let mut left_transform = Transform::default();
            left_transform.translation = Vector3::new(x, y, 0.0);

            // Create a left plank entity.
            world
                .create_entity()
                .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
                .expect("Failed to add sprite render on paddle")
                .with(Brick::new())
                .with(GlobalTransform::default())
                .with(left_transform)
                .build();

            x += BRICK_WIDTH * 1.05;
        }

        y -= BRICK_HEIGHT * 4.0;
        x = ARENA_WIDTH * 0.05;

    }

}


pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            radius: BALL_RADIUS,
            velocity: [0.0, 0.0],
        }   
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}


/// Initialises one ball in the middle-ish of the arena.
fn initialise_ball(world: &mut World, spritesheet: &TextureHandle) {

    // Create the translation.
    let mut local_transform = Transform::default();
    let x = ARENA_WIDTH * 0.5;
    let y = ARENA_HEIGHT * 0.05 + PADDLE_HEIGHT;

    local_transform.translation = Vector3::new(x, y, 0.0);

    // Create the sprite for the ball.
    let sprite = Sprite {
        left: 12.0,
        right: 12.0 + BALL_RADIUS * 2.0,
        top: 4.0,
        bottom: 4.0 + BALL_RADIUS * 2.0,
    };

    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
        .expect("Error creating SpriteRender for ball")
        .with(Ball::new())
        .with(local_transform)
        .with(GlobalTransform::default())
        .build();
}