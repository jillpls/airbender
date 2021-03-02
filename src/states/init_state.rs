/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use crate::components::animation::{AnimationId, AnimationData};
use crate::components::collision::CollisionBox;
use crate::entities::{camera::init_camera};
use crate::resources::loading::load_animations;
use std::collections::HashMap;

use amethyst::assets::Handle;
use amethyst::assets::ProgressCounter;
use amethyst::core::Transform;
use amethyst::window::Window;
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};

#[derive(Default)]
pub struct InitState {
    pub progress_counter: Option<ProgressCounter>,
}

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        self.progress_counter = Some(ProgressCounter::new());
        let progress_counter = self.progress_counter.take().unwrap_or_default();
        let resources = data.resources;

        // let sheet = import_sheet!("sprites/example/character/character_run_debug", resources, progress_counter);

        let mut progress_counter = load_animations("assets/character/animations.ron", resources, progress_counter);


        let run_sheet = import_sheet!("character/character_run", resources, progress_counter);
        let dash_sheet = import_sheet!("character/character_dash", resources, progress_counter);
        
        let anim = load_animation!(resources, progress_counter, 4);

        let mut animation_set = amethyst::animation::AnimationSet::<
            AnimationId,
            amethyst::renderer::SpriteRender,
        >::new();
        animation_set.insert(AnimationId::Run, anim.clone());
        animation_set.insert(AnimationId::Dash, anim);

        let mut sheet_map : HashMap<AnimationId, Handle<SpriteSheet>> = HashMap::new();
        sheet_map.insert(AnimationId::Run, run_sheet.clone());
        sheet_map.insert(AnimationId::Dash, dash_sheet.clone());

        let anim_data = AnimationData {
            current_animation : AnimationId::Run,
            sheet_map,
            current_time : None
        };


        self.progress_counter = Some(progress_counter);

        let mut transform = Transform::default();
        transform.set_translation_xyz(50.0, 50.0, 0.0);
        let world = data.world;
        let collision_box = CollisionBox::new(50.0,50.0);
        let collision_box_debug = collision_box.generate_debug_lines(&transform);
        world.push((
            animation_set,
            SpriteRender::new(run_sheet, 0),
            // SpriteRender::new(run_sheet, 0),
            transform,
            anim_data,
            collision_box,
            collision_box_debug
        ));
        init_camera(world, resources);
        change_screen(resources, 1);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }
}

fn change_screen(resources: &mut Resources, _screen: usize) {
    // TODO: actually set the screen according to parameters.
    if let Some(window) = resources.get_mut::<Window>() {
        for s in window.available_monitors() {
            if s.name().unwrap_or_default().contains("DISPLAY1") {
                window.set_outer_position(s.position());
            }
        }
    }
}