extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DrawFlat, PosTex};

mod brickbreak;

use brickbreak::BrickBreak;


fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let assets_dir = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(path, DrawFlat::<PosTex>::new(), false)?;
    let mut game = Application::build(assets_dir, BrickBreak)?
        .build(game_data)?;
    game.run();
    Ok(())
}
