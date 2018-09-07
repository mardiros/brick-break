extern crate amethyst;

use amethyst::audio::AudioBundle;
use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::renderer::{DrawFlat, PosTex};

mod audio;
mod constants;
mod brickbreak;
mod systems;

use brickbreak::BrickBreak;
use audio::Music;


fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let bindings_config = format!(
        "{}/resources/bindings_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let assets_dir = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(bindings_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))?
        .with_bundle(input_bundle)?
        .with_basic_renderer(path, DrawFlat::<PosTex>::new(), false)?
        .with(systems::BallSystem, "ball_system", &["input_system"])
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::StartSystem, "start_system", &["input_system"])
        .with(systems::CollisionSystem, "collision_system", &["ball_system", "paddle_system"]);

    let mut game = Application::build(assets_dir, BrickBreak)?
        .build(game_data)?;
    game.run();
    Ok(())
}
