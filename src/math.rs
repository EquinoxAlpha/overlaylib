#![allow(unused)]

use crate::{Point};

pub fn magnitude(point: Point) -> f32 {
    (point[0].powi(2) + point[1].powi(2)).sqrt()
}

pub fn normalize(point: Point) -> Point {
    let magnitude = magnitude(point);
    [point[0] / magnitude, point[1] / magnitude]
}


use std::{
    f32::consts::PI,
    ops::{Mul, MulAssign},
};

use glium::uniforms::{AsUniformValue, UniformValue};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}

pub fn vec3_mag_sqr(a: [f32; 3]) -> f32 {
    a[0] * a[0] + a[1] * a[1] + a[2] * a[2]
}

pub fn vec3_mag(a: [f32; 3]) -> f32 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

pub fn vec3_div(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[0] / b[0],
        a[1] / b[1],
        a[2] / b[2],
    ]
}

#[allow(unused)]
pub fn vec3_div_scalar(a: [f32; 3], b: f32) -> [f32; 3] {
    [
        a[0] / b,
        a[1] / b,
        a[2] / b,
    ]
}

pub fn vec3_norm(a: [f32; 3]) -> [f32; 3] {
    vec3_div_scalar(a, vec3_mag(a))
}


pub fn rotation_matrix(angle: f32, axis: [f32; 3]) -> [[f32; 3]; 3] {
    let axis = vec3_norm(axis);
    let (sin_theta, cos_theta) = angle.sin_cos();
    let [x, y, z] = axis;

    let rot = [
        [cos_theta+x*x*(1.0-cos_theta), x*y*(1.0-cos_theta)-z*sin_theta, x*z*(1.0-cos_theta)+y*sin_theta],
        [y*x*(1.0-cos_theta)+z*sin_theta, cos_theta+y*y*(1.0-cos_theta), y*z*(1.0-cos_theta)-x*sin_theta],
        [z*x*(1.0-cos_theta)-y*sin_theta, z*y*(1.0-cos_theta)+x*sin_theta, cos_theta+z*z*(1.0-cos_theta)]
    ];
    rot
}

fn extend_matrix(matrix: [[f32; 3]; 3]) -> [[f32; 4]; 4] {
    let mut extended_matrix = [[0.0; 4]; 4];

    for i in 0..3 {
        for j in 0..3 {
            extended_matrix[i][j] = matrix[i][j];
        }
    }

    extended_matrix[3][3] = 1.0;

    extended_matrix
}

fn matrix_mult_4x4_3x3(a: [[f32; 4]; 4], b: [[f32; 3]; 3]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..3 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}

impl From<[[f32; 4]; 4]> for Matrix4x4 {
    fn from(value: [[f32; 4]; 4]) -> Self {
        Matrix4x4 { data: value }
    }
}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {
        Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }
    
    pub fn rotate(angle: f32, axis: [f32; 3]) -> Matrix4x4 {
        let axis = vec3_norm(axis);
        let (sin_theta, cos_theta) = angle.sin_cos();
        let [x, y, z] = axis;

        let op = [
            [cos_theta+x*x*(1.0-cos_theta), x*y*(1.0-cos_theta)-z*sin_theta, x*z*(1.0-cos_theta)+y*sin_theta, 0.0],
            [y*x*(1.0-cos_theta)+z*sin_theta, cos_theta+y*y*(1.0-cos_theta), y*z*(1.0-cos_theta)-x*sin_theta, 0.0],
            [z*x*(1.0-cos_theta)-y*sin_theta, z*y*(1.0-cos_theta)+x*sin_theta, cos_theta+z*z*(1.0-cos_theta), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Self { data: op }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new()
    }

    pub fn orthographic(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Matrix4x4 {
        let tx = -(right + left) / (right - left);
        let ty = -(top + bottom) / (top - bottom);
        let tz = -(far + near) / (far - near);

        Matrix4x4 {
            data: [
                [2.0 / (right - left), 0.0, 0.0, 0.0],
                [0.0, 2.0 / (top - bottom), 0.0, 0.0],
                [0.0, 0.0, -2.0 / (far - near), 0.0],
                [tx, ty, tz, 1.0]
            ]
        }
    }

    pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let fov = PI / fov;
        let f = (fov / 2.0).tan();

        Self {
            data: [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (far + near) / (far - near), 1.0],
                [0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0],
            ],
        }
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][0] * rhs.data[0][j]
                    + self.data[i][1] * rhs.data[1][j]
                    + self.data[i][2] * rhs.data[2][j]
                    + self.data[i][3] * rhs.data[3][j];
            }
        }
        result
    }
}

impl MulAssign<Matrix4x4> for Matrix4x4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl AsUniformValue for Matrix4x4 {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::Mat4(self.data)
    }
}
