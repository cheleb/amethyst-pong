extern crate amethyst;



use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline,
                         PosTex, RenderBundle, Stage};
use amethyst::core::transform::TransformBundle;

use pong::Pong;

mod pong;



fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = "./resources/display_config.ron";

    let config = DisplayConfig::load(&path);


    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );


    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new("./", Pong, game_data)?;
    game.run();


    Ok(())
}