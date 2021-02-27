/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

use amethyst::core::ecs::{Join, System, WriteStorage, Entities};
use amethyst::animation::{get_animation_set, AnimationSet, AnimationControlSet, EndControl, AnimationCommand};
use amethyst::renderer::SpriteRender;
use crate::components::animation::*;

pub struct DefaultAnimation;

impl<'a> System<'a> for DefaultAnimation {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<AnimationId, SpriteRender>>
    );

    fn run(&mut self, (entities, animation_sets, mut animation_control_sets) : Self::SystemData) {
        for (entity, animation_set) in (&*entities, &animation_sets).join() {
            if let Some(animation_control_set) = get_animation_set(&mut animation_control_sets, entity) {
                if animation_control_set.is_empty() {
                    animation_control_set.add_animation(
                        AnimationId::Idle,
                        animation_set.get(&AnimationId::Idle).expect("yikes"), // TODO: This should not be yikes
                        EndControl::Loop(None),
                        1.0,
                        AnimationCommand::Start);
                }
            }
        }
    }
}

