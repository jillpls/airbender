use::amethyst::ecs::{Component, VecStorage, DenseVecStorage};

#[derive(Default)]
pub struct CollisionRectangle {
    width: u32,
    height: u32
}

impl Component for CollisionRectangle {
    type Storage = VecStorage<Self>;
}

impl CollisionRectangle {
    pub fn new(width : u32, height: u32) -> Self {
        CollisionRectangle {
            width,
            height
        }
    }
}

#[derive(Default)]
pub struct Collider {
    collided : bool
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
