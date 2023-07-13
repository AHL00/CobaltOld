use glad_gl::gl;
use nalgebra_glm as glm;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    VERTEX_SHADER = gl::VERTEX_SHADER,
    FRAGMENT_SHADER = gl::FRAGMENT_SHADER,
}

/// Shader component, can be added to a shader program.
pub struct ShaderComponent {
    pub id: u32,
    pub type_: ShaderType,
}

impl Drop for ShaderComponent {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
        log::trace!("Dropping ShaderComponent {}", self.id);
    }
}

impl ShaderComponent {
    pub fn new(type_: ShaderType) -> ShaderComponent {
        let id = unsafe { gl::CreateShader(type_ as u32) };
        ShaderComponent { id, type_ }
    }

    pub fn compile(&self, source: &str) {
        unsafe {
            let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(self.id, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
        }
    }

    pub fn get_compile_status(&self) -> bool {
        let mut success: i32 = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
        }
        success == 1
    }

    pub fn get_compile_log(&self) -> String {
        let mut log_length: i32 = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut log_length);
        }
        let mut log: Vec<u8> = Vec::with_capacity(log_length as usize);
        unsafe {
            log.set_len((log_length as usize) - 1);
            gl::GetShaderInfoLog(
                self.id,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut i8,
            );
        }
        String::from_utf8(log).unwrap()
    }
}

pub struct ShaderProgram {
    pub id: u32,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        log::trace!("Dropping ShaderProgram: {}", self.id);
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        let id = unsafe { gl::CreateProgram() };
        ShaderProgram { id }
    }

    /// Attach a compiled shader to this program.
    pub fn attach(&self, shader: &ShaderComponent) {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
    }

    /// Remove a shader from this program if it is attached.
    pub fn detach(&self, shader: &ShaderComponent) {
        unsafe {
            gl::DetachShader(self.id, shader.id);
        }
    }

    // Get the attached shader's internal GL IDs.
    pub fn get_attached_shaders(&self) -> Vec<u32> {
        let mut count: i32 = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::ATTACHED_SHADERS, &mut count);
        }
        let mut shaders: Vec<u32> = Vec::with_capacity(count as usize);
        unsafe {
            shaders.set_len(count as usize);
            gl::GetAttachedShaders(
                self.id,
                count,
                std::ptr::null_mut(),
                shaders.as_mut_ptr(),
            );
        }
        shaders
    }

    pub fn link(&self) {
        unsafe {
            gl::LinkProgram(self.id);
        }
    }

    pub fn get_link_status(&self) -> bool {
        let mut success: i32 = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        }
        success == 1
    }

    pub fn get_link_log(&self) -> String {
        let mut log_length: i32 = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut log_length);
        }
        let mut log: Vec<u8> = Vec::with_capacity(log_length as usize);
        unsafe {
            log.set_len((log_length as usize) - 1);
            gl::GetProgramInfoLog(
                self.id,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut i8,
            );
        }
        String::from_utf8(log).unwrap()
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unuse_program(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let c_str = std::ffi::CString::new(name.as_bytes()).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, c_str.as_ptr()) };
        location
    }

    pub fn set_uniform_1i(&self, name: &str, value: i32) {
        let location = self.get_uniform_location(name);
        unsafe {
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_uniform_1f(&self, name: &str, value: f32) {
        let location = self.get_uniform_location(name);
        unsafe {
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_uniform_2f(&self, name: &str, value: (f32, f32)) {
        let location = self.get_uniform_location(name);
        unsafe {
            gl::Uniform2f(location, value.0, value.1);
        }
    }

    pub fn set_uniform_3f(&self, name: &str, value: (f32, f32, f32)) {
        let location = self.get_uniform_location(name);
        unsafe {
            gl::Uniform3f(location, value.0, value.1, value.2);
        }
    }

    pub fn set_uniform_4f(&self, name: &str, value: (f32, f32, f32, f32)) {
        let location = self.get_uniform_location(name);
        unsafe {
            gl::Uniform4f(location, value.0, value.1, value.2, value.3);
        }
    }

    pub fn set_uniform_mat4f(&self, name: &str, value: &glm::Mat4) {
        let location = self.get_uniform_location(name);
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
        }
    }
}