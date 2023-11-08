use system::System;
use winit::{event_loop::EventLoop, event::{Event, WindowEvent}};

pub mod graphics;
pub mod system;
pub mod input;

pub struct App {
    pub window: graphics::Window,
    pub renderer: graphics::Renderer,
    pub input: input::Input,
    pub perf_stats: PerformanceStatistics,
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
        log::info!("Cobalt v{}", env!("CARGO_PKG_VERSION"));
        log::info!("Starting...");
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
                        WindowEvent::CloseRequested => {
                            // app.renderer.destroy();
                            elwt.exit();
                        },
                        WindowEvent::RedrawRequested => {
                            // app.renderer.render();
                        },
                        WindowEvent::Resized(s) => {
                            let res = app.window.resize(s);

                            if let Err(e) = res {
                                log::error!("Failed to resize window: {}", e);
                            }
                        },
                        WindowEvent::ScaleFactorChanged { .. } => {
                            let res = app.window.resize(app.window.winit_win.inner_size());
                            if let Err(e) = res {
                                log::error!("Failed to resize window: {}", e);
                            }
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

            let res = app.renderer.render(&mut app.window);

            if let Err(e) = res {
                log::error!("Failed to render: {}", e);
            }

            app.perf_stats.tick();
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

        log::info!("Window created.");
        
        self.app = Some(App {
            window,
            renderer: graphics::Renderer::new(),
            input: input::Input::new(),
            perf_stats: PerformanceStatistics::new(std::time::Duration::from_millis(500)),
        });

        Ok(())
    }

    pub fn register_system(&mut self, system: System) {
        if !self.systems.iter().any(|s| s.uuid == system.uuid) {
            self.systems.push(system);
        }
    }
}

pub struct PerformanceStatistics {
    pub fps: f64,
    pub avg_frame_time: f64,
    pub collection_duration: std::time::Duration,
    frame_counter: u64,
    last_collection: std::time::Instant,
}

impl PerformanceStatistics {
    pub fn new(collection_duration: std::time::Duration) -> PerformanceStatistics {
        PerformanceStatistics {
            fps: 0.0,
            avg_frame_time: 0.0,
            collection_duration,
            frame_counter: 0,
            last_collection: std::time::Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        if self.last_collection.elapsed() >= self.collection_duration {
            self.fps = self.frame_counter as f64 / self.collection_duration.as_secs_f64();
            self.avg_frame_time = 1.0 / self.fps;
            self.frame_counter = 0;
            self.last_collection = std::time::Instant::now();
        }
        
        self.frame_counter += 1;
    }
}