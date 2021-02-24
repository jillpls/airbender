/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::ecs::{Component, DenseVecStorage, VecStorage};
use amethyst::core::math::Vector2;

pub struct CollisionRectangle {
    dimensions: Vector2<f32>,
    scale : f32
}

impl Default for CollisionRectangle {
    fn default() -> Self {
        CollisionRectangle {
            dimensions: Vector2::new(0.0, 0.0),
            scale : 1.0
        }
    }
}

impl Component for CollisionRectangle {
    type Storage = VecStorage<Self>;
}

impl CollisionRectangle {
    pub fn new(width: u32, height: u32, scale: f32) -> Self {
        CollisionRectangle { 
            dimensions: Vector2::new(width as f32, height as f32),
            scale 
        }
    }
}

#[derive(Default)]
pub struct Collider {
    collided: bool,
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
