use glad_gl::gl;

use super::image::Image;

#[repr(u32)]
pub enum TextureWrap {
    REPEAT = gl::REPEAT,
    MIRRORED_REPEAT = gl::MIRRORED_REPEAT,
    CLAMP_TO_EDGE = gl::CLAMP_TO_EDGE,
    CLAMP_TO_BORDER = gl::CLAMP_TO_BORDER,
}

#[repr(u32)]
pub enum TextureFilter {
    NEAREST = gl::NEAREST,
    LINEAR = gl::LINEAR,
    NEAREST_MIPMAP_NEAREST = gl::NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST = gl::LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR = gl::NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR = gl::LINEAR_MIPMAP_LINEAR,
}

// Exported to user, do not exponse internals.
#[derive(Clone, Copy)]
pub struct Texture {
    pub(crate) id: u32,
    pub(crate) texture_unit: u32,
}

impl Texture {
    pub fn new() -> Texture {
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Texture { id, texture_unit: 0 }
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub(crate) fn unbind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    /// End user function, should not be used internally.
    pub fn set_image(&self, image: &Image) {
        self.bind();
        unsafe {
            let format = match image.channels {
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => panic!("Unsupported number of channels"),
            };
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                image.width as i32,
                image.height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const std::ffi::c_void,
            );
        }
        self.generate_mipmaps();
        self.unbind();
    }

    /// Texture must be bound before calling this function.
    pub fn save_image_to_file(&self, path: &str, size: (u32, u32)) {
        let mut data = vec![0u8; (size.0 * size.1 * 3) as usize];
        unsafe {
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_mut_ptr() as *mut std::ffi::c_void,
            );
        }

        let image = Image {
            width: size.0,
            height: size.1,
            channels: 3,
            data: data.into_boxed_slice(),
        };

        image.save_to_file(path);
    }

    pub fn set_wrap(&self, x_wrap: TextureWrap, y_wrap: TextureWrap) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, x_wrap as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, y_wrap as i32);
        }
    }

    pub fn set_filter(&self, min_filter: TextureFilter, mag_filter: TextureFilter) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub(crate) fn generate_mipmaps(&self) {
        unsafe {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
}