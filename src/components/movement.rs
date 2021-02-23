use::amethyst::ecs::{Component, DenseVecStorage};
use::amethyst::core::{
    Transform,
    math::{Vector2, Vector3}
};

pub struct Motion {
    movement: Vector2<f32>
}

impl Default for Motion {
    fn default() -> Self {
        Motion {
            movement: Vector2::new(0.0, 0.0)
        }
    }
}

impl Component for Motion {
    type Storage = DenseVecStorage<Self>;
}

impl Motion {
    pub fn apply(&mut self, transform : &mut Transform) {
        transform.append_translation_xyz(*self.movement.get(0).unwrap_or(&0.0),
                                         *self.movement.get(1).unwrap_or(&0.0),
                                         0.0);
    }
}
