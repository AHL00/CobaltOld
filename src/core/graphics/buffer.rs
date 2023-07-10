use glad_gl::gl;

#[repr(u32)]
pub enum BufferUsage {
    STATIC_DRAW = gl::STATIC_DRAW,
    DYNAMIC_DRAW = gl::DYNAMIC_DRAW,
    STRAEAM_DRAW = gl::STREAM_DRAW,
}

pub trait Buffer<T> {
    fn new() -> Self;
    fn set_data(&mut self, data: &[T], usage: BufferUsage);
    fn bind(&self);
    fn unbind(&self);
}

pub struct VertexBuffer {
    pub id: u32,
    pub size: usize,
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl Buffer<f32> for VertexBuffer {
    fn new() -> VertexBuffer {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        VertexBuffer { id, size: 0 }
    }

    fn set_data(&mut self, vertices: &[f32], usage: BufferUsage) {
        self.size = vertices.len();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.size * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const std::ffi::c_void,
                usage as u32,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

pub struct IndexBuffer {
    pub id: u32,
    pub size: usize,
}

impl Buffer<u32> for IndexBuffer {
    fn new() -> IndexBuffer {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        IndexBuffer { id, size: 0 }
    }

    fn set_data(&mut self, indices: &[u32], usage: BufferUsage) {
        self.size = indices.len();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.size * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const std::ffi::c_void,
                usage as u32,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

#[repr(u32)]
pub enum DataType {
    FLOAT = gl::FLOAT,
    UNSIGNED_INT = gl::UNSIGNED_INT,
    UNSIGNED_BYTE = gl::UNSIGNED_BYTE,
}

pub struct VertexArray {
    id: u32,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArray { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    /// Sets the layout of the vertex array.
    /// It takes a slice of u32s, where each u32 represents the number of floats in each element.
    /// The index of the element is the index of the attribute in the shader.
    /// Example:
    /// ```ignore
    /// let vertices = [
    /// // position       color          tex_coords
    ///  -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    ///   0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
    ///   0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
    ///  -0.5,  0.5, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0,
    /// ];
    /// 
    /// // Shader:
    /// // layout (location = 0) in vec3 aPos;
    /// // layout (location = 1) in vec3 aColor;
    /// // layout (location = 2) in vec2 aTexCoord;
    /// // ...
    /// 
    /// let layout = [3, 3, 2];
    /// vertex_array.set_layout(&layout, DataType::FLOAT, false);
    /// ```
    /// This would mean that the vertex array has 3 floats for the position, 3 floats for the color, and 2 floats for the texture coordinates.
    /// 
    /// Arguments:
    /// * `layout` - The layout of the vertex array.
    /// * `type_` - The type of the data in the vertex array.
    /// * `normalized` - Whether the data should be normalized.
    /// 
    /// For the type, each variant corresponds to a different rust type:
    /// * `FLOAT` - f32
    /// * `UNSIGNED_INT` - u32
    /// * `UNSIGNED_BYTE` - u8
    pub fn set_layout(&mut self, layout: &[u32], type_: DataType, normalized: bool) {
        let mut offset = 0;
        let size_of = match type_ {
            DataType::FLOAT => std::mem::size_of::<f32>(),
            DataType::UNSIGNED_INT => std::mem::size_of::<u32>(),
            DataType::UNSIGNED_BYTE => std::mem::size_of::<u8>(),
        };
        let stride = layout.iter().sum::<u32>() as i32 * size_of as i32;
        let normalized: u8 = if normalized { gl::TRUE } else { gl::FALSE };

        for (i, element) in layout.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    i as u32,
                    *element as i32,
                    gl::FLOAT,
                    normalized,
                    stride,
                    offset as *const std::ffi::c_void,
                );
                gl::EnableVertexAttribArray(i as u32);
            }
            offset += *element as usize * size_of;
        }
    }
}