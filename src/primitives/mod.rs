pub mod rectangle;
pub mod text;
pub mod line;
pub mod circle;

pub use text::Text;
pub use line::Line;
pub use rectangle::Rectangle;
pub use circle::Circle;

use crate::Vertex;

pub const DEFAULT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Rectangle,
    Text,
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