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
}
