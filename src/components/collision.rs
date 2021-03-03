/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::core::math::{Point2, Vector2, Vector3};


use amethyst::core::Transform;
use amethyst::renderer::{
    debug_drawing::DebugLinesComponent,
    palette::Srgba
};

#[allow(dead_code)] // TODO: remove
pub struct ActiveCollider;

pub struct CollisionBox {
    offset : Vector2<f32>,
    span : Vector2<f32>,
}

pub struct SimpleCollisionBox {
    span: Vector3<f32>
}


impl SimpleCollisionBox {
    pub fn new(width : f32, height : f32) -> Self {
        SimpleCollisionBox {
            span: Vector3::new(width, height, 1.0)
        }
    }

    pub fn generate_debug_lines(&self, transform : &Transform) -> DebugLinesComponent {
        let mut debug_lines = DebugLinesComponent::new();
        debug_lines.add_rotated_rectangle(
            Point2::new(
                transform.translation()[0] - self.span[0] * 0.5,
                transform.translation()[1] - self.span[1] * 0.5 
            ), 
            Point2::new(
                transform.translation()[0] + self.span[0] * 0.5,
                transform.translation()[1] + self.span[1] * 0.5
            ),
            0.0,
            *transform.rotation(),
            Srgba::new(1.0,0.0,0.0,1.0)
            );

        debug_lines
    }
}