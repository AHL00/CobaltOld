// Exports
mod renderable;
pub use renderable::*;

use crate::{core::graphics::Graphics, ecs};

mod render_systems;
use render_systems::register_render_systems;

use self::render_systems::{RenderSystem, RenderSystemsManager};

pub struct Renderer {
    render_pipeline: wgpu::RenderPipeline,
    systems: RenderSystemsManager,
}

impl Renderer {
    pub(crate) fn new(graphics: &Graphics) -> Renderer {
        let shader = graphics
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
            });

        let render_pipeline_layout =
            graphics
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: graphics.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

        let mut systems = RenderSystemsManager::new();

        register_render_systems(&mut systems);

        Renderer {
            render_pipeline,
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

        // render_pass.set_pipeline(&self.render_pipeline);

        // render_pass.draw(0..3, 0..1);

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
