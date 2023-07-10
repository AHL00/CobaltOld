use glad_gl::gl;
use glfw::Context;
use std::{rc::Rc, sync::mpsc::Receiver};

// exports
pub use glfw::CursorMode;
pub mod buffer;
pub use buffer::Buffer; // Export Buffer trait for VertexBuffer and IndexBuffer
pub mod shader;
pub mod texture;
pub mod image;

pub struct GraphicsContext {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    events: Rc<Receiver<(f64, glfw::WindowEvent)>>,
}

impl Drop for GraphicsContext {
    fn drop(&mut self) {
        
    }
}

impl GraphicsContext {
    pub fn new() -> GraphicsContext {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));

        let (mut window, events) = glfw
            .create_window(800, 600, "", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        
        gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

        let events = Rc::new(events);

        window.set_key_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);

        GraphicsContext {
            glfw: glfw,
            window: window,
            events: events,
        }
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.window.set_size(width as i32, height as i32);
    }

    pub fn set_window_pos(&mut self, x: i32, y: i32) {
        self.window.set_pos(x, y);
    }

    pub fn get_primary_monitor(&self) -> glfw::Monitor {
        glfw::Monitor::from_primary()
    }

    pub fn get_monitors<F>(&mut self, callback: F)
    where
        F: FnOnce(&[glfw::Monitor]),
    {
        self.glfw.with_connected_monitors(move |_, m| {
            callback(m);
        });
    }

    pub fn set_fullscreen(
        &mut self,
        width: u32,
        height: u32,
        refresh_rate: u32,
        monitor: &glfw::Monitor,
    ) {
        let monitor = monitor;
        self.window.set_monitor(
            glfw::WindowMode::FullScreen(&monitor),
            0,
            0,
            width as u32,
            height as u32,
            Some(refresh_rate),
        );
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.window.set_cursor_mode(mode);
    }
}
