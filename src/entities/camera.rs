/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use crate::resources::display::DisplayDimensions;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::Camera;
use amethyst::window::ScreenDimensions;

pub fn init_camera(world: &mut World, resources: &mut Resources) {
    // let dimensions = {
    //     if let Some(fetched) = world.try_fetch::<DisplayDimensions>() {
    //         *fetched
    //     } else {
    //         DisplayDimensions::default()
    //     }
    // };



    // let dimensions = DisplayDimensions::default();

    // let mut transform = Transform::default();
    // transform.set_translation_xyz(dimensions.width * 0.5, dimensions.height * 0.5, 1.0);

    let mut camera_transform = Transform::default();
    let width = 500.0;
    let height = 500.0;
    camera_transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world.push(
        (
            Camera::standard_2d(width, height),
            camera_transform
        )
    );
}
