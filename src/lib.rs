pub mod assets;
pub mod camera;
pub mod input;
pub mod physics;
pub mod renderer;
pub mod resources;
pub mod scene;
pub mod system;
pub mod texture;
pub mod transform;
pub mod window;

#[cfg(feature = "renderer_2d")]
pub mod renderer_2d;

#[cfg(feature = "renderer_2d")]
pub use renderer_2d::Renderer2D;
#[cfg(feature = "renderer_3d")]
pub mod renderer_3d;
#[cfg(feature = "renderer_3d")]
pub use renderer_3d::Renderer3D;
#[cfg(feature = "physics_2d")]
pub mod physics_2d;
#[cfg(feature = "physics_2d")]
pub use physics_2d::Physics2D;

#[cfg(feature = "physics_3d")]
pub mod physics_3d;
#[cfg(feature = "physics_3d")]
pub use physics_3d::Physics3D;

#[cfg(feature = "dev_gui")]
pub mod dev_gui;

pub(crate) mod uniform;

pub struct App {
    pub window: window::Window,
    pub renderer: Box<dyn renderer::Renderer>,
    pub input: input::Input,
    pub resources: resources::ResourceManager,
    pub assets: assets::AssetManager,
    pub scenes: scene::ScenesManager,
    pub physics: Option<Box<dyn physics::Physics>>,
    pub perf_stats: PerformanceStatistics,
    #[cfg(feature = "dev_gui")]
    dev_gui: dev_gui::DevGui,
}

impl App {
    pub fn new() -> AppBuilder {
        AppBuilder::new()
    }
}

pub struct AppBuilder {
    app: Option<App>,
    systems: Vec<system::System>,
    renderer: Option<Box<dyn renderer::Renderer>>,
    physics: Option<Box<dyn physics::Physics>>,

    #[cfg(feature = "dev_gui")]
    dev_gui_setup: Option<Box<dyn FnMut(&mut egui::Context)>>,
    #[cfg(feature = "dev_gui")]
    dev_gui_callback: Option<Box<dyn FnMut(&mut App, &mut egui::Context)>>,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
            .with_physics(Box::new(Physics2D::new()))
            .with_renderer(Box::new(Renderer2D::new()))
    }
}

impl AppBuilder {
    pub fn new() -> AppBuilder {
        AppBuilder {
            app: None,
            systems: Vec::new(),
            renderer: None,
            physics: None,
            #[cfg(feature = "dev_gui")]
            dev_gui_setup: None,
            #[cfg(feature = "dev_gui")]
            dev_gui_callback: None,
        }
    }

    fn run_event_system(&mut self, event: crate::system::EventCallbackType, app: &mut App) {
        // Iterate over all systems
        // Find a system where the system_type is Event(event)
        // Run that system

        for system in &mut self.systems {
            if let system::SystemType::EventCallback(e) = &system.system_type {
                if *e == event {
                    (system.update)(app, &system.last_run.elapsed());
                }
            }
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        log::info!("Cobalt v{}", env!("CARGO_PKG_VERSION"));
        log::info!("Starting...");

        let event_loop = self.build()?;

        let mut app = self.app.take().expect("App not initialized.");

        // The scene manager takes a mutable pointer to the app to pass it along to the scene generator.
        // Read more in the scene module.
        // TODO: Find a better way to do this.
        app.scenes.app_ref = &mut app as *mut App;

        // Initialize the renderer
        app.renderer.as_mut().initialize(&app.window);

        // Reset the last_run time for all systems
        for system in &mut self.systems {
            system.last_run = std::time::Instant::now();
        }

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        // Run all the startup systems
        for system in &mut self.systems {
            if let system::SystemType::Startup = system.system_type {
                (system.update)(&mut app, &system.last_run.elapsed());
            }
        }

        // Remove all the startup systems
        self.systems
            .retain(|s| !matches!(s.system_type, system::SystemType::Startup));

        // Run the loop
        event_loop.run(|event, elwt| {
            use winit::event::{Event, WindowEvent};

            match event {
                Event::WindowEvent { event, .. } => {
                    #[cfg(feature = "dev_gui")]
                    let egui_response = app.dev_gui.state.on_window_event(&app.dev_gui.ctx, &event);

                    // If consumed, don't pass the event to the rest of the app
                    #[cfg(feature = "dev_gui")]
                    if egui_response.consumed {
                        // return;
                    }

                    match event {
                        WindowEvent::CloseRequested => {
                            // Cleanup
                            self.run_event_system(system::EventCallbackType::ShutDown, &mut app);

                            elwt.exit();

                            // Drop all assets
                            app.assets.drop_all();
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
                            if let Some(scene) = app.scenes.current_mut() {
                                if let Some(camera) = &mut scene.camera {
                                    camera.update_uniform(&app.window);

                                    // Render
                                    let res = app.renderer.render(
                                        &mut app.window,
                                        camera,
                                        &mut scene.world,
                                    );

                                    if let Err(e) = res {
                                        log::error!("Failed to render: {}", e);
                                    }

                                    // Egui render
                                    #[cfg(feature = "dev_gui")]
                                    {
                                        if let Some(cb) = &mut self.dev_gui_callback {
                                            if egui_response.repaint {
                                                let raw_input = app
                                                    .dev_gui
                                                    .state
                                                    .take_egui_input(&mut app.window.winit_win);

                                                // This is safe because app.dev_gui is not accessible from inside the callback
                                                unsafe {
                                                    app.dev_gui.ctx.begin_frame(raw_input);

                                                    let ctx_ptr: *mut egui::Context =
                                                        (&mut app.dev_gui.ctx)
                                                            as *mut egui::Context;

                                                    cb(&mut app, &mut *ctx_ptr);                                                    
                                                }

                                                let output =
                                                    app.dev_gui.ctx.end_frame();
    
                                                app.dev_gui.state.handle_platform_output(&app.window.winit_win, &app.dev_gui.ctx, output.platform_output);
    
                                                let clipped_primitive=  app.dev_gui.ctx.tessellate(
                                                    output.shapes,
                                                    output.pixels_per_point
                                                );
    
                                                let mut encoder = app.window.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                                    label: Some("Egui Render Encoder"),
                                                });

                                                {
                                                    let view = app.window.surface.get_current_texture().expect("Could not get current texture!").texture.create_view(&wgpu::TextureViewDescriptor::default());
                                                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                                        label: Some("Egui Render Pass"),
                                                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                                            view: &view,
                                                            resolve_target: None,
                                                            ops: wgpu::Operations {
                                                                load: wgpu::LoadOp::Load,
                                                                store: wgpu::StoreOp::Store,
                                                            },
                                                        })],
                                                        depth_stencil_attachment: None,
                                                        occlusion_query_set: None,
                                                        timestamp_writes: None,
                                                    });

                                                    let size = app.window.winit_win.inner_size();

                                                    let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
                                                        size_in_pixels: [size.width as u32, size.height as u32],
                                                        pixels_per_point: app.window.winit_win.scale_factor() as f32,
                                                    };
    
                                                    app.dev_gui.renderer
                                                        .render(
                                                            &mut render_pass,
                                                            clipped_primitive.as_slice(),
                                                            &screen_descriptor,
                                                        );
                                                }
                                            }        
                                        }
                                    }
                                } else {
                                    log::error!("No camera in scene!");
                                }
                            } else {
                                log::error!("No scene loaded!");
                            }

                            app.perf_stats.tick();
                        }
                        WindowEvent::Resized(s) => {
                            let res = app.window.resize(s);

                            // Event: WindowResize
                            self.run_event_system(
                                system::EventCallbackType::WindowResize,
                                &mut app,
                            );

                            if let Err(e) = res {
                                log::error!("Failed to resize window: {}", e);
                            }

                            app.renderer.resize_callback(&app.window);
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            let res = app.window.resize(app.window.winit_win.inner_size());

                            // Event: WindowResize
                            self.run_event_system(
                                system::EventCallbackType::WindowResize,
                                &mut app,
                            );

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

            if let Some(physics) = &mut app.physics {
                if let Some(scene) = app.scenes.current_mut() {
                    physics.simulate(&mut scene.world);
                }
            }
        })?;

        Ok(())
    }

    fn build(&mut self) -> anyhow::Result<winit::event_loop::EventLoop<()>> {
        let event_loop = winit::event_loop::EventLoop::new()?;

        let window = window::Window::create(&event_loop)?;

        log::info!("Window created.");

        if self.renderer.is_none() {
            return Err(anyhow::anyhow!("No renderer specified."));
        }

        #[cfg(feature = "dev_gui")]
        let mut dev_gui = dev_gui::DevGui::new(&window);

        #[cfg(feature = "dev_gui")]
        self.dev_gui_setup.take().map(|mut cb| cb(&mut dev_gui.ctx));

        self.app = Some(App {
            window,
            renderer: self.renderer.take().unwrap(),
            resources: resources::ResourceManager::new(),
            assets: assets::AssetManager::new(),
            scenes: scene::ScenesManager::new(),
            input: input::Input::new(),
            physics: if let Some(physics) = self.physics.take() {
                Some(physics)
            } else {
                None
            },
            perf_stats: PerformanceStatistics::new(std::time::Duration::from_millis(500)),

            #[cfg(feature = "dev_gui")]
            dev_gui,
        });

        Ok(event_loop)
    }

    #[cfg(feature = "dev_gui")]
    pub fn with_dev_gui_startup<T>(mut self, callback: T) -> Self
    where
        T: FnMut(&mut egui::Context) + 'static,
    {
        self.dev_gui_setup = Some(Box::new(callback));

        self
    }

    #[cfg(feature = "dev_gui")]
    pub fn with_dev_gui<T>(mut self, callback: T) -> Self
    where
        T: FnMut(&mut App, &mut egui::Context) + 'static,
    {
        self.dev_gui_callback = Some(Box::new(callback));

        self
    }

    pub fn register_system(&mut self, system: system::System) {
        if !self.systems.iter().any(|s| s.uuid == system.uuid) {
            self.systems.push(system);
        }
    }

    pub fn with_renderer(mut self, renderer: Box<dyn renderer::Renderer>) -> Self {
        self.renderer = Some(renderer);

        self
    }

    pub fn with_physics(mut self, physics: Box<dyn physics::Physics>) -> Self {
        self.physics = Some(physics);

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
            self.avg_frame_time = if self.frame_counter == 0 {
                std::time::Duration::from_secs(0)
            } else {
                self.collection_duration / self.frame_counter as u32
            };
            self.frame_counter = 0;
            self.last_collection = std::time::Instant::now();
        }

        self.frame_counter += 1;
    }
}
