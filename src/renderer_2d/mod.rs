pub mod renderables;

use ahash::AHashMap;

use crate::{camera::Camera, renderer::Renderer, transform::Transform, window::Window};

use self::renderables::{sprite::Sprite, Renderable, TranslucentSprite};

pub struct Renderer2D {
    pipelines: AHashMap<std::any::TypeId, wgpu::RenderPipeline>,
    depth_texture: Option<wgpu::Texture>,

    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    text_atlas: Option<glyphon::TextAtlas>,
    text_renderer: Option<glyphon::TextRenderer>,
}

impl Renderer2D {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new() -> Renderer2D {
        Renderer2D {
            pipelines: AHashMap::new(),
            depth_texture: None,
            font_system: glyphon::FontSystem::new(),
            swash_cache: glyphon::SwashCache::new(),
            text_atlas: None,
            text_renderer: None,
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        self.depth_texture = Some(window.device.create_texture(&desc));
    }
}

impl Renderer for Renderer2D {
    fn initialize(&mut self, window: &Window) {
        log::info!("Initializing Renderer2D.");

        self.text_atlas = Some(glyphon::TextAtlas::new(
            &window.device,
            &window.queue,
            window.config.format,
        ));

        self.create_depth_buffer(window);

        self.text_renderer = Some(glyphon::TextRenderer::new(
            self.text_atlas.as_mut().unwrap(),
            &window.device,
            wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            Some(wgpu::DepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
        ));
    }

    fn resize_callback(&mut self, window: &Window) {
        log::info!("Resizing depth buffer texture.");

        self.create_depth_buffer(window);
    }

    fn render(
        &mut self,
        window: &mut Window,
        camera: &Camera,
        world: &mut hecs::World,
    ) -> anyhow::Result<()> {
        let output = window.surface.get_current_texture()?;

        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_view = self
            .depth_texture
            .as_ref()
            .unwrap_or_else(|| {
                log::error!("Depth buffer texture not initialized!");
                panic!("Depth buffer texture not initialized!")
            })
            .create_view(&wgpu::TextureViewDescriptor::default());

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
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
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

            if !self.pipelines.contains_key(&TranslucentSprite::type_id()) {
                // Generate pipeline
                let pipeline = TranslucentSprite::create_pipeline(window)?;

                self.pipelines
                    .extend(std::iter::once((TranslucentSprite::type_id(), pipeline)));
            }

            let world_raw_ptr = world as *mut hecs::World;

            unsafe {
                render_pass.set_pipeline(self.pipelines.get(&Sprite::type_id()).unwrap());

                for (i, (renderable, transform)) in
                    (&mut *world_raw_ptr).query_mut::<(&mut Sprite, &mut Transform)>()
                {
                    renderable.render(window, camera, transform, &mut render_pass)?;
                }

                render_pass
                    .set_pipeline(self.pipelines.get(&TranslucentSprite::type_id()).unwrap());

                for (i, (renderable, transform)) in
                    (&mut *world_raw_ptr).query_mut::<(&mut TranslucentSprite, &mut Transform)>()
                {
                    renderable.render(window, camera, transform, &mut render_pass)?;
                }
            }

            let mut buffer = glyphon::Buffer::new(
                &mut self.font_system,
                glyphon::Metrics {
                    font_size: 32.0,
                    line_height: 40.0,
                },
            );

            buffer.set_size(&mut self.font_system, 800.0, 120.0);
            buffer.set_text(&mut self.font_system,  "Hello world! 👋\nThis is rendered with 🦅 glyphon 🦁\nThe text below should be partially clipped.\na b c d e f g h i j k l m n o p q r s t u v w x y z", glyphon::Attrs::new().family(glyphon::Family::SansSerif), glyphon::Shaping::Advanced);
            buffer.shape_until_scroll(&mut self.font_system);

            let size = window.winit_win.inner_size();

            self.text_renderer.as_mut().unwrap().prepare(
                &window.device,
                &window.queue,
                &mut self.font_system,
                self.text_atlas.as_mut().unwrap(),
                glyphon::Resolution {
                    width: size.width as u32,
                    height: size.height as u32,
                },
                [glyphon::TextArea {
                    buffer: &buffer,
                    top: 10.0,
                    left: 10.0,
                    scale: 1.0,
                    bounds: glyphon::TextBounds {
                        left: 0,
                        top: 0,
                        right: 800,
                        bottom: 120,
                    },
                    default_color: glyphon::Color::rgb(255, 255, 255),
                }],
                &mut self.swash_cache,
            )?;

            self.text_renderer
                .as_mut()
                .unwrap()
                .render(self.text_atlas.as_ref().unwrap(), &mut render_pass)?;
        }
        window.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
