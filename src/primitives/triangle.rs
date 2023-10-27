use crate::Vertex;

use super::{Primitive, PrimitiveType};

pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Default for Triangle {
    fn default() -> Self {
        Self { vertices: [Default::default(); 3] }
    }
}

impl Primitive for Triangle {
    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::Triangle
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.to_vec()
    }
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self {
            vertices
        }
    }
}