use winit::event_loop::EventLoop;



pub struct Window {
    pub winit_win: winit::window::Window,
}

impl Window {
    pub fn create(event_loop: &EventLoop<()>) -> anyhow::Result<Window> {
        let winit_win = winit::window::WindowBuilder::new()
            .with_title("Cobalt")
            .build(&event_loop)
            .unwrap();

        Ok(Window {
            winit_win,
        })
    }
}

pub struct Renderer {

}