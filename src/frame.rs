use crate::{
    primitives::{text::Text, Primitive, PrimitiveType},
    Overlay, Vertex,
};

pub struct Frame<'a> {
    shape_buffer: Vec<Vertex>,
    text_buffer: Vec<Vertex>,
    overlay: &'a Overlay,
}

impl<'a> Frame<'a> {
    pub fn new(overlay: &'a Overlay) -> Self {
        Self {
            shape_buffer: Vec::new(),
            text_buffer: Vec::new(),
            overlay,
        }
    }

    pub fn clear(&mut self) {
        self.shape_buffer.clear();
        self.text_buffer.clear();
    }

    pub fn add(&mut self, shape: impl Primitive) {
        let shape = Box::new(shape);
        match shape.get_type() {
            PrimitiveType::Text => {
                let mut text: Box<Text> = unsafe { std::mem::transmute(shape) };
                if text.font.is_none() {
                    text.font = Some(&self.overlay.fonts[0]);
                }
                self.text_buffer.extend(text.get_vertices());
            }
            _ => self.shape_buffer.extend(shape.get_vertices()),
        }
    }

    pub fn get_draw_data(&self) -> Vec<&Vec<Vertex>> {
        vec![&self.shape_buffer, &self.text_buffer]
    }
}
