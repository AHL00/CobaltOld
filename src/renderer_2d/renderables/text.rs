use ultraviolet::Mat4;
use wgpu::util::DeviceExt;

use crate::{
    assets::Asset, camera::Camera, texture::Texture, transform::Transform, uniform::Uniform,
    window::Window, App, Renderer2D,
};

use super::Renderable;

pub struct Text {
    text_buffer: Option<glyphon::Buffer>,
    text: String,
    bounds: (f32, f32),
    font_size: f32,
    line_height: f32,
    advanced_text: bool,
    last_z: f32,

    _dirty_metrics: bool,
    _dirty_text: bool,
    _dirty_bounds: bool,
}

impl<'a> Text {
    pub fn new<S>(text: S, bounds: (f32, f32), font_size: f32, line_height: f32) -> Self
    where
        S: Into<String>,
    {
        Self {
            text_buffer: None,
            text: text.into(),
            bounds,
            font_size,
            line_height,
            advanced_text: false,
            last_z: 0.0,

            _dirty_metrics: true,
            _dirty_text: true,
            _dirty_bounds: true,
        }
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self._dirty_metrics = true;
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn set_line_height(&mut self, line_height: f32) {
        self.line_height = line_height;
        self._dirty_metrics = true;
    }

    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    pub fn set_advanced_text_shaping(&mut self, advanced_text: bool) {
        self.advanced_text = advanced_text;
        self._dirty_text = true;
    }

    pub fn advanced_text_shaping(&self) -> bool {
        self.advanced_text
    }

    pub fn set_text<S>(&mut self, text: S)
    where
        S: Into<String>,
    {
        self.text = text.into();
        self._dirty_text = true;
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn set_size(&mut self, size: (f32, f32)) {
        self.bounds = size;
        self._dirty_bounds = true;
    }

    pub fn size(&self) -> (f32, f32) {
        self.bounds
    }

    pub fn initialize(&mut self, font_system: &'a mut glyphon::FontSystem) {
        self.text_buffer = Some(glyphon::Buffer::new(
            font_system,
            glyphon::Metrics { 
                font_size: 32.0,
                line_height: 40.0,
            }
        ));
    }
    
    pub(crate) fn render(
        &'a mut self,
        window: &mut crate::window::Window,
        camera: &'a Camera,
        transform: &'a mut Transform,
        font_system: &'a mut glyphon::FontSystem,
        swash_cache: &'a mut glyphon::SwashCache,
        text_atlas: &'a mut glyphon::TextAtlas,
        text_renderer: &'a mut glyphon::TextRenderer,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> anyhow::Result<()> {
        let text_buffer = if let Some(text_buffer) = &mut self.text_buffer {
            text_buffer
        } else {
            self.initialize(font_system);
            self.text_buffer.as_mut().unwrap()
        };

        if self.last_z != transform.position().z {
            self._dirty_text = true;
        }

        let need_to_shape = self._dirty_text || self._dirty_bounds || self._dirty_metrics;

        if self._dirty_text {
            text_buffer.set_text(font_system, self.text.as_str(), glyphon::Attrs::new().metadata(transform.position().z.to_bits() as usize), glyphon::Shaping::Basic);
            
            self._dirty_text = false;
        }

        if self._dirty_bounds {
            text_buffer.set_size(font_system, self.bounds.0, self.bounds.1);
            
            self._dirty_bounds = false;
        }

        if self._dirty_metrics {
            text_buffer.set_metrics(font_system, glyphon::Metrics {
                font_size: self.font_size,
                line_height: self.line_height,
            });
            
            self._dirty_metrics = false;
        }

        if need_to_shape {
            text_buffer.shape_until_scroll(font_system);
        }
        
        let screen_size = window.winit_win.inner_size();

        let text_area = glyphon::TextArea {
            buffer: &self.text_buffer.as_ref().unwrap(),
            left: 0.0,
            top: 0.0,
            scale: 1.0,
            bounds: glyphon::TextBounds {
                left: 0,
                top: 0,
                right: screen_size.width as i32,
                bottom: screen_size.height as i32,
            },
            default_color: glyphon::Color::rgb(255, 255, 255),
        };


        text_renderer.prepare_with_depth(
            &window.device,
            &window.queue,
            font_system,
            text_atlas,
            glyphon::Resolution {
                width: screen_size.width,
                height: screen_size.height,
            },
            [text_area],
            swash_cache,
            |metadata| {
                // f32::from_bits(metadata as u32)
                0.2
            }
        )?;

        text_renderer.render(text_atlas, render_pass)?;

        // Last z
        self.last_z = transform.position().z;
        Ok(())
    }
}
