/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::core::transform::TransformBundle;
use amethyst::assets::LoaderBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    SpriteRender,
    plugins::{RenderFlat2D, RenderToWindow, RenderDebugLines},
    rendy::hal::command::ClearColor,
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::animation::{AnimationBundle};
use amethyst::utils::application_root_dir;

use crate::states::*;
use crate::components::animation::AnimationId;
use crate::systems::animation::{AnimationController};

#[macro_use]
mod resources;
#[macro_use]
mod components;
mod entities;
mod states;
mod systems;
mod utils;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher
        .add_bundle(LoaderBundle)
        .add_bundle(AnimationBundle::<AnimationId, SpriteRender>::default())
        .add_bundle(TransformBundle::default())
        .flush()
        // .add_system(PlayerAnimation)
        // .add_system(AnimationSwitch)
        .add_system(AnimationController)
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear( ClearColor { float32 : [0.5, 0.5, 0.5, 1.0]
                        }),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default())
        );
        // .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new("animation_control", "sampler_interpolation"))?
        // .with(DefaultAnimation {}, "default_animation", &[]);
    let game = Application::new(assets_dir.clone(), InitState::new(assets_dir), dispatcher)?;
    game.run();
    Ok(())
}
