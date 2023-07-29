use wgpu::util::DeviceExt;

use crate::{ecs, Transform, App};

use super::{RenderableTrait, Renderer, OriginType, Color, Vertex};

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
        let vertices = [
            Vertex {
                position: [0.0, 0.0],
            },
            Vertex {
                position: [width, 0.0],
            },
            Vertex {
                position: [width, height],
            },
            Vertex {
                position: [0.0, height],
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
    render_pass: &mut wgpu::RenderPass<'_>,
    renderer: &Renderer,
) {
    let mut query = world.query::<(&mut Rect, &Transform)>();
    for (ent, (rect, transform)) in query.iter() {
        
    }
}
