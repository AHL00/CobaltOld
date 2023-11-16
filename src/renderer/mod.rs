pub mod renderables;

use ahash::AHashMap;

use crate::{window::Window, camera::Camera};

use self::renderables::rect::Rect;

pub trait Renderable<'a> {
    // Called right before the render function
    // Used to do things like update the vertex buffer
    fn update(&mut self, window: &mut Window) -> anyhow::Result<()>;

    // Called after the pipeline generated at the start is set to the render_pass
    // fn render(&mut self, window: &mut Window, encoder: &mut wgpu::CommandEncoder) -> anyhow::Result<()>;
    fn render(
        &'a self,
        window: &mut Window,
        camera: &'a Camera,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> anyhow::Result<()>;

    // Generate the render pipeline at the start, store in a hashmap with the type_id
    // If the type_id doesn't exist in the hash, the renderer will call the get_pipeline function.
    // When rendering next, get the pipeline from the hashmap
    fn type_id() -> std::any::TypeId;
    fn create_pipeline(window: &mut Window) -> anyhow::Result<wgpu::RenderPipeline>;
}

pub struct Renderer {
    pipelines: AHashMap<std::any::TypeId, wgpu::RenderPipeline>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            pipelines: AHashMap::new(),
        }
    }

    /// Updates all renderables then renders them to one render pass
    pub fn render(&mut self, window: &mut Window, camera: &Camera, world: &mut hecs::World) -> anyhow::Result<()> {
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
                for (i, rect) in (&mut *world_raw_ptr).query_mut::<&mut Rect>() {
                    rect.render(window, camera, &mut render_pass)?;
                }
            }
        } // Renderpass

        window.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
