use crate::{window::Window, camera::Camera};

pub trait Renderer {
    /// Updates all renderables then renders them to one render pass
    fn render(&mut self, window: &mut Window, camera: &Camera, world: &mut hecs::World) -> anyhow::Result<()>;
}