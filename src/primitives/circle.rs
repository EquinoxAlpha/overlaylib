use crate::Vertex;

use super::{Outline, Primitive, PrimitiveType, DEFAULT_COLOR};

pub struct Circle {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub radius: f32,
    pub filled: bool,
    pub detail: u32,
    pub border: Option<Outline>,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: DEFAULT_COLOR,
            radius: 0.0,
            filled: false,
            detail: 32,
            border: Some(Outline {
                color: DEFAULT_COLOR,
                thickness: 1.0,
            }),
        }
    }
}

impl Primitive for Circle {
    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::Line
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        let mut buf = Vec::new();

        for i in 0..self.detail {
            let angle = 2.0 * std::f32::consts::PI * (i as f32 / self.detail as f32);
            let x = self.position[0] + self.radius * angle.cos();
            let y = self.position[1] + self.radius * angle.sin();
            let next_angle = 2.0 * std::f32::consts::PI * ((i + 1) as f32 / self.detail as f32);
            let next_x = self.position[0] + self.radius * next_angle.cos();
            let next_y = self.position[1] + self.radius * next_angle.sin();
            if self.filled {
                buf.push(Vertex {
                    position: [x, y],
                    color: self.color,
                    tex_coords: [0.0, 0.0],
                });
                buf.push(Vertex {
                    position: [next_x, next_y],
                    color: self.color,
                    tex_coords: [0.0, 0.0],
                });
                buf.push(Vertex {
                    position: self.position,
                    color: self.color,
                    tex_coords: [0.0, 0.0],
                });
            }
            if let Some(border) = self.border {
                buf.extend(super::line::get_line(
                    [x, y],
                    [next_x, next_y],
                    border.color,
                    border.thickness,
                ));
            }
        }

        buf
    }
}

impl Circle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn position(self, position: impl Into<[f32; 2]>) -> Self {
        Self {
            position: position.into(),
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
        let thickness = thickness.into();
        if self.border.is_none() {
            Self {
                border: Some(Outline {
                    thickness,
                    color: DEFAULT_COLOR,
                }),
                ..self
            }
        } else {
            Self {
                border: Some(Outline {
                    thickness,
                    ..self.border.unwrap()
                }),
                ..self
            }
        }
    }

    pub fn detail(self, detail: impl Into<u32>) -> Self {
        let detail = detail.into();

        Self {
            detail,
            ..self
        }
    }

    pub fn radius(self, radius: impl Into<f32>) -> Self {
        let radius = radius.into();

        Self {
            radius,
            ..self
        }
    }

    pub fn filled(self, filled: bool) -> Self {
        Self {
            filled,
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
