use crate::entities::camera::init_camera;
use crate::resources::sprites::*;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
use amethyst::renderer::SpriteRender;
use amethyst::core::math::Vector3;
use std::borrow::Borrow;

#[derive(Default)]
pub struct InitState;

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert(SpriteSheetMap::default());
        init_camera(world);

        // sprite_test, TODO: Delete later

        let handle = load_sprite_sheet(world,
                          SpriteSheetType::Custom(String::from("test")),
                          "sprites/example/character_run").unwrap();

        let sprite_render = SpriteRender::new(handle, 1);

        println!("{:?}", sprite_render);

        let mut transform = Transform::default();
        transform.set_translation_xyz(400.0, 400.0, 0.0);
        transform.set_scale(Vector3::new(4.0, 4.0, 1.0));

        world.create_entity()
            .with(sprite_render)
            .with(transform)
            .build();

        // /sprite_test
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
