use crate::{renderer::Renderable, window::Window};


pub struct TestTriangle {
    // pub(crate) pipeline: wgpu::RenderPipeline,
    // pub(crate) vertex_buffer: wgpu::Buffer,
    // pub(crate) index_buffer: wgpu::Buffer,
    // pub(crate) num_indices: u32,
    // pub(crate) _marker: std::marker::PhantomData<&'a ()>,
}

impl TestTriangle {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl<'a> Renderable<'a> for TestTriangle {
    fn render(&'a mut self, window: &mut Window, render_pass: &mut wgpu::RenderPass<'a>) -> anyhow::Result<()> {


        Ok(())
    }

    fn type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn get_pipeline() -> anyhow::Result<wgpu::RenderPipeline> {
        
    }
}

// impl RenderableTrait<'static> for TestTriangle {

// }