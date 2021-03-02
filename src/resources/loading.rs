/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use amethyst::prelude::*;
use amethyst::assets::{DefaultLoader, Loader, ProgressCounter, Handle};
use amethyst::renderer::{SpriteSheet};
use crate::components::animation::AnimationId;
use ron::de;

pub fn load_animations(path: &str, resources : &mut Resources, progress_counter : ProgressCounter) -> ProgressCounter{
    let animations_map : HashMap<AnimationId, (String, String, String)> = de::from_reader::<BufReader<File>, HashMap<AnimationId, (String, String, String)>>(
        BufReader::new(File::open(path).expect("yikes"))
    ).expect("yikes2");

    let mut sheet_map : HashMap<String, Handle<SpriteSheet>> = HashMap::new();

    let mut progress_counter = progress_counter;

    for (png, ron, _) in animations_map.values() {
        let mut key = png.clone();
        key.push_str(&ron);
        if sheet_map.contains_key(&key) {
            continue;
        }

        let (pc, sheet) = load_sprite_sheet(&png, &ron, resources, progress_counter);
        progress_counter = pc;

        sheet_map.insert(key, sheet);
    }

    for (k, (png, ron, anim)) in animations_map.iter() {
        println!("{:?}, {}, {}, {}", k, png, ron, anim);
        // TODO: DO THINGS
        // ACTUALLY LOAD THE ANIMATION DATA
        // take Handle<sheet>, by using png+ron as a key for sheet_map
    }

    progress_counter
}

pub fn load_sprite_sheet(texture_path : &str, ron_path : &str, resources : &mut Resources, progress_counter : ProgressCounter) -> (ProgressCounter, Handle<SpriteSheet>) {
        let mut progress_counter = progress_counter;
        let loader = resources
            .get_mut::<DefaultLoader>()
            .expect("oof1");
        
        let texture = loader.load(texture_path);
        let sprites = loader.load(ron_path);
        
        let sheet : Handle<SpriteSheet> = loader.load_from_data(
            amethyst::renderer::SpriteSheet { texture, sprites },
            &mut progress_counter,
            &resources.get().expect("oof2"),
        );
        (progress_counter, sheet)
}