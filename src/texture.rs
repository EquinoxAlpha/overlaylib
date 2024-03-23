use glium::{backend::Facade, GlObject, Texture2d};

pub struct Texture2D {
    pub texture: glium::texture::Texture2d,
    pub dimensions: (u32, u32),
}

impl PartialEq for Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.texture.get_id() == other.texture.get_id()
    }
}

#[derive(Debug)]
pub enum TextureError {
    InvalidImage,
    TextureCreationError(glium::texture::TextureCreationError),
}

impl Texture2D {
    pub fn new(texture: Texture2d, dimensions: (u32, u32)) -> Self {
        Self { texture, dimensions }
    }

    /// Loads a texture from a file and returns a reference to the Texture2D.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the texture file.
    ///
    /// # Returns
    /// * `Result<Texture2D, TextureError>` - The result of texture creation.
    pub fn load_from_file<F: Facade>(
        facade: &F,
        path: &str,
    ) -> Result<Texture2D, TextureError> {
        let image = image::open(path)
            .map_err(|_| TextureError::InvalidImage)?
            .to_rgba8();
        let dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), dimensions);
        let texture = glium::texture::Texture2d::new(facade, image)
            .map_err(|e| TextureError::TextureCreationError(e))?;
        Ok(Texture2D {
            texture,
            dimensions,
        })
    }

    /// Loads a texture from a file, reverses it and returns a reference to the Texture2D.
    /// # Arguments
    ///
    /// * `facade` - The facade to create the texture with.
    /// * `path` - The path to the texture file.
    ///
    /// # Returns
    ///
    /// * `Result<Texture2D, TextureError>` - The result of texture creation.
    pub fn load_from_file_reversed<F: Facade>(
        facade: &F,
        path: &str,
    ) -> Result<Texture2D, TextureError> {
        let image = image::open(path)
            .map_err(|_| TextureError::InvalidImage)?
            .to_rgba8();
        let dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        let texture = glium::texture::Texture2d::new(facade, image)
            .map_err(|e| TextureError::TextureCreationError(e))?;
        Ok(Texture2D {
            texture,
            dimensions,
        })
    }

    /// Loads a texture from bytes and returns a reference to the Texture2D.
    ///
    /// # Arguments
    ///
    /// * `data` - The bytes of the texture.
    ///
    /// # Returns
    ///
    /// * `Result<Texture2D, TextureError>` - The result of texture creation.
    pub fn load_from_memory<F: Facade>(
        facade: &F,
        data: &[u8],
    ) -> Result<Texture2D, TextureError> {
        let image = image::load_from_memory(data)
            .map_err(|_| TextureError::InvalidImage)?
            .to_rgba8();
        let dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), dimensions);
        let texture = glium::texture::Texture2d::new(facade, image)
            .map_err(|e| TextureError::TextureCreationError(e))?;
        Ok(Texture2D {
            texture,
            dimensions,
        })
    }

    /// Loads a texture from bytes, reverses it and returns a reference to the Texture2D.
    /// # Arguments
    ///
    /// * `facade` - The facade to create the texture with.
    /// * `data` - The bytes of the texture.
    ///
    /// # Returns
    /// * `Result<Texture2D, TextureError>` - The result of texture creation.
    pub fn load_from_memory_reversed<F: Facade>(
        facade: &F,
        data: &[u8],
    ) -> Result<Texture2D, TextureError> {
        let image = image::load_from_memory(data)
            .map_err(|_| TextureError::InvalidImage)?
            .to_rgba8();
        let dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        let texture = glium::texture::Texture2d::new(facade, image)
            .map_err(|e| TextureError::TextureCreationError(e))?;
        Ok(Texture2D {
            texture,
            dimensions,
        })
    }

    /// Returns a reference to the underlying glium texture.
    pub fn get_gl_texture(&self) -> &Texture2d {
        &self.texture
    }
}
