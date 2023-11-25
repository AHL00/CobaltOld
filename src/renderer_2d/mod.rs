pub mod renderables;

use ahash::AHashMap;

use crate::{window::Window, camera::Camera, renderer::Renderer, transform::Transform};

use self::renderables::{sprite::Sprite, Renderable};

pub struct Renderer2D {
    pipelines: AHashMap<std::any::TypeId, wgpu::RenderPipeline>,
    depth_texture: Option<wgpu::Texture>,
}

impl Renderer2D {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new() -> Renderer2D {
        Renderer2D {
            pipelines: AHashMap::new(),
            depth_texture: None,
        }
    }

    fn create_depth_buffer(&mut self, window: &Window) {
        let size = wgpu::Extent3d { 
            width: window.config.width,
            height: window.config.height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Buffer Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT 
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        self.depth_texture = Some(window.device.create_texture(&desc));
    }
}

impl Renderer for Renderer2D {
    fn initialize(&mut self, window: &Window) {
        log::info!("Initializing Renderer2D.");
        
        self.create_depth_buffer(window);       
    }

    fn resize_callback(&mut self, window: &Window) {
        log::info!("Resizing depth buffer texture.");

        self.create_depth_buffer(window);
    }
    
    fn render(&mut self, window: &mut Window, camera: &Camera, world: &mut hecs::World) -> anyhow::Result<()> {
        let output = window.surface.get_current_texture()?;

        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_view = self.depth_texture.as_ref().unwrap_or_else(| | {
            log::error!("Depth buffer texture not initialized!");
            panic!("Depth buffer texture not initialized!")
        }).create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = window
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // TODO: Make a macro that does this for a bunch of types automatically
            if !self.pipelines.contains_key(&Sprite::type_id()) {
                // Generate pipeline
                let pipeline = Sprite::create_pipeline(window)?;

                self.pipelines
                    .extend(std::iter::once((Sprite::type_id(), pipeline)));
            }

            let world_raw_ptr = world as *mut hecs::World;

            unsafe {
                render_pass.set_pipeline(self.pipelines.get(&Sprite::type_id()).unwrap());

                for (i, (rect, transform)) in (&mut *world_raw_ptr).query_mut::<(&mut Sprite, &mut Transform)>() {
                    rect.render(window, camera, transform, &mut render_pass)?;
                }
            }
        } // Renderpass

        window.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}