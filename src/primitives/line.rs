use crate::{math, Vertex};

use super::{Primitive, PrimitiveType, DEFAULT_COLOR};

pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub thickness: f32,
    pub color: [f32; 4],
}

impl Default for Line {
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
            thickness: 1.0,
            color: DEFAULT_COLOR,
        }
    }
}

pub(crate) fn get_line(
    start: [f32; 2],
    end: [f32; 2],
    color: [f32; 4],
    thickness: f32,
) -> Vec<Vertex> {

    let delta = [end[0] - start[0], end[1] - start[1]];

    let direction = math::normalize(delta);

    let start_corner1 = [-direction[1], direction[0]];
    let start_corner2 = [direction[1], -direction[0]];

    let end_corner1 = [
        end[0] + start_corner1[0] * thickness,
        end[1] + start_corner1[1] * thickness,
    ];
    let end_corner2 = [
        end[0] + start_corner2[0] * thickness,
        end[1] + start_corner2[1] * thickness,
    ];

    let start_corner1 = [
        start[0] + start_corner1[0] * thickness,
        start[1] + start_corner1[1] * thickness,
    ];
    let start_corner2 = [
        start[0] + start_corner2[0] * thickness,
        start[1] + start_corner2[1] * thickness,
    ];

    // Add the vertices
    vec![
        Vertex {
            position: start_corner1,
            color,
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: end_corner2,
            color,
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: end_corner1,
            color,
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: start_corner2,
            color,
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: end_corner2,
            color,
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: start_corner1,
            color,
            tex_coords: [0.0, 0.0],
        },
    ]
}

impl Primitive for Line {
    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::Line
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        get_line(self.start, self.end, self.color, self.thickness)
    }
}

impl Line {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn start(self, start: impl Into<[f32; 2]>) -> Self {
        Self {
            start: start.into(),
            ..self
        }
    }

    pub fn end(self, end: impl Into<[f32; 2]>) -> Self {
        Self {
            end: end.into(),
            ..self
        }
    }

    pub fn color(self, color: impl Into<[f32; 4]>) -> Self {
        Self {
            color: color.into(),
            ..self
        }
    }

    pub fn thickness(self, thickness: impl Into<f32>) -> Self {
        Self {
            thickness: thickness.into(),
            ..self
        }
    }
}
