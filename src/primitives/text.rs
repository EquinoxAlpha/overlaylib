use crate::{font::Font, Vertex};

use super::{Outline, Primitive, PrimitiveType, DEFAULT_COLOR};

#[derive(Clone)]
pub struct Text<'a> {
    pub text: String,
    pub text_size: f32,
    pub position: [f32; 2],
    pub font: Option<&'a Font>,
    pub color: [f32; 4],
    pub shadow: Option<Outline>,
    pub offset: [f32; 2],
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Self {
            text: Default::default(),
            text_size: 12.0,
            position: Default::default(),
            font: Default::default(),
            color: DEFAULT_COLOR,
            shadow: Default::default(),
            offset: Default::default(),
        }
    }
}

impl<'a> Text<'a> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn color(self, color: [f32; 4]) -> Self {
        Self { color, ..self }
    }

    pub fn size(self, text_size: f32) -> Self {
        Self { text_size, ..self }
    }

    pub fn position(self, position: [f32; 2]) -> Self {
        Self { position, ..self }
    }

    pub fn font(self, font: &'a Font) -> Self {
        Self {
            font: Some(font),
            ..self
        }
    }

    pub fn centered(self, centered: bool) -> Self {
        if centered {
            return self.offset([0.5, 0.5]);
        }
        self
    }

    pub fn offset(self, offset: [f32; 2]) -> Self {
        Self { offset, ..self }
    }
}

pub fn calc_text_size(text: impl Into<String>, font: &Font, text_size: f32) -> [f32; 2] {
    let mut x = 0.0;
    let mut y = 0.0;

    let atlas = &font.atlas;

    let mut min_x = std::f32::MAX;
    let mut min_y = std::f32::MAX;
    let mut max_x = std::f32::MIN;
    let mut max_y = std::f32::MIN;

    for c in text.into().chars() {
        let glyph = atlas.get_glyph(c).unwrap();

        let scale = atlas.font_size / text_size;
        let x2 = x + glyph.bitmap_left * scale;
        let y2 = -y + glyph.bitmap_top * scale;
        let w = glyph.bitmap_width * scale;
        let h = glyph.bitmap_height * scale;

        x += glyph.advance_x * scale;
        y += glyph.advance_y * scale;

        if w == 0.0 || h == 0.0 {
            continue;
        }

        let p1 = [x2, -y2];
        let p2 = [x2 + w, -y2 + h];

        min_x = min_x.min(p1[0]);
        min_y = min_y.min(p1[1]);
        max_x = max_x.max(p2[0]);
        max_y = max_y.max(p2[1]);
    }

    let width = max_x - min_x;
    let height = max_y - min_y;

    [width, height]
}

impl<'a> Primitive for Text<'a> {
    fn get_vertices(&self) -> Vec<Vertex> {
        let text = &self.text;
        let position = self.position;
        let mut x = position[0];
        let mut y = position[1];

        let atlas = &self.font.unwrap().atlas;
        let mut buffer = Vec::with_capacity(text.len() * 6);

        let color = self.color;

        for c in text.chars() {
            let glyph = atlas.get_glyph(c).unwrap();

            let scale = self.text_size / atlas.font_size;
            let x2 = x + glyph.bitmap_left * scale;
            let y2 = -y + glyph.bitmap_top * scale;
            let w = glyph.bitmap_width * scale;
            let h = glyph.bitmap_height * scale;

            x += glyph.advance_x * scale;
            y += glyph.advance_y * scale;

            if w == 0.0 || h == 0.0 {
                continue;
            }

            buffer.push(Vertex {
                position: [x2, -y2],
                color,
                tex_coords: [glyph.texture_x, 0.0],
            });
            buffer.push(Vertex {
                position: [x2 + w, -y2],
                color,
                tex_coords: [
                    glyph.texture_x + glyph.bitmap_width / atlas.texture_dimensions.0 as f32,
                    0.0,
                ],
            });
            buffer.push(Vertex {
                position: [x2, -y2 + h],
                color,
                tex_coords: [
                    glyph.texture_x,
                    glyph.bitmap_height / atlas.texture_dimensions.1 as f32,
                ],
            });

            buffer.push(Vertex {
                position: [x2 + w, -y2],
                color,
                tex_coords: [
                    glyph.texture_x + glyph.bitmap_width / atlas.texture_dimensions.0 as f32,
                    0.0,
                ],
            });
            buffer.push(Vertex {
                position: [x2, -y2 + h],
                color,
                tex_coords: [
                    glyph.texture_x,
                    glyph.bitmap_height / atlas.texture_dimensions.1 as f32,
                ],
            });
            buffer.push(Vertex {
                position: [x2 + w, -y2 + h],
                color,
                tex_coords: [
                    glyph.texture_x + glyph.bitmap_width / atlas.texture_dimensions.0 as f32,
                    glyph.bitmap_height / atlas.texture_dimensions.1 as f32,
                ],
            });
        }

        let mut min_x = std::f32::MAX;
        let mut min_y = std::f32::MAX;
        let mut max_x = std::f32::MIN;
        let mut max_y = std::f32::MIN;

        for vertex in &buffer {
            min_x = min_x.min(vertex.position[0]);
            min_y = min_y.min(vertex.position[1]);
            max_x = max_x.max(vertex.position[0]);
            max_y = max_y.max(vertex.position[1]);
        }

        let width = max_x - min_x;
        let height = max_y - min_y;

        for vertex in &mut buffer {
            //println!("vp0: {:.1?}, vp1: {:.1?}", vertex.position[0], vertex.position[1]);
            vertex.position[0] -= width * self.offset[0];
            vertex.position[1] -= height * (self.offset[1] - 1.0);
        }

        buffer
    }

    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::Text
    }
}
