
pub use winit::event::VirtualKeyCode as Key;

pub struct Input {
    keys_states: [bool; 1024],

}

impl Input {
    pub fn new() -> Input {
        Input {
            keys_states: [false; 1024],
        }
    }

    pub(crate) fn process_key_event(&mut self, event: &winit::event::KeyboardInput) {
        if let Some(keycode) = event.virtual_keycode {
            match event.state {
                winit::event::ElementState::Pressed => self.keys_states[keycode as usize] = true,
                winit::event::ElementState::Released => self.keys_states[keycode as usize] = false,
            }
        }
    }

    pub fn key_pressed(&self, key: u32) -> bool {
        self.keys_states[key as usize]
    }
}