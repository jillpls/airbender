/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use crate::entities::camera::init_camera;
use crate::components::animation::AnimationId;
use crate::resources::sprites::*;
use amethyst::assets::ProgressCounter;
use amethyst::renderer::SpriteRender;
use amethyst::animation::{AnimationCommand, EndControl, AnimationSet, get_animation_set};
use amethyst::prelude::*;

#[derive(Default)]
pub struct InitState {
    pub progress_counter : Option<ProgressCounter>
}

impl SimpleState for InitState {
    fn on_start(&mut self, mut data: StateData<'_, GameData>) {

        // sprite_test, TODO: Delete later

        // let handle = load_sprite_sheet(
        //     world,
        //     SpriteSheetType::Custom(String::from("test")),
        //     "sprites/example/character_run",
        // )
        // .unwrap();



        // let sprite_render = SpriteRender::new(handle, 1);


        // let mut transform = Transform::default();
        // transform.set_translation_xyz(400.0, 400.0, 0.0);
        // transform.set_scale(Vector3::new(4.0, 4.0, 1.0));

        // world
        //     .create_entity()
        //     .with(sprite_render)
        //     .with(transform)
        //     .build();

        // /sprite_test

        self.progress_counter = Some(
            ProgressCounter::new()
        );

        
        self.progress_counter = Some(crate::components::animation::load_animation(
            "sprites/example/character_run",
            &mut data,
            self.progress_counter.take()));

        {
            let world = data.world;
            let resources = data.resources;
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
                    if let Some(control_set) = get_animation_set(&mut subworld, &mut buffer, *entity) {
                        println!("{:?}", control_set);
                        if control_set.is_empty() {
                            control_set.add_animation(
                                AnimationId::Idle,
                                &animation_set.get(&AnimationId::Idle).unwrap(),
                                EndControl::Loop(None),
                                1.0,
                                AnimationCommand::Start
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
