use crate::components::animation::AnimationId;
use crate::entities::PlayerState;
use amethyst::animation::{AnimationControlSet, AnimationSet, get_animation_set, EndControl, AnimationCommand};
use amethyst::ecs::SystemBuilder;
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
                            if !state.running && !control_set.has_animation(AnimationId::Idle){
                                control_set.add_animation(
                                    AnimationId::Idle,
                                    &animation_set.get(&AnimationId::Idle).unwrap(),
                                    EndControl::Loop(None),
                                    1.0,
                                    AnimationCommand::Start,
                                );
                            } else if state.running {
                                control_set.abort(
                                    AnimationId::Idle
                                );
                            }
                        }
                    }
                }),
        )
    }
}
