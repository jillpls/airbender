/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    SpriteRender,
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::animation::{AnimationBundle};
use amethyst::utils::application_root_dir;

use crate::states::*;
use crate::components::animation::AnimationId;
use crate::systems::animation::DefaultAnimation;

mod components;
mod entities;
mod resources;
mod states;
mod systems;
mod utils;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.5, 0.5, 0.5]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new("animation_control", "sampler_interpolation"))?
        .with_bundle(TransformBundle::new())?
        .with(DefaultAnimation {}, "default_animation", &[]);
    let mut game = Application::new(assets_dir, InitState::default(), game_data)?;
    game.run();
    Ok(())
}
