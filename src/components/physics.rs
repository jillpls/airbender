/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use ::amethyst::ecs::{Component, DenseVecStorage};

pub struct Physics {
    gravity: f32,
    deacceleration: Option<f32>,
}

impl Default for Physics {
    fn default() -> Self {
        Physics {
            gravity: 1.0,
            deacceleration: None,
        }
    }
}

impl Component for Physics {
    type Storage = DenseVecStorage<Self>;
}
