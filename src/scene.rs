use ultraviolet::Vec3;

use crate::{camera, transform::Transform};

pub struct Scene {
    pub world: hecs::World,
    pub camera: Option<camera::Camera>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: hecs::World::new(),
            camera: None,
        }
    }

    
}
