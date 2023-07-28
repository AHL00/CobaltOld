use pollster::FutureExt as _;
use winit::{event_loop::EventLoop, window::WindowBuilder};

pub struct Window {
    /// Underlying winit window.
    pub(crate) winit: winit::window::Window,
    pub(crate) event_loop: EventLoop<()>,
}

impl Window {
    pub(crate) fn new() -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("")
            .build(&event_loop)
            .unwrap();

        Window {
            winit: window,
            event_loop,
        }
    }
}

/// Internal engine only use struct, not to be exported.
pub(crate) struct Graphics {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl Graphics {
    pub fn new(window: &Window) -> Graphics {
        let size = window.winit.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window.winit) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap_or_else(|| {
                log::error!("Failed to find a suitable adapter");
                panic!("Failed to find a suitable adapter");
            });

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .block_on()
            .unwrap_or_else(|e| {
                log::error!("Failed to create device: {:?}", e);
                panic!("Failed to create device: {:?}", e);
            });

        let surface_cap = surface.get_capabilities(&adapter);

        // Look for sRGB surface, if not found, use the first available format
        let surface_fmt = surface_cap
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or_else(|| {
                log::error!(
                    "sRGB surface format not found, falling back to {:?}",
                    surface_cap.formats[0]
                );
                surface_cap.formats[0]
            });

        // Start with no vsync
        let present_mode = wgpu::PresentMode::AutoNoVsync;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_fmt,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode: surface_cap.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Graphics {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // make sure new_size is not 0
        if new_size.width == 0 || new_size.height == 0 {
            log::error!("Tried to resize surface to 0, ignoring.");
            return;
        }
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.25,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));

        output.present();

        Ok(())
    }
}
