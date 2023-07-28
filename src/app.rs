use crate::core;
use crate::ecs;
use crate::resources;
use crate::renderer;

use winit::{event::*, event_loop::ControlFlow};

pub struct App {
    graphics: core::graphics::Graphics,
    pub window: core::graphics::Window,
    pub renderer: renderer::Renderer,

    pub input: core::input::Input,

    pub ecs: ecs::World,
    pub resources: resources::ResourceManager,
    pub fps_counter: core::utils::FpsCounter,

    _current_gameobject_id: u32,
}

impl App {
    pub fn new() -> App {
        // get crate version
        let version = env!("CARGO_PKG_VERSION");
        log::info!("Cobalt Engine v{}", version);

        let window = core::graphics::Window::new();
        let graphics = core::graphics::Graphics::new(&window);
        let renderer = renderer::Renderer::new();

        let ecs = ecs::World::new();
        let resources = resources::ResourceManager::new();
        let fps_counter = core::utils::FpsCounter::new(2.5);

        let input = core::input::Input::new();

        App {
            window,
            graphics,
            renderer,
            input,
            ecs,
            resources,
            fps_counter,
            _current_gameobject_id: 0,
        }
    }

    pub fn run(mut self) {
        self.window.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            if self.fps_counter.is_refreshed() {
                self.window.winit.set_title(&format!(
                    "Cobalt Engine v{} - FPS: {}",
                    env!("CARGO_PKG_VERSION"),
                    self.fps_counter.fps
                ));
            }

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        self.graphics.resize(size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.graphics.resize(*new_inner_size);
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        self.input.process_key_event(&input);
                    },
                    _ => (),
                },
                Event::MainEventsCleared => {
                    self.window.winit.request_redraw();
                },
                Event::RedrawRequested(_) => {
                    self.fps_counter.tick();
                    match self.graphics.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => self.graphics.resize(self.graphics.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => log::error!("{:?}", e),
                    }
                },
                _ => (),
            }
        });
    }

    pub fn window(&self) -> &core::graphics::Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut core::graphics::Window {
        &mut self.window
    } 
}