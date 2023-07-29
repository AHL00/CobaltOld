use wgpu::util::DeviceExt;

use crate::{core::graphics::Graphics, ecs, App, Transform};

use super::{Color, OriginType, RenderableTrait, Renderer, Vertex};

/// A rectangle primitive
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub origin: OriginType,
    pub color: Color,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    _dirty: bool,
}

impl RenderableTrait for Rect {
    fn update(&mut self) {
        if self._dirty {}
    }
}

impl Rect {
    pub fn new(app: &App, width: f32, height: f32, color: Color) -> Rect {
        // origin is center
        let vertices = [
            Vertex {
                position: [-width / 2.0, -height / 2.0],
            },
            Vertex {
                position: [width / 2.0, -height / 2.0],
            },
            Vertex {
                position: [width / 2.0, height / 2.0],
            },
            Vertex {
                position: [-width / 2.0, height / 2.0],
            },
        ];

        let vertex_buffer = wgpu::util::BufferInitDescriptor {
            label: Some("Rect Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        };

        let index_buffer = wgpu::util::BufferInitDescriptor {
            label: Some("Rect Index Buffer"),
            contents: bytemuck::cast_slice(&[0u16, 1, 2, 0, 2, 3]),
            usage: wgpu::BufferUsages::INDEX,
        };

        Rect {
            width,
            height,
            origin: OriginType::Center,
            color,
            vertex_buffer: app.graphics.device.create_buffer_init(&vertex_buffer),
            index_buffer: app.graphics.device.create_buffer_init(&index_buffer),
            _dirty: true,
        }
    }
}

pub(crate) fn rect_system(
    world: &mut ecs::World,
    resources: &ecs::resources::ResourceManager,
    renderer: &Renderer,
) {
    let mut query = world.query::<(&mut Rect, &Transform)>();
    for (ent, (rect, transform)) in query.iter() {
        let model = transform.get_model_matrix().as_slice();

        let color = rect.color.as_array();


    }
}

pub(crate) fn rect_pipeline(graphics: &Graphics) -> wgpu::RenderPipeline {
    let shader = graphics
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Rect shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/rect.wgsl").into()),
            });

    let render_pipeline_layout =
        graphics
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
    
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
        })
}
