use std::io::Write;
use std::fs::File;

use stb_image::image::{self, LoadResult};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub channels: u32,
    pub data: Box<[u8]>,
}

impl Image {
    pub fn from_file(path: &str) -> Result<Image, String> {
        let load_res = image::load(path);

        match load_res {
            LoadResult::Error(e) => Err(e),
            LoadResult::ImageU8(image) => {
                let width = image.width as u32;
                let height = image.height as u32;
                let channels = image.depth as u32;
                let data = image.data.into_boxed_slice();
                Ok(Image {
                    width,
                    height,
                    channels,
                    data,
                })
            }
            LoadResult::ImageF32(_image) => {
                println!("ImageF32 not implemented");
                Err("ImageF32 not implemented".to_string())
            }
        }
    }

    pub fn from_data(width: u32, height: u32, channels: u32, data: Box<[u8]>) -> Image {
        Image {
            width,
            height,
            channels,
            data,
        }
    }

    pub fn save_to_file(&self, path: &str) {
        // write raw bitmap RGB format
        let mut file = std::fs::File::create(path).unwrap();

        if let Err(err) = write_bmp_file(self.width, self.height, self.data.as_ref(), path) {
            println!("Failed to save image: {}", err);
        } else {
            println!("Saved image to {}", path);
        }
    }
}

fn write_bmp_file(width: u32, height: u32, buffer: &[u8], path: &str) -> std::io::Result<()> {
    let file_header_size = 14;
    let info_header_size = 40;
    let padding = (4 - (width * 3) % 4) % 4;
    let data_size = (width * 3 + padding) * height;
    let file_size = file_header_size + info_header_size + data_size;

    let mut file = std::fs::File::create(path)?;

    // Write file header
    file.write_all(&[0x42, 0x4D])?;                          // BMP signature
    file.write_all(&(file_size as u32).to_le_bytes())?;      // File size
    file.write_all(&[0x00, 0x00])?;                          // Reserved
    file.write_all(&[0x00, 0x00])?;                          // Reserved
    file.write_all(&((file_header_size + info_header_size) as u32).to_le_bytes())?;  // Offset to pixel data

    // Write info header
    file.write_all(&(info_header_size as u32).to_le_bytes())?;   // Info header size
    file.write_all(&(width as i32).to_le_bytes())?;             // Image width
    file.write_all(&(height as i32).to_le_bytes())?;            // Image height
    file.write_all(&[0x01, 0x00])?;                             // Number of color planes
    file.write_all(&[0x18, 0x00])?;                             // Bits per pixel
    file.write_all(&[0x00, 0x00, 0x00, 0x00])?;                 // Compression method (0 = none)
    file.write_all(&(data_size as u32).to_le_bytes())?;         // Image size
    file.write_all(&[0x13, 0x0B, 0x00, 0x00])?;                 // Pixels per meter (2835 x 2835)
    file.write_all(&[0x13, 0x0B, 0x00, 0x00])?;                 // Pixels per meter (2835 x 2835)
    file.write_all(&[0x00, 0x00, 0x00, 0x00])?;                 // Colors in color table
    file.write_all(&[0x00, 0x00, 0x00, 0x00])?;                 // Important color count

    // Write pixel data
    for y in (0..height).rev() {
        for x in 0..width {
            let offset = ((y * width + x) * 3) as usize;
            file.write_all(&buffer[offset + 2..offset + 3])?;   // Blue channel
            file.write_all(&buffer[offset + 1..offset + 2])?;   // Green channel
            file.write_all(&buffer[offset..offset + 1])?;       // Red channel
        }
        // Write padding bytes
        for _ in 0..padding {
            file.write_all(&[0x00])?;
        }
    }

    Ok(())
}
