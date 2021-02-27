/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use crate::components::animation::AnimationId;
use crate::entities::camera::init_camera;
use amethyst::animation::{get_animation_set, AnimationCommand, AnimationSet, EndControl};
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
            world.push((animation_set, SpriteRender::new(sheet,0), transform));
            init_camera(world, resources);
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
        let mut query = <(Entity, Read<AnimationSet<AnimationId, SpriteRender>>)>::query();
        let mut buffer = CommandBuffer::new(data.world);

        if let Some(ref progress_counter) = &self.progress_counter {
            if progress_counter.is_complete() {
                let (query_world, mut subworld) = data.world.split_for_query(&query);
                for (entity, animation_set) in query.iter(&query_world) {
                    if let Some(control_set) =
                        get_animation_set(&mut subworld, &mut buffer, *entity)
                    {
                        println!("{:?}", control_set);
                        if control_set.is_empty() {
                            control_set.add_animation(
                                AnimationId::Idle,
                                &animation_set.get(&AnimationId::Idle).unwrap(),
                                EndControl::Loop(None),
                                1.0,
                                AnimationCommand::Start,
                            );
                            self.progress_counter = None;
                            println!("{:?}", control_set);
                        }
                    }
                }
            }
        }

        buffer.flush(data.world);
        Trans::None
    }
}
