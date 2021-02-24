/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

#[derive(Clone, Copy)]
pub struct DisplayDimensions {
    pub width: f32,
    pub height: f32,
}

impl Default for DisplayDimensions {
    fn default() -> Self {
        DisplayDimensions {
            width: 1920.0,
            height: 1080.0,
        }
    }
}
