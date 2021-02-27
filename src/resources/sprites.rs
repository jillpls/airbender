/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */


#[macro_export]
macro_rules! import_sheet {
    ($path:expr,$resources:expr,$progress_counter:expr) => {
        {
            let loader = ($resources).get_mut::<DefaultLoader>().expect("oof1");

            let texture = loader.load(&($path.to_owned() + ".png")); 
            let sprites = loader.load(&($path.to_owned() + ".ron")); 

            let sheet : Handle<SpriteSheet> = loader.load_from_data(
                SpriteSheet {texture, sprites},
                &mut $progress_counter,
                &$resources.get().expect("oof2")
            );

            sheet
        }        
    };
}

// use std::collections::HashMap;

// use amethyst::assets::{AssetStorage, DefaultLoader, ProgressCounter};
// use amethyst::prelude::*;
// use amethyst::renderer::{ImageFormat, SpriteSheet, Texture};

// #[derive(Clone, Eq, Hash, PartialEq)]
// pub enum SpriteSheetType {
//     Background(String),
//     Character(String),
//     Custom(String),
//     Animation(u32)
// }

// #[derive(Default)]
// pub struct SpriteSheetMap {
//     // sheets: HashMap<SpriteSheetType, SpriteSheetHandle>,
// }

// impl SpriteSheetMap {
//     pub fn insert(
//         &mut self,
//         key: SpriteSheetType,
//         value: SpriteSheetHandle,
//     ) -> amethyst::Result<()> {
//         if self.sheets.contains_key(&key) {
//             return Err(amethyst::Error::from_string("key already in map"));
//         }
//         self.sheets.insert(key, value);
//         Ok(())
//     }

//     pub fn replace(&mut self, key: SpriteSheetType, value: SpriteSheetHandle) {
//         self.sheets.insert(key, value);
//     }

//     pub fn get(&self, key: &SpriteSheetType) -> Option<&SpriteSheetHandle> {
//         self.sheets.get(key)
//     }
// }

// pub fn load_sprite_sheet(
//     world: &mut World,
//     key: SpriteSheetType,
//     path: &str,
//     progress: Option<ProgressCounter>
// ) -> amethyst::Result<LoadingAsset<SpriteSheet>> {
//     let fetched = world.try_fetch_mut::<SpriteSheetMap>();
//     if let Some(mut map) = fetched {
//         let img_path = path.to_owned() + ".png";
//         let ron_path = path.to_owned() + ".ron";

//         println!("Image path: {:?}", img_path);
//         println!("Ron path: {:?}", ron_path);

//         let mut progress_counter = progress.unwrap_or(ProgressCounter::new());

//         let texture_handle = {
//             let loader = world.read_resource::<DefaultLoader>();
//             let texture_storage = world.read_resource::<AssetStorage<Texture>>();
//             loader.load(&img_path, ImageFormat::default(), &mut progress_counter, &texture_storage)
//         };

//         let loader = world.read_resource::<DefaultLoader>();
//         let mut sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
//         let handle = loader.load(
//             &ron_path, // Here we load the associated ron file
//             SpriteSheetFormat(texture_handle),
//             &mut progress_counter,
//             &mut sprite_sheet_storage,
//         );

//         map.insert(key, handle.clone())?;

//         Ok(LoadingAsset::<SpriteSheet>::new(progress_counter, handle))
//     } else {
//         Err(amethyst::Error::from_string(
//             "SpriteSheetMap SpriteSheetMap resource not in world",
//         ))
//     }
// }
