use ultraviolet::{Vec4, Mat4};
use wgpu::util::DeviceExt;

use crate::{
    assets::Asset, camera::Camera, texture::Texture, window::Window, App, transform::{Transform, self}, uniform::Uniform,
};

use super::{UvVertex, Renderable, UvColorVertex};

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

pub struct Sprite {
    texture: Option<Asset<Texture>>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    /// Automatically synced during render
    model_matrix_uniform: Uniform<Mat4>,
}

impl Sprite {
    pub fn new(app: &App, texture: Asset<Texture>) -> Self {
        let vertex_buffer =
            app.window
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Sprite Vertex Buffer"),
                    contents: bytemuck::cast_slice(&RECT_VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let index_buffer =
            app.window
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Sprite Index Buffer"),
                    contents: bytemuck::cast_slice(&RECT_INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let transform_uniform = Uniform::<Mat4>::new(&app.window.device, &Mat4::identity(), 0);

        Self {
            texture: Some(texture),
            vertex_buffer,
            index_buffer,
            model_matrix_uniform: transform_uniform,
        }
    }
}

impl<'a> Renderable<'a> for Sprite {
    fn update(&mut self, window: &mut crate::window::Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(
        &'a self,
        window: &mut crate::window::Window,
        camera: &'a Camera,
        transform: &'a mut Transform,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> anyhow::Result<()> {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        if let Some(texture) = &self.texture {
            render_pass.set_bind_group(0, &texture.bind_group, &[]);
        }

        render_pass.set_bind_group(1, &camera.bind_group, &[]);

        if transform.recalculate_matrix() {
            self.model_matrix_uniform.update(&transform.model_matrix(), &window.queue);
        }

        render_pass.set_bind_group(2, &self.model_matrix_uniform.bind_group, &[]);

        render_pass.draw_indexed(0..RECT_INDICES.len() as u32, 0, 0..1);

        Ok(())
    }

    fn type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn create_pipeline(window: &mut Window) -> anyhow::Result<wgpu::RenderPipeline> {
        let shader = window
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/sprite.wgsl"));

        let texture_bind_group_layout = Texture::get_bind_group_layout(&window.device);
        let camera_bind_group_layout = Camera::get_bind_group_layout(&window.device);
        let model_matrix_bind_group_layout = Uniform::<Mat4>::get_bind_group_layout(&window.device);

        let render_pipeline_layout =
            window
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Quad Render Pipeline Layout"),
                    bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout, &model_matrix_bind_group_layout],
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
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
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
