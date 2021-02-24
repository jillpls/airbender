use amethyst::core::Transform;
use amethyst::prelude::*;

use crate::components::collision::CollisionRectangle;

pub fn load_player(world: &mut World) -> amethyst::Result<()> {
    world
        .create_entity()
        .with(CollisionRectangle::new(4, 8))
        .with(Transform::default())
        .build();
    Ok(())
}
