use ultraviolet::Mat4;
use wgpu::util::DeviceExt;

use crate::{
    camera::Camera, transform::Transform, uniform::Uniform, window::Window, App, Renderer2D,
};

use super::{Renderable, UvVertex};

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

struct RectRenderInfo {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    /// Automatically synced during render
    model_matrix_uniform: Uniform<Mat4>,
    color_uniform: Uniform<[f32; 4]>,
}

pub struct Rect {
    color: [f32; 4],
    _dirty_color: bool,
    render_info: Option<RectRenderInfo>,
}

impl<'a> Rect {
    pub fn new(color: (f32, f32, f32, f32)) -> Self {
        Self {
            color: [color.0, color.1, color.2, color.3],
            _dirty_color: true,
            render_info: None,
        }
    }

    fn set_color(&mut self, color: (f32, f32, f32, f32)) {
        self.color = [color.0, color.1, color.2, color.3];
        self._dirty_color = true;
    }

    fn color(&self) -> (f32, f32, f32, f32) {
        (self.color[0], self.color[1], self.color[2], self.color[3])
    }

    fn initialize(&mut self, window: &mut Window) {
        let vertex_buffer = window
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Sprite Vertex Buffer"),
                contents: bytemuck::cast_slice(&RECT_VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = window
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Sprite Index Buffer"),
                contents: bytemuck::cast_slice(&RECT_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

        let model_matrix_uniform = Uniform::<Mat4>::new(&window.device, &Mat4::identity(), 0, wgpu::ShaderStages::VERTEX);
        let color_uniform = Uniform::<[f32; 4]>::new(&window.device, &self.color, 1, wgpu::ShaderStages::VERTEX_FRAGMENT);

        self.render_info = Some(RectRenderInfo {
            vertex_buffer,
            index_buffer,
            model_matrix_uniform,
            color_uniform,
        });
    }
}

impl<'a> Renderable<'a> for Rect {
    fn render(
        &'a mut self,
        window: &mut crate::window::Window,
        camera: &'a Camera,
        transform: &'a mut Transform,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> anyhow::Result<()> {
        if self.render_info.is_none() {
            self.initialize(window);
        };

        let render_info = self.render_info.as_ref().unwrap();

        render_pass.set_vertex_buffer(0, render_info.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            render_info.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        render_pass.set_bind_group(0, &camera.bind_group, &[]);

        if transform.recalculate_matrix() {
            render_info
                .model_matrix_uniform
                .update(&transform.model_matrix(), &window.queue);
        }

        render_pass.set_bind_group(1, &render_info.model_matrix_uniform.bind_group, &[]);

        if self._dirty_color {
            render_info.color_uniform.update(&self.color, &window.queue);
            self._dirty_color = false;
        }

        render_pass.set_bind_group(2, &render_info.color_uniform.bind_group, &[]);

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

        let camera_bind_group_layout = Camera::get_bind_group_layout(&window.device);
        let model_matrix_bind_group_layout = Uniform::<Mat4>::get_bind_group_layout(&window.device, wgpu::ShaderStages::VERTEX);
        let color_bind_group_layout = Uniform::<[f32; 4]>::get_bind_group_layout(&window.device, wgpu::ShaderStages::VERTEX_FRAGMENT);

        let render_pipeline_layout =
            window
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Quad Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &camera_bind_group_layout,
                        &model_matrix_bind_group_layout,
                        &color_bind_group_layout,
                    ],
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
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: Renderer2D::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
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
