pub use hecs::*;

pub mod resources;
use self::resources::ResourceManager;

pub struct Ecs {
    pub world: World,
    pub resources: ResourceManager,
    pub systems: SystemsManager,
    // TODO: add fixed systems, setting fixed update rate in the Ecs struct
}

impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            world: World::new(),
            resources: ResourceManager::new(),
            systems: SystemsManager::new(),
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

pub struct SystemsManager {
    systems: Vec<System>,
}

impl SystemsManager {
    pub(crate) fn new() -> SystemsManager {
        SystemsManager {
            systems: Vec::new(),
        }
    }

    pub fn register_system(&mut self, system: System) {
        self.systems.push(system);
    }

    pub(crate) fn run_systems(&mut self, world: &mut World, resources: &mut ResourceManager) {
        for system in 0..self.systems.len() {
            (self.systems[system].run)(world, resources);
        }
    }
}

pub enum SystemRunType {
    /// Runs every frame
    Frame,
    /// Runs every fixed amount of time, set in the Ecs struct
    Fixed,
}

// Systems are functions that are executed every frame, or fixed amount of time
pub struct System {
    pub name: &'static str,
    pub run: Box<dyn FnMut(&mut World, &mut ResourceManager)>,
    pub run_type: SystemRunType,
}