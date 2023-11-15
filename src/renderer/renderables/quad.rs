use ultraviolet::Vec4;
use wgpu::util::DeviceExt;

use crate::{renderer::Renderable, window::Window, App};

use super::ColorVertex;

pub struct Quad {
    pub color: Vec4,

    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) num_indices: u32,    
}

const QUAD_VERTICES: &[ColorVertex] = &[
    ColorVertex {
        position: [-0.5, 0.5, 0.0],
        color: [1.0, 0.0, 0.0, 1.0],
    }, // A
    ColorVertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0, 1.0],
    }, // B
    ColorVertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0, 1.0],
    }, // C
    ColorVertex {
        position: [0.5, 0.5, 0.0],
        color: [1.0, 1.0, 1.0, 1.0],
    }, // D
];

const QUAD_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

impl Quad {
    pub fn new(app: &App) -> Self {
        let vertex_buffer = app.window.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad ColorVertex Buffer"),
            contents: bytemuck::cast_slice(&QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = app.window.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Index Buffer"),
            contents: bytemuck::cast_slice(&QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            vertex_buffer,
            index_buffer,
            num_indices: QUAD_INDICES.len() as u32,
        }
    }
}

impl<'a> Renderable<'a> for Quad {
    fn update(&mut self, window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(
        &'a self,
        window: &mut Window,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> anyhow::Result<()> {
        
        // Bind groups
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

        Ok(())
    }

    fn type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn create_pipeline(window: &mut Window) -> anyhow::Result<wgpu::RenderPipeline> {
        let shader = window
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/quad.wgsl"));

        let render_pipeline_layout =
            window
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Quad Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            window
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main", // 1.
                        buffers: &[ColorVertex::desc()],           // 2.
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: window.config.format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                        // or Features::POLYGON_MODE_POINT
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
                    // If the pipeline will be used with a multiview render pass, this
                    // indicates how many array layers the attachments will have.
                    multiview: None,
                });

                Ok(render_pipeline)
            }

}


