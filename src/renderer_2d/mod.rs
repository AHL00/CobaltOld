pub mod renderables;

use ahash::AHashMap;

use crate::{window::Window, camera::Camera, renderer::{Renderable, Renderer}, transform::Transform};

use self::renderables::rect::Rect;

pub struct Renderer2D {
    pipelines: AHashMap<std::any::TypeId, wgpu::RenderPipeline>,
}

impl Renderer2D {
    pub fn new() -> Renderer2D {
        Renderer2D {
            pipelines: AHashMap::new(),
        }
    }
}

impl Renderer for Renderer2D {
    fn render(&mut self, window: &mut Window, camera: &Camera, world: &mut hecs::World) -> anyhow::Result<()> {
        let output = window.surface.get_current_texture()?;
        let view = output
            .texture
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
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // TODO: Make a macro that does this for a bunch of types automatically
            if !self.pipelines.contains_key(&Rect::type_id()) {
                // Generate pipeline
                let pipeline = Rect::create_pipeline(window)?;

                self.pipelines
                    .extend(std::iter::once((Rect::type_id(), pipeline)));
            }

            let world_raw_ptr = world as *mut hecs::World;

            unsafe {
                render_pass.set_pipeline(self.pipelines.get(&Rect::type_id()).unwrap());
                for (i, (rect, transform)) in (&mut *world_raw_ptr).query_mut::<(&mut Rect, &mut Transform)>() {
                    rect.render(window, camera, transform, &mut render_pass)?;
                }
            }
        } // Renderpass

        window.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}