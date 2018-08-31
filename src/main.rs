extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::renderer::{DrawFlat, PosTex};

mod brickbreak;
mod systems;

use brickbreak::BrickBreak;


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
        .with_basic_renderer(path, DrawFlat::<PosTex>::new(), false)?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);

    let mut game = Application::build(assets_dir, BrickBreak)?
        .build(game_data)?;
    game.run();
    Ok(())
}
