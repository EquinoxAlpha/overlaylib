use std::collections::HashMap;

use gl::types::GLuint;
use glium::{GlObject, Texture2d};

use crate::{
    primitives::{text::Text, Primitive, PrimitiveType},
    Overlay, Vertex,
};

pub struct TexturedBuffer<'a> {
    pub texture: Option<&'a Texture2d>,
    pub vertices: Vec<Vertex>,
}

impl<'a> TexturedBuffer<'a> {
    pub fn with_texture(texture: &'a Texture2d) -> Self {
        Self {
            texture: Some(texture),
            vertices: Vec::new(),
        }
    }

    pub fn new() -> Self {
        Self {
            texture: None,
            vertices: Vec::new(),
        }
    }
}

pub const UNTEXTURED_BUFFER_ID: GLuint = 0;

pub struct Frame<'a> {
    // shape_buffer: Vec<Vertex>,
    // text_buffer: Vec<Vertex>,
    pub buffers: HashMap<GLuint, TexturedBuffer<'a>>,
    pub overlay: &'a Overlay,
}

impl<'a> Frame<'a> {
    pub fn new(overlay: &'a Overlay) -> Self {
        Self {
            buffers: HashMap::new(),
            overlay,
        }
    }

    pub fn clear(&mut self) {
        for buffer in self.buffers.values_mut() {
            buffer.vertices.clear();
        }
        self.buffers.clear();
    }

    pub fn add(&mut self, shape: impl Primitive) {
        let shape = Box::new(shape);
        match shape.get_type() {
            PrimitiveType::Text => {
                let mut text: Box<Text> = unsafe { std::mem::transmute(shape) };
                if text.font.is_none() {
                    text.font = Some(self.overlay.current_font().expect("No font on the stack"));
                }
                self.buffers
                    .entry(text.font.unwrap().get_texture().get_id())
                    .or_insert_with(|| {
                        TexturedBuffer::with_texture(text.font.unwrap().get_texture())
                    })
                    .vertices
                    .extend(text.get_vertices());
            }
            _ => {
                self.buffers
                    .entry(UNTEXTURED_BUFFER_ID)
                    .or_insert_with(|| TexturedBuffer::new())
                    .vertices
                    .extend(shape.get_vertices());
            }
        }
    }
}
