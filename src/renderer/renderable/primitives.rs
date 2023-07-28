use super::{RenderableTrait, Renderer};

pub struct Rect {
    pub width: f32,
    pub height: f32,
    // vertex_buffer: wgpu::Buffer,
    // index_buffer: wgpu::Buffer,
    _dirty: bool,
}

impl RenderableTrait for Rect {
    fn render(&self, renderer: &mut Renderer) {
        
    }

    fn update(&self, renderer: &mut Renderer) {
        
    }
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Rect {
        Rect {
            width,
            height,
            _dirty: true,
        }
    }
}