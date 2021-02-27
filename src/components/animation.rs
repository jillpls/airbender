/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::animation::{Sampler, SpriteRenderPrimitive, InterpolationFunction, Animation, AnimationSet};
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::assets::{ProgressCounter, Handle, DefaultLoader, Loader};
use amethyst::prelude::*;
use amethyst::core::Transform;
use itertools_num::{linspace};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
#[allow(dead_code)]
pub enum AnimationId {
    Attack(u32),
    Dash,
    Jump,
    Run,
    Idle
}

impl Default for AnimationId {
    fn default() -> Self {
        Self::Idle
    }
}

pub fn load_animation(path : &str, data: &mut StateData<'_, GameData>, progress_counter : Option<ProgressCounter>) -> ProgressCounter {
    let StateData {
        world, resources, ..
    } = data;

    let mut progress_counter = progress_counter.unwrap_or_default();

    {
        let loader = resources.get_mut::<DefaultLoader>().expect("oof1");

        println!("Path: {}", path);

        let texture = loader.load(&(path.to_owned() + ".png")); 
        let sprites = loader.load(&(path.to_owned() + ".ron")); 

        let sheet : Handle<SpriteSheet> = loader.load_from_data(
            SpriteSheet {texture, sprites},
            &mut progress_counter,
            &resources.get().expect("oof2")
        );

        let animation_length = 4; // TODO: not all animations have length 4

        let input : Vec<f32> = linspace(0.0, 1.0,  animation_length + 1).collect();
        let output : Vec<SpriteRenderPrimitive> = generate_output(animation_length);

        let animations : Handle<Sampler<SpriteRenderPrimitive>> = loader.load_from_data(
            Sampler::<SpriteRenderPrimitive> {
                input,
                output,
                function: InterpolationFunction::Step,
            },
            &mut progress_counter,
            &resources.get().expect("oof3")
        );

        let animation_handle : Handle<Animation<SpriteRender>> = loader.load_from_data(
            Animation::<SpriteRender>::new_single(
                0,
                amethyst::animation::SpriteRenderChannel::SpriteIndex,
                animations
            ),
            &mut progress_counter,
            &resources.get().expect("oof4")
        );

        let mut animation_set = AnimationSet::new();
        animation_set.insert(AnimationId::Idle, animation_handle);
        let mut transform = Transform::default();
        transform.set_translation_xyz(250.0,250.0, 0.0);
        world.push((SpriteRender::new(sheet, 0), transform, animation_set));
        //world.push((SpriteRender::new(sheet, 0), transform));
    }

    progress_counter
}



// pub fn load_animation(world : &mut World, p : &str, progress_counter : Option<ProgressCounter>) -> amethyst::Result<ProgressCounter>{

//     // TODO: REMOVE UNHANDLED EXPECTS - also known as line 54 - 

//     let mut handle = load_sprite_sheet(world, SpriteSheetType::Animation(0), p, progress_counter)?;

//     // let animation_length = sheet_storage.get(&handle).expect("oof2").sprites.len();

//     let animation_length = 4; // TODO: not all animations have length 4

//     let input : Vec<f32> = linspace(0.0, 1.0,  animation_length + 1).collect();
//     let output : Vec<SpriteRenderPrimitive> = generate_output(animation_length);

//     println!("Input: {:?}, Output: {:?}", input, output);
//     let sampler = Sampler::<SpriteRenderPrimitive> {
//             input,
//             output,
//             function: InterpolationFunction::Step
//         };

//     let sampler_storage = world.get_mut::<AssetStorage<Sampler<SpriteRenderPrimitive>>>().expect("oof3");
//     let sampler_handle = sampler_storage.insert(sampler);
    
//     let animation = Animation::<SpriteRender>::new_single(
//         0,
//         amethyst::animation::SpriteRenderChannel::SpriteIndex,
//         sampler_handle
//     );

//     let animation_storage = world.get_mut::<AssetStorage<Animation<SpriteRender>>>().expect("oof4");
//     let animation_handle = animation_storage.insert(animation);

//     let mut animation_set = AnimationSet::<AnimationId, SpriteRender>::new();

//     animation_set.insert(AnimationId::Idle, animation_handle);

//     let mut transform = Transform::default();
//     transform.set_translation_xyz(500.0, 500.0, 0.0);
//     transform.set_scale(Vector3::new(6.0, 6.0, 1.0));

//     let e = world
//         .create_entity()
//         .with(SpriteRender::new(handle.handle.take().expect("oof5"), 0))
//         .with(transform)
//         .with(animation_set)
//         .build();

//     Ok(handle.progress_counter.take().unwrap_or(ProgressCounter::new()))
// }

fn generate_output(count : usize) -> Vec<SpriteRenderPrimitive> {
    let mut output = Vec::<SpriteRenderPrimitive>::new();
    for i in 0..(count) {
        output.push(SpriteRenderPrimitive::SpriteIndex(i));
    }
    output
}