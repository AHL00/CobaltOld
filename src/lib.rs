use camera::Camera;
use system::System;
use ultraviolet::{Rotor3, Vec3};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::transform::Transform;

pub mod assets;
pub mod camera;
pub mod input;
pub mod renderer;
pub mod resources;
pub mod system;
pub mod texture;
pub mod transform;
pub mod window;

#[cfg(feature = "renderer_2d")]
pub mod renderer_2d;
pub use renderer_2d::Renderer2D;

pub(crate) mod uniform;


pub struct App {
    pub window: window::Window,
    pub camera: camera::Camera,
    pub renderer: Box<dyn renderer::Renderer>,
    pub input: input::Input,
    pub resources: resources::ResourceManager,
    pub assets: assets::AssetManager,
    pub world: hecs::World,
    pub perf_stats: PerformanceStatistics,
}

pub struct AppBuilder {
    app: Option<App>,
    systems: Vec<System>,
    event_loop: Option<EventLoop<()>>,
    renderer: Option<Box<dyn renderer::Renderer>>,
}

impl AppBuilder {
    pub fn new() -> AppBuilder {
        AppBuilder {
            app: None,
            systems: Vec::new(),
            event_loop: None,
            renderer: None,
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

        // Run all the startup systems
        for system in &mut self.systems {
            if let system::SystemType::Startup = system.system_type {
                (system.update)(&mut app, &system.last_run.elapsed());
            }
        }

        // Remove all the startup systems
        self.systems.retain(|s| {
            if let system::SystemType::Startup = s.system_type {
                false
            } else {
                true
            }
        });

        // Run the loop
        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            // Cleanup

                            elwt.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            // Update and run systems
                            for system in &mut self.systems {
                                match system.system_type {
                                    system::SystemType::Timed(duration) => {
                                        if system.last_run.elapsed() >= duration {
                                            (system.update)(&mut app, &system.last_run.elapsed());
                                            system.last_run = std::time::Instant::now();
                                        }
                                    }
                                    system::SystemType::Update => {
                                        (system.update)(&mut app, &system.last_run.elapsed());
                                        system.last_run = std::time::Instant::now();
                                    }
                                    _ => {}
                                }
                            }

                            app.assets.update_ref_counts();

                            // Update camera buffer
                            app.camera.update_uniform(&app.window);

                            // Render
                            let res =
                                app.renderer
                                    .render(&mut app.window, &app.camera, &mut app.world);

                            if let Err(e) = res {
                                log::error!("Failed to render: {}", e);
                            }

                            app.perf_stats.tick();
                        }
                        WindowEvent::Resized(s) => {
                            let res = app.window.resize(s);

                            if let Err(e) = res {
                                log::error!("Failed to resize window: {}", e);
                            }
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            let res = app.window.resize(app.window.winit_win.inner_size());
                            if let Err(e) = res {
                                log::error!("Failed to resize window: {}", e);
                            }
                        }
                        _ => {}
                    }
                    app.input.update(&event).expect("Failed to update input");
                }
                Event::AboutToWait => {
                    app.window.winit_win.request_redraw();
                }
                _ => {}
            }
        })?;

        Ok(())
    }

    fn build(&mut self) -> anyhow::Result<()> {
        self.event_loop = Some(EventLoop::new()?);

        let window = if let Some(event_loop) = &self.event_loop {
            window::Window::create(event_loop)?
        } else {
            return Err(anyhow::anyhow!(
                "Event loop not initialized, could not create window."
            ));
        };

        let camera = Camera::new(
            // Transform::new(
            //     Vec3::new(0.0, 1.0, 2.0),
            //     Rotor3::from_euler_angles(0.0, 0.0, 180.0_f32.to_radians()),
            //     Vec3::one(),
            // ),
            // camera::Projection::Perspective { 
            //     fov: 70.0,
            //     aspect: 1.7778,
            //     near: 0.1,
            //     far: 100.0,
            // },

            Transform::new(
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(0.0, 0.0, 180.0_f32.to_radians()),
                Vec3::one(),
            ),
            camera::Projection::Orthographic {
                aspect: 1.7778,
                height: 10.0,
                near: 0.1,
                far: 100.0,
            },
            &window,
        );

        log::info!("Window created.");

        if self.renderer.is_none() {
            return Err(anyhow::anyhow!("No renderer specified."));
        }

        self.app = Some(App {
            window,
            camera,
            renderer: self.renderer.take().unwrap(),
            resources: resources::ResourceManager::new(),
            assets: assets::AssetManager::new(),
            world: hecs::World::new(),
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

    pub fn with_renderer(mut self, renderer: Box<dyn renderer::Renderer>) -> Self {
        self.renderer = Some(renderer);

        self
    }
}

pub struct PerformanceStatistics {
    pub fps: f64,
    pub avg_frame_time: std::time::Duration,
    pub collection_duration: std::time::Duration,
    frame_counter: u64,
    last_collection: std::time::Instant,
}

impl PerformanceStatistics {
    pub fn new(collection_duration: std::time::Duration) -> PerformanceStatistics {
        PerformanceStatistics {
            fps: 0.0,
            avg_frame_time: std::time::Duration::from_secs(0),
            collection_duration,
            frame_counter: 0,
            last_collection: std::time::Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        if self.last_collection.elapsed() >= self.collection_duration {
            self.fps = self.frame_counter as f64 / self.collection_duration.as_secs_f64();
            self.avg_frame_time = self.collection_duration / self.frame_counter as u32;
            self.frame_counter = 0;
            self.last_collection = std::time::Instant::now();
        }

        self.frame_counter += 1;
    }
}
