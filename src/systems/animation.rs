/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */
use crate::components::animation::{AnimationData, AnimationId};
use crate::entities::PlayerState;
use amethyst::animation::{
    get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
};
use amethyst::ecs::SystemBuilder;
use amethyst::core::timing::Time;
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;

#[derive(Default)]
pub struct PlayerAnimation;

impl System for PlayerAnimation {
    fn build(self) -> Box<dyn ParallelRunnable> {
        Box::new(
            SystemBuilder::new("PlayerAnimation")
                .with_query(<(
                    Entity,
                    Read<PlayerState>,
                    Read<AnimationSet<AnimationId, SpriteRender>>,
                )>::query())
                .write_component::<AnimationControlSet<AnimationId, SpriteRender>>()
                .build(move |mut commands, world, _, query| {
                    let (subworld, mut remainder) = world.split_for_query(query);
                    for (entity, state, animation_set) in query.iter(&subworld) {
                        if let Some(control_set) =
                            get_animation_set(&mut remainder, &mut commands, *entity)
                        {
                            if !state.running && !control_set.has_animation(AnimationId::Idle) {
                                control_set.add_animation(
                                    AnimationId::Idle,
                                    &animation_set.get(&AnimationId::Idle).unwrap(),
                                    EndControl::Loop(None),
                                    1.0,
                                    AnimationCommand::Start,
                                );
                            } else if state.running {
                                control_set.abort(AnimationId::Idle);
                            }
                        }
                    }
                }),
        )
    }
}

pub struct AnimationController;

impl System for AnimationController {
    fn build(self) -> Box<dyn ParallelRunnable> {
        Box::new(
            SystemBuilder::new("AnimationController")
                .with_query(<(
                    Entity,
                    Read<AnimationData>,
                    Write<SpriteRender>,
                    Read<AnimationSet<AnimationId, SpriteRender>>,
                )>::query())
                .write_component::<AnimationControlSet<AnimationId, SpriteRender>>()
                .read_component::<AnimationSet<AnimationId, SpriteRender>>()
                .read_component::<AnimationData>()
                .write_component::<SpriteRender>()
                .build(move |mut commands, world, _, query| {
                    let (mut subworld, mut remainder) = world.split_for_query(query);

                    for (entity, data, sprite, set) in query.iter_mut(&mut subworld) {
                        if let Some(control) =
                            get_animation_set(&mut remainder, &mut commands, *entity)
                        {
                            if control.has_animation(data.current_animation) {
                                continue;
                            }

                            clear_control_set(control);

                            control.add_animation(
                                data.current_animation,
                                &set.get(&data.current_animation).unwrap(), // TODO: Seems unsafe
                                EndControl::Loop(None), // TODO: Should be a parameter of some sort
                                1.0,
                                AnimationCommand::Start,
                            );
                            if let Some(required_sheet) =
                                data.sheet_map.get(&data.current_animation)
                            {
                                println!("{:?}", data.sheet_map);
                                sprite.sprite_sheet = required_sheet.clone();
                            }
                        }
                    }
                }),
        )
    }
}

fn clear_control_set(control: &mut AnimationControlSet<AnimationId, SpriteRender>) {
    let ids = {
        let mut collected_ids = Vec::new();
        for (id, _) in &control.animations {
            collected_ids.push(id.clone());
        }
        collected_ids
    };

    for id in ids {
        control.abort(id);
    }
}

pub struct AnimationSwitch;

impl System for AnimationSwitch {
    fn build(self) -> Box<dyn ParallelRunnable> {
        Box::new(
            SystemBuilder::new("AnimationSwitch")
                .with_query(<Write<AnimationData>>::query())
                .read_resource::<Time>()
                .write_component::<AnimationData>()
                    .build(move  |_, world, time, query| {
                        for anim_data in query.iter_mut(world) {
                            let current_time = time.delta_seconds() + anim_data.current_time.unwrap_or(0.0);
                            anim_data.current_time = Some(current_time);
                            if current_time > 10.0 {
                                if anim_data.current_animation == AnimationId::Run {
                                    anim_data.current_animation = AnimationId::Dash;
                                } else {
                                    anim_data.current_animation = AnimationId::Run
                                }
                                anim_data.current_time = Some(0.0);
                            }
                        }
                }),
        )
    }
}