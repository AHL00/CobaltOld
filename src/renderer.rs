use crate::{window::Window, camera::Camera};

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

pub trait Renderer {
    /// Updates all renderables then renders them to one render pass
    fn render(&mut self, window: &mut Window, camera: &Camera, world: &mut hecs::World) -> anyhow::Result<()>;
}