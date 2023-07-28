pub use hecs::*;

pub mod resources;
use self::resources::ResourceManager;

pub struct Ecs {
    pub world: World,
    pub resources: ResourceManager,
    systems: Vec<System>,
    // TODO: add fixed systems, setting fixed update rate in the Ecs struct
}

impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            world: World::new(),
            resources: ResourceManager::new(),
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }

    pub fn run_systems(&mut self) {
        for system in &mut self.systems {
            (system.run)(&mut self.world);
        }
    }
}

/// Component which is attached to an entity to make it a child of another entity
pub struct Parent {
    entity: Entity,
}

impl Parent {
    pub fn new(entity: Entity) -> Parent {
        Parent { entity }
    }
}

pub enum SystemType {
    /// Runs every frame
    Frame,
    /// Runs every fixed amount of time, set in the Ecs struct
    Fixed,
}

// Systems are functions that are executed every frame, or fixed amount of time
pub struct System {
    pub name: String,
    pub run: Box<dyn FnMut(&mut World)>,
    pub type_: SystemType,
}