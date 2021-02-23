use amethyst::derive::SystemDesc;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};

use crate::components::movement::Motion;
use crate::components::collision::Collider;

#[derive(SystemDesc)]
pub struct MotionSystem;

impl<'a> System<'a> for MotionSystem {
    type SystemData = (
        WriteStorage<'a, Motion>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Collider>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut motions, mut transforms, colliders) = data;
        for (mut motion, mut transform, _) in (&mut motions, &mut transforms, !&colliders).join() {
            motion.apply(transform);
        }
    }
}