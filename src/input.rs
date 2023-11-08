pub struct Input {
    pub(crate) keys: Vec<winit::keyboard::KeyCode>,
    pub(crate) mouse: winit::event::MouseButton,
    pub(crate) mouse_pos: (f64, f64),
    pub(crate) mouse_delta: (f64, f64),
    pub(crate) mouse_wheel_delta: (f64, f64),
}

impl Input {
    pub(crate) fn new() -> Input {
        Input {
            keys: Vec::new(),
            mouse: winit::event::MouseButton::Other(0),
            mouse_pos: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
            mouse_wheel_delta: (0.0, 0.0),
        }
    }

    pub(crate) fn update(&mut self, event: &winit::event::WindowEvent) -> anyhow::Result<()> {
        match event {
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(key_code) = event.physical_key {
                    match event.state {
                        winit::event::ElementState::Pressed => {
                            self.keys.push(key_code);
                        }
                        winit::event::ElementState::Released => {
                            self.keys.retain(|&k| k != key_code);
                        }
                    }
                }
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => match state {
                winit::event::ElementState::Pressed => {
                    self.mouse = *button;
                }
                winit::event::ElementState::Released => {
                    self.mouse = winit::event::MouseButton::Other(0);
                }
            },
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_delta = (position.x - self.mouse_pos.0, position.y - self.mouse_pos.1);
                self.mouse_pos = (position.x, position.y);
            }
            winit::event::WindowEvent::MouseWheel { delta, .. } => match delta {
                winit::event::MouseScrollDelta::LineDelta(x, y) => {
                    self.mouse_wheel_delta = (*x as f64, *y as f64);
                }
                winit::event::MouseScrollDelta::PixelDelta(pos) => {
                    self.mouse_wheel_delta = (pos.x, pos.y);
                }
            },
            _ => {}
        };

        Ok(())
    }

    pub fn is_key_down(&self, key: winit::keyboard::KeyCode) -> bool {
        self.keys.contains(&key)
    }

    pub fn is_mouse_down(&self, button: winit::event::MouseButton) -> bool {
        self.mouse == button
    }

    pub fn mouse_pos(&self) -> (f64, f64) {
        self.mouse_pos
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    pub fn mouse_wheel_delta(&self) -> (f64, f64) {
        self.mouse_wheel_delta
    }
}
