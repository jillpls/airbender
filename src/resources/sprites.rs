use std::collections::HashMap;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::renderer::sprite::SpriteSheetHandle;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum SpriteSheetType {
    Background(String),
    Character(String),
    Custom(String),
}

#[derive(Default)]
pub struct SpriteSheetMap {
    sheets: HashMap<SpriteSheetType, SpriteSheetHandle>,
}

impl SpriteSheetMap {
    pub fn insert(
        &mut self,
        key: SpriteSheetType,
        value: SpriteSheetHandle,
    ) -> amethyst::Result<()> {
        if self.sheets.contains_key(&key) {
            return Err(amethyst::Error::from_string("key already in map"));
        }
        self.sheets.insert(key, value);
        Ok(())
    }

    pub fn replace(&mut self, key: SpriteSheetType, value: SpriteSheetHandle) {
        self.sheets.insert(key, value);
    }

    pub fn get(&self, key: &SpriteSheetType) -> Option<&SpriteSheetHandle> {
        self.sheets.get(key)
    }
}

pub fn load_sprite_sheet(world: &mut World, key: SpriteSheetType, path: &str) -> amethyst::Result<SpriteSheetHandle> {
    let fetched = world.try_fetch_mut::<SpriteSheetMap>();
    if let Some(mut map) = fetched {
        let img_path = path.to_owned() + ".png";
        let ron_path = path.to_owned() + ".ron";

        println!("Image path: {:?}", img_path);
        println!("Ron path: {:?}", ron_path);

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                &img_path,
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        let handle = loader.load(
            &ron_path, // Here we load the associated ron file
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        );


        Ok(handle)
    } else {
        Err(amethyst::Error::from_string("SpriteSheetMap SpriteSheetMap resource not in world"))
    }
}
