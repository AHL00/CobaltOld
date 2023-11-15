use ultraviolet::Vec4;
use wgpu::util::DeviceExt;

use crate::{assets::Asset, renderer::Renderable, texture::Texture, App, window::Window};

use super::UvVertex;

const RECT_VERTICES: &[UvVertex] = &[
    UvVertex {
        position: [-0.5, 0.5, 0.0],
        uv: [0.0, 0.0],
    }, // A
    UvVertex {
        position: [-0.5, -0.5, 0.0],
        uv: [0.0, 1.0],
    }, // B
    UvVertex {
        position: [0.5, -0.5, 0.0],
        uv: [1.0, 1.0],
    }, // C
    UvVertex {
        position: [0.5, 0.5, 0.0],
        uv: [1.0, 0.0],
    }, // D
];

const RECT_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub struct Rect {
    color: Vec4,
    texture: Option<Asset<Texture>>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl Rect {
    pub fn new(app: &App, color: Vec4) -> Self {
        let vertex_buffer =
            app.window
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Quad Vertex Buffer"),
                    contents: bytemuck::cast_slice(&RECT_VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let index_buffer =
            app.window
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Quad Index Buffer"),
                    contents: bytemuck::cast_slice(&RECT_INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                });

        Self {
            color,
            texture: None,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn with_texture(app: &App, texture: Asset<Texture>) -> Self {
        let vertex_buffer =
            app.window
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Quad Vertex Buffer"),
                    contents: bytemuck::cast_slice(&RECT_VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let index_buffer =
            app.window
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Quad Index Buffer"),
                    contents: bytemuck::cast_slice(&RECT_INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                });

        Self {
            color: Vec4::one(),
            texture: Some(texture),
            vertex_buffer,
            index_buffer,
        }
    }
}

impl<'a> Renderable<'a> for Rect {
    fn update(&mut self, window: &mut crate::window::Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(
        &'a self,
        window: &mut crate::window::Window,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> anyhow::Result<()> {

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        if let Some(texture) = &self.texture {
            render_pass.set_bind_group(0, &texture.bind_group, &[]);
        }

        render_pass.draw_indexed(0..RECT_INDICES.len() as u32, 0, 0..1);

        Ok(())
    }

    fn type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn create_pipeline(window: &mut Window) -> anyhow::Result<wgpu::RenderPipeline> {
        let shader = window
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/rect.wgsl"));

        let texture_bind_group_layout = Texture::get_bind_group_layout(&window.device);

        let render_pipeline_layout =
            window
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Quad Render Pipeline Layout"),
                    bind_group_layouts: &[&texture_bind_group_layout],
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
                        entry_point: "vs_main",
                        buffers: &[UvVertex::desc()],
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
