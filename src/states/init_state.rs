/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use crate::entities::camera::init_camera;
use crate::resources::sprites::*;
use amethyst::assets::ProgressCounter;
use amethyst::prelude::*;

#[derive(Default)]
pub struct InitState {
    pub progress_counter : Option<ProgressCounter>
}

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert(SpriteSheetMap::default());
        init_camera(world);

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

        
        self.progress_counter = crate::components::animation::load_animation(world, "sprites/example/character_run", self.progress_counter.take()).ok();
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(p) = &self.progress_counter {
            if p.is_complete() {

            }
            
        }
        Trans::None
    }
}
