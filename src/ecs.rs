pub use hecs::*;

/// Component which is attached to an entity to make it a child of another entity
pub struct Parent {
    entity: Entity,
}

impl Parent {
    pub fn new(entity: Entity) -> Parent {
        Parent { entity }
    }
}