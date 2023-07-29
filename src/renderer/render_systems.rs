use crate::{ecs, Transform};

use super::{primitives::Rect, Renderer};

pub struct RenderSystemsManager {
    systems: Vec<RenderSystem>,
}

impl RenderSystemsManager {
    pub(crate) fn new() -> RenderSystemsManager {
        RenderSystemsManager {
            systems: Vec::new(),
        }
    }

    pub fn register_system(&mut self, system: RenderSystem) {
        self.systems.push(system);
    }

    pub(crate) fn run_systems(
        &self,
        ecs: &mut ecs::Ecs,
        render_pass: &mut wgpu::RenderPass<'_>,
        renderer: &Renderer,
    ) {
        for system in self.systems.iter() {
            (system.run)(&mut ecs.world, &ecs.resources, render_pass, renderer);
        }
    }
}

/// A system that runs every frame
pub struct RenderSystem {
    pub name: &'static str,
    pub run: Box<
        dyn Fn(
            &mut ecs::World,
            &ecs::resources::ResourceManager,
            &mut wgpu::RenderPass<'_>,
            &Renderer,
        ),
    >,
}

//////////////////////
/// Render Systems ///
//////////////////////

pub(crate) fn register_render_systems(systems: &mut RenderSystemsManager) {
    systems.register_system(RenderSystem {
        name: "rect_system",
        run: Box::new(super::primitives::rect_system),
    });
}