use crate::{
    primitives::{text::Text, Primitive, PrimitiveType, Rectangle},
    texture::Texture2D,
    Overlay, Vertex,
};

pub struct TexturedBuffer<'a> {
    pub texture: Option<&'a Texture2D>,
    pub vertices: Vec<Vertex>,
}

impl<'a> TexturedBuffer<'a> {
    pub fn with_texture(texture: &'a Texture2D) -> Self {
        Self {
            texture: Some(texture),
            vertices: Vec::new(),
        }
    }

    pub fn with_texture_and_buffer(texture: &'a Texture2D, vertices: Vec<Vertex>) -> Self {
        Self {
            texture: Some(texture),
            vertices,
        }
    }

    pub fn with_buffer(vertices: Vec<Vertex>) -> Self {
        Self {
            texture: None,
            vertices,
        }
    }

    pub fn new() -> Self {
        Self {
            texture: None,
            vertices: Vec::new(),
        }
    }
}

pub struct Frame<'a> {
    pub buffers: Vec<TexturedBuffer<'a>>,
    pub overlay: &'a Overlay,
}

impl<'a> Frame<'a> {
    pub fn new(overlay: &'a Overlay) -> Self {
        Self {
            buffers: vec![],
            overlay,
        }
    }

    pub fn clear(&mut self) {
        self.buffers.clear();
    }

    fn add_buffer(&mut self, buffer: TexturedBuffer<'a>) {
        if self.buffers.len() == 0 {
            self.buffers.push(buffer);
            return;
        }
        let len = self.buffers.len();
        if self.buffers[len - 1].texture == buffer.texture {
            self.buffers[len - 1].vertices.extend_from_slice(&buffer.vertices);
        } else {
            self.buffers.push(buffer);
        }
    }

    pub fn add(&mut self, shape: impl Primitive) {
        let shape = Box::new(shape);
        match shape.get_type() {
            PrimitiveType::Text => {
                let mut text: Box<Text> = unsafe { std::mem::transmute(shape) }; // a necessary evil, PRs welcome
                if text.font.is_none() {
                    text.font = Some(
                        self.overlay
                            .current_font()
                            .expect("No font on the stack"),
                    );
                }
                let Some(font) = text.font else {return;};
                let buffer = TexturedBuffer::with_texture_and_buffer(
                    font.get_texture(),
                    text.get_vertices(),
                );
                self.add_buffer(buffer);
            }
            PrimitiveType::Rectangle => {
                let rect: Box<Rectangle> = unsafe { std::mem::transmute(shape) }; // a necessary evil, PRs welcome
                let buffer = match rect.texture {
                    Some(texture) => {
                        TexturedBuffer::with_texture_and_buffer(texture, rect.get_vertices())
                    },
                    None => {
                        TexturedBuffer::with_buffer(rect.get_vertices())
                    }
                };
                self.add_buffer(buffer);
            }
            _ => {
                let buffer = TexturedBuffer::with_buffer(shape.get_vertices());
                self.add_buffer(buffer);
            }
        }
    }
}
