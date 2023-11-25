pub mod sprite;
pub use sprite::Sprite;
pub mod translucent_sprite;
pub use translucent_sprite::TranslucentSprite;

use crate::{window::Window, camera::Camera, transform::Transform};

pub trait Renderable<'a> {
    // Called right before the render function
    // Used to do things like update the vertex buffer
    fn update(&mut self, window: &mut Window) -> anyhow::Result<()>;

    // Called after the pipeline generated at the start is set to the render_pass
    // fn render(&mut self, window: &mut Window, encoder: &mut wgpu::CommandEncoder) -> anyhow::Result<()>;
    fn render(
        &'a self,
        window: &mut Window,
        camera: &'a Camera,
        transform: &'a mut Transform,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> anyhow::Result<()>;

    // Generate the render pipeline at the start, store in a hashmap with the type_id
    // If the type_id doesn't exist in the hash, the renderer will call the get_pipeline function.
    // When rendering next, get the pipeline from the hashmap
    fn type_id() -> std::any::TypeId;
    fn create_pipeline(window: &mut Window) -> anyhow::Result<wgpu::RenderPipeline>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl ColorVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColorVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UvVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl UvVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<UvVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UvColorVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

impl UvColorVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<UvColorVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<[f32; 2]>())
                        as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 2,
                },
            ],
        }
    }
}