/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};

use crate::components::collision::*;
use crate::components::movement::Motion;

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, CollisionRectangle>,
        WriteStorage<'a, Motion>,
    );

    fn run(&mut self, data: Self::SystemData) {}
}
