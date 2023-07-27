use std::cell::RefCell;
use std::rc::Rc;

use crate::core;
use crate::ecs;
use crate::renderer;
use crate::resources;

pub struct App {
    //pub renderer: renderer::Renderer,
    pub ecs: ecs::World,
    pub fps_counter: core::utils::FpsCounter,

    resources: Rc<RefCell<resources::ResourceManager>>,
    //pub graphics_context: core::graphics::GraphicsContext,
    _current_gameobject_id: u32,
}

impl App {
    pub fn new() -> App {

        // get crate version
        let version = env!("CARGO_PKG_VERSION");
        log::info!("Cobalt Engine v{}", version);
        
        //let graphics_context = core::graphics::GraphicsContext::new();
        //let renderer = renderer::Renderer::new();

        App {
            //renderer,
            ecs: ecs::World::new(),
            fps_counter: core::utils::FpsCounter::new(2.5),
            resources: Rc::new(RefCell::new(resources::ResourceManager::new())),
            //graphics_context,
            _current_gameobject_id: 0,
        }
    }

    pub fn run(&mut self) {
        // self.graphics_context.set_window_title("Cobalt Engine");
        // self.graphics_context.set_window_size(1280, 720);
        // self.graphics_context.set_clear_color(0.1, 0.1, 0.1, 1.0);

        // while !self.graphics_context.window.should_close() {
        //     self.fps_counter.tick();
        //     self.graphics_context.poll_events();
        //     self.graphics_context.clear();

        //     if self.fps_counter.is_refreshed() {
        //         self.graphics_context
        //             .set_window_title(&format!("Cobalt Engine - FPS: {}", self.fps_counter.fps));
        //     }

        //     self.renderer.render(&self.ecs, &self.resources.borrow());

        //     self.graphics_context.swap_buffers();
        // }
    }

    pub fn res(&self) -> std::cell::Ref<resources::ResourceManager> {
        self.resources.borrow()
    }

    pub fn res_mut(&self) -> std::cell::RefMut<resources::ResourceManager> {
        self.resources.borrow_mut()
    }
}
