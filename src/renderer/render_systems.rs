use crate::{ecs, Transform, core::graphics::Graphics};

use super::{primitives::{Rect, rect_pipeline}, Renderer};

pub struct RenderSystemsManager {
    systems: Vec<RenderSystem>,
}

impl<'a> RenderSystemsManager {
    pub(crate) fn new() -> RenderSystemsManager {
        RenderSystemsManager {
            systems: Vec::new(),
        }
    }

    pub fn register_system(&mut self, system: RenderSystem) {
        self.systems.push(system);
    }

    pub(crate) fn run_systems(
        &'a self,
        ecs: &mut ecs::Ecs,
        render_pass: &mut wgpu::RenderPass<'a>,
        renderer: &Renderer,
    ) {
        for system in self.systems.iter() {
            render_pass.set_pipeline(&system.pipeline);
            (system.run)(&mut ecs.world, &ecs.resources, renderer);
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
            &Renderer,
        ),
    >,
    pub pipeline: wgpu::RenderPipeline,
}

//////////////////////
/// Render Systems ///
//////////////////////

pub(crate) fn register_render_systems(systems: &mut RenderSystemsManager, graphics: &Graphics) {
    systems.register_system(RenderSystem {
        name: "rect_system",
        run: Box::new(super::primitives::rect_system),
        pipeline: rect_pipeline(graphics),
    });
}