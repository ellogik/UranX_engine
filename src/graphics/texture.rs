use image;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Texture {
    pub texture_id: u32,
    pub texture_type: String,
}

impl Texture {
    pub fn new(texture_type: String) -> Self {
        let mut texture_id = 1;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
        }

        Texture {
            texture_id,
            texture_type,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }

    pub fn load(texture_path: &str, texture_type: String) -> Self {
        let texture = Texture::new(texture_type);
        texture.bind();

        let img = image::open(&Path::new(texture_path)).expect("Failed to load texture");
        let img = img.flipv();

        let data = img.to_rgb8();
        let width = img.width();
        let height = img.height();

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        texture
    }
}
