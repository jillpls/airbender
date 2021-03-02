/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use crate::components::collision::CollisionBox;
use crate::entities::{camera::init_camera};
use crate::resources::loading::{load_animations, AssetsDir};

use amethyst::assets::ProgressCounter;
use amethyst::core::Transform;
use amethyst::window::Window;
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender};

pub struct InitState {
    pub assets_dir : Option<std::path::PathBuf>,
    pub progress_counter: Option<ProgressCounter>,
}

impl InitState {
    pub fn new(assets_dir : std::path::PathBuf) -> Self {
        InitState {
            assets_dir : Some(assets_dir),
            progress_counter : None
        }
    }
}

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            world,
            resources,
            ..
        } = data;

        resources.insert::<AssetsDir>(AssetsDir(self.assets_dir.take().unwrap_or_default()));

        self.progress_counter = Some(ProgressCounter::new());
        let progress_counter = self.progress_counter.take().unwrap_or_default();

        // let sheet = import_sheet!("sprites/example/character/character_run_debug", resources, progress_counter);

        let (progress_counter, default_sheet, anim_data, anim_set) = load_animations("character/animations.ron", resources, progress_counter);
        self.progress_counter = Some(progress_counter);

        let mut transform = Transform::default();
        transform.set_translation_xyz(50.0, 50.0, 0.0);
        let collision_box = CollisionBox::new(50.0,50.0);
        let collision_box_debug = collision_box.generate_debug_lines(&transform);
        world.push((
            anim_set,
            SpriteRender::new(default_sheet, 0),
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