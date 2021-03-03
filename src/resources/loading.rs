/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use crate::components::animation::{AnimationData, AnimationId};
use amethyst::animation::{
    Animation, AnimationSet, InterpolationFunction, Sampler, SpriteRenderChannel,
    SpriteRenderPrimitive,
};
use amethyst::assets::{DefaultLoader, Handle, Loader, ProgressCounter};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use ron::de;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::string::ToString;

#[derive(Default)]
pub struct AssetsDir(pub PathBuf);

impl Display for AssetsDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_str().unwrap_or_default())
    }
}

pub fn load_animations(
    path: &str,
    resources: &mut Resources,
    progress_counter: ProgressCounter,
) -> (
    ProgressCounter,
    Handle<SpriteSheet>,
    AnimationData,
    AnimationSet<AnimationId, SpriteRender>,
) {
    let animations_map = de::from_reader::<
        BufReader<File>,
        HashMap<AnimationId, (String, String, String)>,
    >(BufReader::new(
        File::open(format!("{}/{}", assets_dir(resources), path)).expect("yikes"),
    ))
    .expect("yikes2");
    let mut progress_counter = progress_counter;

    let current_dir = {
        let mut split = path.rsplitn(2, '/');
        split.next();
        String::from(split.next().unwrap_or_default())
    };

    let mut sheet_file_map: HashMap<String, Handle<SpriteSheet>> = HashMap::new();
    let mut sheet_map: HashMap<AnimationId, Handle<SpriteSheet>> = HashMap::new();
    let mut animation_set = AnimationSet::<AnimationId, SpriteRender>::new();

    for (k, (png, ron, anim)) in animations_map.iter() {
        let key = format!("{}{}", png, ron);

        let png = format!("{}/{}", current_dir, png);
        let ron = format!("{}/{}", current_dir, ron);
        let anim = format!("{}/{}", current_dir, anim);

        let sheet = if !sheet_file_map.contains_key(&key) {
            let s = load_sprite_sheet(&png, &ron, resources, &mut progress_counter);
            sheet_file_map.insert(key.clone(), s.clone());
            Some(s)
        } else {
            None
        };

        // TODO: this probably works but isn't nice

        sheet_map.insert(
            *k,
            if let Some(s) = sheet {
                s
            } else {
                sheet_file_map.get(&key).unwrap().clone() // oof
            },
        );

        let (pc, animation) = load_animation(&anim, resources, progress_counter);
        progress_counter = pc;

        animation_set.insert(*k, animation);
    }

    if sheet_map.is_empty() {
        // TODO: Error Handling if sheet map is empty
        panic!();
    }

    if !sheet_map.contains_key(&AnimationId::default()) {
        // Ensure a default sheet is set and make a random sheet the default otherwise.
        let (k, h) = {
            let (k, h) = sheet_map
                .iter()
                .collect::<Vec<(&AnimationId, &Handle<SpriteSheet>)>>()[0];
            (k.clone(), h.clone())
        };
        let a = animation_set.get(&k).unwrap().clone();
        sheet_map.insert(AnimationId::default(), h);
        animation_set.insert(AnimationId::default(), a);
    }

    let anim_data = AnimationData {
        current_animation: Some(AnimationId::default()),
        sheet_map: sheet_map,
    };

    (
        progress_counter,
        anim_data
            .sheet_map
            .get(&AnimationId::default())
            .unwrap()
            .clone(),
        anim_data,
        animation_set,
    )
}

fn load_animation(
    path: &str,
    resources: &mut Resources,
    progress_counter: ProgressCounter,
) -> (ProgressCounter, Handle<Animation<SpriteRender>>) {
    let mut progress_counter = progress_counter;

    let loader = resources.get_mut::<DefaultLoader>().unwrap();

    let path = format!("{}/{}", assets_dir(resources), path);
    println!("{}", path);

    let (mut anim_data_map, _) = de::from_reader::<
        BufReader<File>,
        (HashMap<
            String,
            (
                Vec<f32>,
                Vec<usize>,
                InterpolationFunction<SpriteRenderPrimitive>,
            ),
        >, Option<()>),
    >(BufReader::new(File::open(path).expect("yikes")))
    .unwrap();

    let (input, output, function) = anim_data_map.remove("Sampler").unwrap();
    let output = convert_to_srp(output);
    let sampler: Sampler<SpriteRenderPrimitive> = Sampler {
        input,
        output,
        function,
    };
    let sampler: Handle<Sampler<SpriteRenderPrimitive>> =
        loader.load_from_data(sampler, &mut progress_counter, &resources.get().unwrap());

    let animation: Handle<Animation<SpriteRender>> = loader.load_from_data(
        Animation::<SpriteRender>::new_single(0, SpriteRenderChannel::SpriteIndex, sampler),
        &mut progress_counter,
        &resources.get().unwrap(),
    );

    (progress_counter, animation)
}

pub fn load_sprite_sheet(
    texture_path: &str,
    ron_path: &str,
    resources: &mut Resources,
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let loader = resources.get_mut::<DefaultLoader>().expect("oof1");

    let texture = loader.load(texture_path);
    let sprites = loader.load(ron_path);

    let sheet: Handle<SpriteSheet> = loader.load_from_data(
        amethyst::renderer::SpriteSheet { texture, sprites },
        progress_counter,
        &resources.get().expect("oof2"),
    );
    sheet
}

fn convert_to_srp(vec: Vec<usize>) -> Vec<SpriteRenderPrimitive> {
    let mut spr_vec = Vec::new();
    for e in vec {
        spr_vec.push(SpriteRenderPrimitive::SpriteIndex(e))
    }
    spr_vec
}

fn assets_dir(resources: &Resources) -> String {
    if let Some(d) = resources.get::<AssetsDir>() {
        (*d).to_string()
    } else {
        String::default()
    }
}
