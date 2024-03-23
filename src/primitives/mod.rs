pub mod rectangle;
pub mod text;
pub mod line;
pub mod circle;
pub mod triangle;

pub use text::Text;
pub use line::Line;
pub use rectangle::Rectangle;
pub use circle::Circle;
pub use triangle::Triangle;

use crate::Vertex;

pub const DEFAULT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Rectangle,
    Text,
    Circle,
    Triangle,
    Line,
}

pub trait Primitive {
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_type(&self) -> PrimitiveType;
}

#[derive(Clone, Copy)]
pub struct Outline {
    pub thickness: f32,
    pub color: [f32; 4],
}

impl Outline {
    pub fn new() -> Self {
        Self {
            thickness: 1.0,
            color: [0.0, 0.0, 0.0, 1.0]
        }
    }

    pub fn thickness(self, thickness: f32) -> Self {
        Self {
            thickness,
            ..self
        }
    }

    pub fn color(self, color: [f32; 4]) -> Self {
        Self {
            color,
            ..self
        }
    }
}