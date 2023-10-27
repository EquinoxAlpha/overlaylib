use freetype_sys::{
    FT_Done_Face, FT_GlyphSlot, FT_Init_FreeType, FT_Library, FT_Load_Char, FT_New_Memory_Face,
    FT_Set_Char_Size, FT_LOAD_RENDER,
};
use glium::{backend::Facade, texture::RawImage2d};

pub struct Glyph {
    pub advance_x: f32,
    pub advance_y: f32,
    pub bitmap_width: f32,
    pub bitmap_height: f32,
    pub bitmap_left: f32,
    pub bitmap_top: f32,
    pub texture_x: f32,
}

pub struct Font {
    pub atlas: FontAtlas,
    pub name: String,
}

impl Font {
    pub fn new<F>(facade: &F, font_data: &[u8], font_size: u32) -> Self
    where
        F: ?Sized + Facade,
    {
        let name = "memory".to_string();

        Self {
            atlas: FontAtlas::new(facade, font_data, font_size),
            name,
        }
    }

    pub fn get_glyph(&self, c: char) -> Option<&Glyph> {
        self.atlas.get_glyph(c)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_texture(&self) -> &glium::texture::Texture2d {
        &self.atlas.texture
    }
}

pub struct FontAtlas {
    pub texture: glium::texture::Texture2d,
    pub texture_dimensions: (u32, u32),
    pub glyph_height: f32,
    pub font_size: f32,
    pub glyphs: Vec<Glyph>,
}

static mut LIBRARY: Option<FT_Library> = None;

impl FontAtlas {
    pub fn new<F>(facade: &F, font_data: &[u8], font_size: u32) -> Self
    where
        F: ?Sized + Facade,
    {
        unsafe {
            if LIBRARY.is_none() {
                let mut library = std::ptr::null_mut();
                FT_Init_FreeType(&mut library);
                LIBRARY = Some(library);
            }
        }

        let library = unsafe { LIBRARY.unwrap() };

        let face = unsafe {
            let mut face = std::ptr::null_mut();
            FT_New_Memory_Face(
                library,
                font_data.as_ptr(),
                font_data.len() as i64,
                0,
                &mut face,
            );
            face
        };
        let char_height = 64.0;
        unsafe {
            FT_Set_Char_Size(face, 0, 16 * char_height as i64, font_size, font_size);
        }

        let glyph: FT_GlyphSlot = unsafe { (*face).glyph };
        let mut w = 0;
        let mut h = 0;

        for i in 0..128 {
            unsafe {
                if FT_Load_Char(face, i as u64, FT_LOAD_RENDER) != 0 {
                    println!("Failed to load char {}", i);
                }

                w += (*glyph).bitmap.width + 1;
                h = h.max((*glyph).bitmap.rows);
            }
        }
        let mut image = vec![0u8; (w * h) as usize];

        let mut x = 0;

        let mut glyphs = Vec::with_capacity(128);

        for i in 0..128 {
            unsafe {
                if FT_Load_Char(face, i as u64, FT_LOAD_RENDER) != 0 {
                    println!("Failed to load char {}", i);
                }

                let bitmap = &(*glyph).bitmap;

                for y in 0..bitmap.rows {
                    let src = bitmap.buffer.offset((y * bitmap.pitch) as isize);
                    let dst = image.as_mut_ptr().offset((x + y * w) as isize);
                    std::ptr::copy_nonoverlapping(src, dst, bitmap.width as usize);
                }

                glyphs.push(Glyph {
                    advance_x: (*glyph).advance.x as f32 / 64.0,
                    advance_y: (*glyph).advance.y as f32 / 64.0,
                    bitmap_width: bitmap.width as f32,
                    bitmap_height: bitmap.rows as f32,
                    bitmap_left: (*glyph).bitmap_left as f32,
                    bitmap_top: (*glyph).bitmap_top as f32,
                    texture_x: x as f32 / w as f32,
                });

                x += bitmap.width + 1;
            }
        }

        // translate image to rgba (u8, u8, u8, u8)

        let image = image
            .chunks_exact(1)
            .map(|chunk| {
                [
                    *chunk.first().unwrap(),
                    *chunk.first().unwrap(),
                    *chunk.first().unwrap(),
                    *chunk.first().unwrap(),
                ]
            })
            .flatten()
            .collect::<Vec<_>>();

        let image = RawImage2d::from_raw_rgba(image, (w as u32, h as u32));

        let texture = glium::texture::Texture2d::new(facade, image).unwrap();

        unsafe { FT_Done_Face(face) };

        Self {
            texture,
            texture_dimensions: (w as u32, h as u32),
            glyph_height: char_height,
            font_size: font_size as f32,
            glyphs,
        }
    }

    pub fn get_glyph(&self, c: char) -> Option<&Glyph> {
        let index = c as usize;

        if index >= 128 {
            return None;
        }

        Some(&self.glyphs[index])
    }
}
