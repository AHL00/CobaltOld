use crate::window::Window;

/// A developer GUI using egui
pub(crate) struct DevGui {
    pub state: egui_winit::State,
    pub ctx: egui::Context,
    pub renderer: egui_wgpu::Renderer,
}

impl DevGui {
    pub fn new(window: &Window) -> Self {
        let ctx = egui::Context::default();

        let state = egui_winit::State::new(
            ctx.viewport_id(),
            &window.winit_win,
            Some(window.winit_win.scale_factor() as f32),
            None,
        );

        let renderer = egui_wgpu::Renderer::new(
            &window.device,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            None,
            1,
        );

        Self { state, ctx, renderer }
    }
}
