use crate::resources::display::DisplayDimensions;
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::Camera;

pub fn init_camera(world: &mut World) {
    let dimensions = {
        if let Some(fetched) = world.try_fetch::<DisplayDimensions>() {
            *fetched
        } else {
            DisplayDimensions::default()
        }
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width * 0.5, dimensions.height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width, dimensions.height))
        .with(transform)
        .build();
}
