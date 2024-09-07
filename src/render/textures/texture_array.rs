use gl::types::{GLenum, GLint, GLsizei, GLvoid};

pub struct TextureArray {
    pub id: u32,
    pub layers: usize,
}

impl TextureArray {

    pub fn create(paths: Vec<&str>, width: usize, height: usize) -> TextureArray {
        let layers = paths.len();
        let mut texture_id: u32 = 0;

        unsafe {
            // Generate and bind the texture array
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, texture_id);

            // Allocate storage for the texture array
            gl::TexImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA8 as GLint,
                width as GLsizei,
                height as GLsizei,
                layers as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            // Load each texture into the texture array
            for (i, path) in paths.iter().enumerate() {
                let img = image::open(path).expect("Failed to load texture");
                let data = img.raw_pixels();

                gl::TexSubImage3D(
                    gl::TEXTURE_2D_ARRAY,
                    0,
                    0,
                    0,
                    i as GLint,
                    width as GLsizei,
                    height as GLsizei,
                    1,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const GLvoid,
                );
            }

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

            // Unbind the texture array
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, 0);
        }

        TextureArray {
            id: texture_id,
            layers,
        }
    }

    pub fn bind(&self, texture_unit: GLenum) {
        unsafe {
            gl::ActiveTexture(texture_unit);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, 0)
        }
    }
}