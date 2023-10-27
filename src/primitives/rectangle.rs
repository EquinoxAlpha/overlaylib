use crate::Vertex;

use super::{Outline, Primitive, PrimitiveType, DEFAULT_COLOR};

#[allow(unused)]
pub struct Rectangle {
    color: [f32; 4],
    dimensions: [f32; 2],
    position: [f32; 2],
    border: Option<Outline>,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            dimensions: [0.0, 0.0],
            position: [0.0, 0.0],
            border: None,
        }
    }
}

impl Rectangle {
    pub fn color(self, color: impl Into<[f32; 4]>) -> Self {
        Self {
            color: color.into(),
            ..self
        }
    }

    pub fn dimensions(self, dimensions: impl Into<[f32; 2]>) -> Self {
        Self {
            dimensions: dimensions.into(),
            ..self
        }
    }

    pub fn position(self, position: impl Into<[f32; 2]>) -> Self {
        Self {
            position: position.into(),
            ..self
        }
    }

    pub fn border(self, border: impl Into<Option<Outline>>) -> Self {
        Self {
            border: border.into(),
            ..self
        }
    }
}

impl Primitive for Rectangle {
    fn get_vertices(&self) -> Vec<Vertex> {
        // todo: add border
        vec![
            Vertex {
                position: [self.position[0], self.position[1]],
                color: self.color,
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [self.position[0] + self.dimensions[0], self.position[1]],
                color: self.color,
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [
                    self.position[0] + self.dimensions[0],
                    self.position[1] + self.dimensions[1],
                ],
                color: self.color,
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [self.position[0], self.position[1] + self.dimensions[1]],
                color: self.color,
                tex_coords: [0.0, 1.0],
            },
        ]
    }

    fn get_type(&self) -> super::PrimitiveType {
        PrimitiveType::Rectangle
    }
}
