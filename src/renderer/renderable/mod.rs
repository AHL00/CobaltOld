use super::Renderer;

pub mod primitives;

pub(crate) trait RenderableTrait {
    fn update(&mut self);
}

pub enum OriginType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(color: (f32, f32, f32, f32)) -> Self {
        Color {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub position: [f32; 2],
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct ColorVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct TextureVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
