use system::System;
use winit::{event_loop::EventLoop, event::Event};

pub mod graphics;
pub mod system;
pub mod input;

pub struct App {
    pub window: graphics::Window,
    // renderer: graphics::Renderer,
    pub input: input::Input,
}

pub struct AppBuilder {
    app: Option<App>,
    systems: Vec<System>,
    event_loop: Option<EventLoop<()>>,
}

impl AppBuilder {

    pub fn new() -> AppBuilder {
        AppBuilder {
            app: None,
            systems: Vec::new(),
            event_loop: None,
        }
    }
  
    pub fn run(mut self) -> anyhow::Result<()> {
        self.build()?;

        let mut app = self.app.unwrap();

        // Reset the last_run time for all systems
        for system in &mut self.systems {
            system.last_run = std::time::Instant::now();
        }

        let event_loop = self.event_loop.unwrap();

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        // Run the loop
        let systems_ptr: *mut Vec<System> = &mut self.systems;

        event_loop.run(move |event, elwt| {

            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            // app.renderer.destroy();
                            elwt.exit();
                        },
                        winit::event::WindowEvent::RedrawRequested => {
                            // app.renderer.render();
                        },
                        _ => {}
                    }
                    app.input.update(&event).expect("Failed to update input"); 
                },
                Event::AboutToWait => {
                    app.window.winit_win.request_redraw();
                },
                _ => {}
            }


            for system in unsafe { &mut *systems_ptr } {
                match system.system_type {
                    system::SystemType::Once => {
                        (system.update)(&mut app, &system.last_run.elapsed());
                        self.systems.retain(|s| s.uuid != system.uuid); 
                    },
                    system::SystemType::Timed(duration) => {
                        if system.last_run.elapsed() >= duration {
                            (system.update)(&mut app, &system.last_run.elapsed());
                            system.last_run = std::time::Instant::now();
                        }
                    },
                    system::SystemType::Update => {
                        (system.update)(&mut app, &system.last_run.elapsed());
                        system.last_run = std::time::Instant::now();
                    }
                }
            }
        })?;

        Ok(())
    }

    fn build(&mut self) -> anyhow::Result<()> {
        self.event_loop = Some(EventLoop::new()?);

        let window = if let Some(event_loop) = &self.event_loop {
            graphics::Window::create(event_loop)?
        } else {
            return Err(anyhow::anyhow!("Event loop not initialized, could not create window."));
        };
        
        self.app = Some(App {
            window,
            input: input::Input::new(),
        });

        Ok(())
    }

    pub fn register_system(&mut self, system: System) {
        if !self.systems.iter().any(|s| s.uuid == system.uuid) {
            self.systems.push(system);
        }
    }
}