/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

 use std::collections::HashMap;
 use serde::{Serialize, Deserialize};
 use amethyst::assets::Handle;
 use amethyst::renderer::SpriteSheet;

// macro_rules! generate_output {
//     ($count: expr) => {{
//         let mut output = Vec::<amethyst::animation::SpriteRenderPrimitive>::new();
//         for i in 0..($count) {
//             output.push(amethyst::animation::SpriteRenderPrimitive::SpriteIndex(i));
//         }
//         output
//     }};
// }

// macro_rules! load_animation {
//     ($resources:expr,$progress_counter:expr,$length:expr) => {{
//         let loader = ($resources).get_mut::<amethyst::assets::DefaultLoader>().expect("oof1");

//         let input: Vec<f32> = itertools_num::linspace(0.0, 1.0, $length + 1).collect();
//         let output: Vec<amethyst::animation::SpriteRenderPrimitive> = generate_output!($length);

//         let animations: amethyst::assets::Handle<amethyst::animation::Sampler<amethyst::animation::SpriteRenderPrimitive>> = 
//             amethyst::assets::Loader::load_from_data(&*loader,
//             amethyst::animation::Sampler::<amethyst::animation::SpriteRenderPrimitive> {
//                 input,
//                 output,
//                 function: amethyst::animation::InterpolationFunction::Step,
//             },
//             &mut $progress_counter,
//             &$resources.get().expect("oof3"),
//         );

//         let animation_handle: amethyst::assets::Handle<amethyst::animation::Animation<amethyst::renderer::SpriteRender>> =
//         amethyst::assets::Loader::load_from_data(
//             &*loader,
//             amethyst::animation::Animation::<SpriteRender>::new_single(
//                 0,
//                 amethyst::animation::SpriteRenderChannel::SpriteIndex,
//                 animations,
//             ),
//             &mut $progress_counter,
//             &$resources.get().expect("oof4"),
//         );

//         animation_handle
//     }};
// }

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum AnimationId {
    Attack(u32),
    Dash,
    Jump,
    Run,
    Idle,
}

impl Default for AnimationId {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug)]
pub struct AnimationData {
    pub current_animation : Option<AnimationId>,
    pub sheet_map : HashMap<AnimationId, Handle<SpriteSheet>>,
}