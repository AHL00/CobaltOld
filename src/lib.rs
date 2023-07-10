#![allow(dead_code)]
#![allow(non_camel_case_types)]

// Internal crate mods
//pub(crate) mod core;
pub mod core;

// TODO: Change this to pub(crate) after testing

// Exported mods
pub mod renderer;
pub extern crate nalgebra_glm as maths;

pub mod ecs;
pub use ecs::*;
pub mod transform;
pub use transform::*;

pub struct App {
    pub renderer: renderer::Renderer,
    pub ecs: ecs::World,
    graphics_context: core::graphics::GraphicsContext,
    _current_gameobject_id: u32,
}

impl App {
    pub fn new() -> App {
        let graphics_context = core::graphics::GraphicsContext::new();
        let renderer = renderer::Renderer::new();
        
        App {
            renderer,
            ecs: ecs::World::new(),
            graphics_context,
            _current_gameobject_id: 0,
        }
    }

    pub fn run(&mut self) {
        self.graphics_context.set_window_title("Cobalt Engine");
        self.graphics_context.set_window_size(1280, 720);
        self.graphics_context.set_clear_color(0.1, 0.1, 0.1, 1.0);
        
        while !self.graphics_context.window.should_close() {
            self.graphics_context.poll_events();
            self.graphics_context.clear();

            self.renderer.render(&self.ecs);
            
            self.graphics_context.swap_buffers();
        }
    }
}