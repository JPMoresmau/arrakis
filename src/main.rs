use amethyst::{
    core::transform::TransformBundle,
    ecs::prelude::{ReadExpect, Resources, SystemData},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod arrakis;
mod config;
mod systems;
use crate::arrakis::Arrakis;
use crate::config::ArrakisConfig;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let assets_dir = app_root.join("assets");
    let binding_path = config_dir.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let arr_config = ArrakisConfig::load(config_dir.join("config.ron"));

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::StatusSystem, "status_system",&[])
        .with(systems::PlayerSystem, "player_system",&["input_system"])
        ;

    let mut game = Application::build(assets_dir, Arrakis)?
        .with_resource(arr_config)
        .build(game_data)?;
    game.run();

    Ok(())
}
