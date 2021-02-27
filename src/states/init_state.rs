/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use crate::components::animation::AnimationId;
use crate::entities::{
    PlayerState,
    camera::init_camera
};

use amethyst::assets::Handle;
use amethyst::assets::ProgressCounter;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};

#[derive(Default)]
pub struct InitState {
    pub progress_counter: Option<ProgressCounter>,
}

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        self.progress_counter = Some(ProgressCounter::new());
        let mut progress_counter = self.progress_counter.take().unwrap_or_default();
        let resources = data.resources;

        let sheet = import_sheet!("sprites/example/character_run", resources, progress_counter);

        let anim = load_animation!(resources, progress_counter, 4);

        let mut animation_set = amethyst::animation::AnimationSet::<
            AnimationId,
            amethyst::renderer::SpriteRender,
        >::new();
        animation_set.insert(AnimationId::Idle, anim);

        self.progress_counter = Some(progress_counter);

        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(50.0, 50.0, 0.0);
            let world = data.world;
            world.push((animation_set, SpriteRender::new(sheet, 0), transform, PlayerState::default()));
            init_camera(world, resources);
        }
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }
}
