/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::assets::{ProgressCounter, Handle};

pub struct LoadingAsset<T> {
    pub progress_counter : Option<ProgressCounter>,
    pub handle : Option<Handle<T>>
}

impl<T> LoadingAsset<T> {
    pub fn new(progress_counter : ProgressCounter, handle : Handle<T>) -> Self {
        Self {
            progress_counter : Some(progress_counter),
            handle : Some(handle)
        }
    }
}