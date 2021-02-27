/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

macro_rules! generate_output {
    ($count: expr) => {{
        let mut output = Vec::<amethyst::animation::SpriteRenderPrimitive>::new();
        for i in 0..($count) {
            output.push(amethyst::animation::SpriteRenderPrimitive::SpriteIndex(i));
        }
        output
    }};
}

macro_rules! load_animation {
    ($resources:expr,$progress_counter:expr,$length:expr) => {{
        let loader = ($resources).get_mut::<amethyst::assets::DefaultLoader>().expect("oof1");

        let input: Vec<f32> = itertools_num::linspace(0.0, 1.0, $length + 1).collect();
        let output: Vec<amethyst::animation::SpriteRenderPrimitive> = generate_output!($length);

        let animations: amethyst::assets::Handle<amethyst::animation::Sampler<amethyst::animation::SpriteRenderPrimitive>> = 
            amethyst::assets::Loader::load_from_data(&*loader,
            amethyst::animation::Sampler::<amethyst::animation::SpriteRenderPrimitive> {
                input,
                output,
                function: amethyst::animation::InterpolationFunction::Step,
            },
            &mut $progress_counter,
            &$resources.get().expect("oof3"),
        );

        let animation_handle: amethyst::assets::Handle<amethyst::animation::Animation<amethyst::renderer::SpriteRender>> =
        amethyst::assets::Loader::load_from_data(
            &*loader,
            amethyst::animation::Animation::<SpriteRender>::new_single(
                0,
                amethyst::animation::SpriteRenderChannel::SpriteIndex,
                animations,
            ),
            &mut $progress_counter,
            &$resources.get().expect("oof4"),
        );

        animation_handle
    }};
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
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
