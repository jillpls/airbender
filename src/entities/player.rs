/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::core::Transform;
use amethyst::prelude::*;

use crate::components::collision::CollisionRectangle;

pub fn load_player(world: &mut World) -> amethyst::Result<()> {
    world
        .create_entity()
        .with(CollisionRectangle::new(4, 8, 0.0))
        .with(Transform::default())
        .build();
    Ok(())
}
