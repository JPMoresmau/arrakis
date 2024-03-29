//! Entry point
use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
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
mod audio;
mod build;
mod components;
mod config;
mod states;
mod systems;
use crate::config::ArrakisConfig;
use crate::states::InterTitle;

/// Game entry point
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let assets_dir = app_root.join("assets");
    let binding_path = config_dir.join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let arr_config = ArrakisConfig::load(config_dir.join("config.ron"))?;
    let render =
        RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]);
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(render)
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with(systems::StatusSystem, "status_system", &[])
        .with(systems::MoveSystem::new(), "move_system", &["input_system"])
        .with(
            systems::ActionSystem::new(),
            "action_system",
            &["input_system"],
        );

    let mut game = Application::build(assets_dir, InterTitle::start())?
        .with_resource(arr_config)
        .build(game_data)?;
    game.run();

    Ok(())
}
