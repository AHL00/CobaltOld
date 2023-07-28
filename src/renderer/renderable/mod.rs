use hecs::QueryBorrow;

use super::Renderer;

pub mod primitives;

pub(crate) trait RenderableTrait {
    fn render(&self, renderer: &mut Renderer);
    fn update(&self, renderer: &mut Renderer);
}