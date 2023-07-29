// Exports
mod renderable;
pub use renderable::*;

use crate::{core::graphics::Graphics, ecs};

mod render_systems;
use render_systems::register_render_systems;

use self::render_systems::{RenderSystem, RenderSystemsManager};

pub struct Renderer {
    systems: RenderSystemsManager,
}

impl Renderer {
    pub(crate) fn new(graphics: &Graphics) -> Renderer {
        let mut systems = RenderSystemsManager::new();

        register_render_systems(&mut systems, graphics);

        Renderer {
            systems,
        }
    }

    pub(crate) fn create_frame(&self, graphics: &Graphics) -> Result<Frame, wgpu::SurfaceError> {
        Ok(Frame::new(graphics))
    }

    pub(crate) fn render(
        &mut self,
        frame: &mut Frame,
        ecs: &mut ecs::Ecs,
    ) -> Result<(), wgpu::SurfaceError> {
        let mut render_pass = frame
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.25,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

        self.systems.run_systems(ecs, &mut render_pass, &*self);

        Ok(())
    }

    pub(crate) fn present_frame(&self, graphics: &Graphics, frame: Frame) {
        graphics.queue.submit(Some(frame.encoder.finish()));

        frame.texture.present();
    }
}

// TODO: Implement pipeline for each renderable type, figure out how to iterate over them in ecs

pub(crate) struct Frame {
    pub texture: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
    // pub depth_view: wgpu::TextureView,
}

impl Frame {
    fn new(graphics: &Graphics) -> Frame {
        let texture = graphics.surface.get_current_texture().unwrap();

        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        Frame {
            texture,
            view,
            encoder,
        }
    }
}

pub(crate) struct RenderPass {
    pub(crate) render_pass: wgpu::RenderPass<'static>,
}
