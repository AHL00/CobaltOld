use winit::event_loop::EventLoop;
use pollster::FutureExt;

pub struct Window {
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub winit_win: winit::window::Window,
}

impl Window {
    pub fn create(event_loop: &EventLoop<()>) -> anyhow::Result<Window> {
        let winit_win = winit::window::WindowBuilder::new()
            .with_title("Cobalt")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .with_fullscreen(None)
            .with_decorations(true)
            .with_resizable(true)
            .build(event_loop)
            .unwrap();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&winit_win) }?;

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).block_on();

        if adapter.is_none() {
            return Err(anyhow::anyhow!("Failed to find a suitable GPU adapter."));
        }

        let adapter = adapter.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ).block_on()?;

        let size = winit_win.inner_size();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);

        let present_mode = surface_caps.present_modes.iter()
            .copied()
            .find(|f| *f == wgpu::PresentMode::Immediate)
            .unwrap_or(surface_caps.present_modes[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(Window { 
            surface,
            device,
            queue,
            config,
            winit_win 
        })
    }

    pub(crate) fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) -> anyhow::Result<()> {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Window size cannot be zero."))
        }
    }
}