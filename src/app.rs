use crate::core;
use crate::ecs;
use crate::renderer;

use winit::event;
use winit::event_loop::EventLoop;
use winit::{event::*, event_loop::ControlFlow};

pub struct AppBuilder {
    event_loop: EventLoop<()>,
    app: App,
    init: Option<Box<dyn FnOnce(&mut App) -> ()>>,
}

impl AppBuilder {
    pub fn new() -> AppBuilder {
        let event_loop = EventLoop::new();
        let app = App::new(&event_loop);

        AppBuilder { event_loop, app, init: None }
    }

    /// Accepts a closure that will be called once the app has started.
    pub fn init(&mut self, on_start: impl FnOnce(&mut App) -> () + 'static) {
        self.init = Some(Box::new(on_start));
    }

    pub fn run(self) {
        let AppBuilder { event_loop, mut app, init: on_start } = self;

        if let Some(on_start) = on_start {
            on_start(&mut app);
        }

        app.run(event_loop);
    }
}

pub struct App {
    pub(crate) graphics: core::graphics::Graphics,
    pub window: core::graphics::Window,
    pub renderer: renderer::Renderer,

    pub input: core::input::Input,

    pub ecs: ecs::Ecs,
    pub fps_counter: core::utils::FpsCounter,

    _current_gameobject_id: u32,
}

impl App {
    fn new(event_loop: &EventLoop<()>) -> App {
        // get crate version
        let version = env!("CARGO_PKG_VERSION");
        log::info!("Cobalt Engine v{}", version);

        let window = core::graphics::Window::new(event_loop);
        let graphics = core::graphics::Graphics::new(&window);
        let renderer = renderer::Renderer::new(&graphics);

        let ecs = ecs::Ecs::new();
        let fps_counter = core::utils::FpsCounter::new(2.5);

        let input = core::input::Input::new();

        App {
            window,
            graphics,
            renderer,
            input,
            ecs,
            fps_counter,
            _current_gameobject_id: 0,
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
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

                    let mut frame = self.renderer.create_frame(&self.graphics).unwrap();
                    
                    match self.renderer.render(&mut frame, &mut self.ecs) {
                        Ok(_) => {
                            self.renderer.present_frame(&self.graphics, frame);
                        }
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