use amethyst::prelude::*;
use amethyst::core::Transform;
use crate::entities::camera::init_camera;

pub struct InitState;

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);
    }
}