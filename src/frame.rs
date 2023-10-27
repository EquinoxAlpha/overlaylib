use crate::{
    math,
    primitives::{text::Text, Primitive, PrimitiveType},
    Color, Overlay, Point, Vertex,
};

pub struct Frame<'a> {
    shape_buffer: Vec<Vertex>, // Used by the shape builder functions
    text_buffer: Vec<Vertex>,  // Used by the add_text function
    overlay: &'a Overlay,
}

pub struct Circle {
    pub center: Point,
    pub radius: f32,
    pub color: [f32; 4],
    pub detail: usize,
    pub filled: bool,
    pub thickness: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            radius: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            detail: 64,
            filled: false,
            thickness: 1.0,
        }
    }
}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
    pub color: [f32; 4],
    pub filled: bool,
    pub thickness: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            top_left: [0.0, 0.0],
            bottom_right: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            filled: false,
            thickness: 1.0,
        }
    }
}

impl<'a> Frame<'a> {
    pub fn new(overlay: &'a Overlay) -> Self {
        Self {
            shape_buffer: Vec::new(),
            text_buffer: Vec::new(),
            overlay,
        }
    }

    /// For internal use only. Adds a vertex to the frame.
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.shape_buffer.push(vertex);
    }

    pub fn add_line(
        &mut self,
        start: impl Into<Point>,
        end: impl Into<Point>,
        mut thickness: f32,
        color: impl Into<Color>,
    ) {
        let start = start.into();
        let end = end.into();
        let color = color.into();

        thickness /= 2.0;

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
        self.shape_buffer.push(Vertex {
            position: start_corner1,
            color,
            tex_coords: [0.0, 0.0],
        });
        self.shape_buffer.push(Vertex {
            position: end_corner2,
            color,
            tex_coords: [0.0, 0.0],
        });
        self.shape_buffer.push(Vertex {
            position: end_corner1,
            color,
            tex_coords: [0.0, 0.0],
        });

        self.shape_buffer.push(Vertex {
            position: start_corner2,
            color,
            tex_coords: [0.0, 0.0],
        });
        self.shape_buffer.push(Vertex {
            position: end_corner2,
            color,
            tex_coords: [0.0, 0.0],
        });
        self.shape_buffer.push(Vertex {
            position: start_corner1,
            color,
            tex_coords: [0.0, 0.0],
        });
    }

    pub fn add_rect(&mut self, mut rectangle: Rectangle) {
        rectangle.top_left = [rectangle.top_left[0].round(), rectangle.top_left[1].round()];
        rectangle.bottom_right = [
            rectangle.bottom_right[0].round(),
            rectangle.bottom_right[1].round(),
        ];
        if rectangle.filled {
            self.shape_buffer.push(Vertex {
                position: rectangle.top_left,
                color: rectangle.color,
                tex_coords: [0.0, 0.0],
            });

            self.shape_buffer.push(Vertex {
                position: [rectangle.bottom_right[0], rectangle.top_left[1]],
                color: rectangle.color,
                tex_coords: [0.0, 0.0],
            });

            self.shape_buffer.push(Vertex {
                position: [rectangle.top_left[0], rectangle.bottom_right[1]],
                color: rectangle.color,
                tex_coords: [0.0, 0.0],
            });

            // Bottom right triangle

            self.shape_buffer.push(Vertex {
                position: [rectangle.bottom_right[0], rectangle.top_left[1]],
                color: rectangle.color,
                tex_coords: [0.0, 0.0],
            });

            self.shape_buffer.push(Vertex {
                position: [rectangle.bottom_right[0], rectangle.bottom_right[1]],
                color: rectangle.color,
                tex_coords: [0.0, 0.0],
            });

            self.shape_buffer.push(Vertex {
                position: [rectangle.top_left[0], rectangle.bottom_right[1]],
                color: rectangle.color,
                tex_coords: [0.0, 0.0],
            });
        } else {
            // top line
            self.add_line(
                rectangle.top_left,
                [rectangle.bottom_right[0], rectangle.top_left[1]],
                rectangle.thickness,
                rectangle.color,
            );

            // right line
            self.add_line(
                [
                    rectangle.bottom_right[0],
                    rectangle.top_left[1] - rectangle.thickness / 2.0,
                ],
                [
                    rectangle.bottom_right[0],
                    rectangle.bottom_right[1] + rectangle.thickness / 2.0,
                ],
                rectangle.thickness,
                rectangle.color,
            );

            // bottom line
            self.add_line(
                rectangle.bottom_right,
                [rectangle.top_left[0], rectangle.bottom_right[1]],
                rectangle.thickness,
                rectangle.color,
            );

            // left line)
            self.add_line(
                [
                    rectangle.top_left[0],
                    rectangle.bottom_right[1] + rectangle.thickness / 2.0,
                ],
                [
                    rectangle.top_left[0],
                    rectangle.top_left[1] - rectangle.thickness / 2.0,
                ],
                rectangle.thickness,
                rectangle.color,
            );
        }
    }

    pub fn add_circle(&mut self, circle: Circle) {
        for i in 0..circle.detail {
            let angle = 2.0 * std::f32::consts::PI * (i as f32 / circle.detail as f32);
            let x = circle.center[0] + circle.radius * angle.cos();
            let y = circle.center[1] + circle.radius * angle.sin();
            let next_angle = 2.0 * std::f32::consts::PI * ((i + 1) as f32 / circle.detail as f32);
            let next_x = circle.center[0] + circle.radius * next_angle.cos();
            let next_y = circle.center[1] + circle.radius * next_angle.sin();
            if circle.filled {
                self.add_vertex(Vertex {
                    position: [x, y],
                    color: circle.color,
                    tex_coords: [0.0, 0.0],
                });
                self.add_vertex(Vertex {
                    position: [next_x, next_y],
                    color: circle.color,
                    tex_coords: [0.0, 0.0],
                });
                self.add_vertex(Vertex {
                    position: circle.center,
                    color: circle.color,
                    tex_coords: [0.0, 0.0],
                });
            } else {
                self.add_line([x, y], [next_x, next_y], circle.thickness, circle.color);
            }
        }
    }

    pub fn add_text(
        &mut self,
        position: impl Into<Point>,
        text: impl Into<String>,
        color: impl Into<Color>,
        centered: bool,
        scale: f32,
    ) {
        let text = text.into();
        let position = position.into();
        let mut x = position[0];
        let mut y = position[1];

        let atlas = &self.overlay.fonts[self.overlay.current_font].atlas;
        // size
        let sx = scale / 20.0;
        let sy = scale / 20.0;

        let mut buffer = Vec::with_capacity(text.len() * 6);

        let color = color.into();

        for c in text.chars() {
            let glyph = atlas.get_glyph(c).unwrap();

            let x2 = x + glyph.bitmap_left * sx;
            let y2 = -y + glyph.bitmap_top * sy;
            let w = glyph.bitmap_width * sx;
            let h = glyph.bitmap_height * sy;

            // Advance the cursor to the start of the next character
            x += glyph.advance_x * sx;
            y += glyph.advance_y * sy;

            //println!("w{}, h{}", w, h);

            // Skip glyphs that have no pixels
            if w == 0.0 || h == 0.0 {
                continue;
            }

            buffer.push(Vertex {
                position: [x2, -y2],
                color,
                tex_coords: [glyph.texture_x, 0.0],
            });
            buffer.push(Vertex {
                position: [x2 + w, -y2],
                color,
                tex_coords: [
                    glyph.texture_x + glyph.bitmap_width / atlas.texture_dimensions.0 as f32,
                    0.0,
                ],
            });
            buffer.push(Vertex {
                position: [x2, -y2 + h],
                color,
                tex_coords: [
                    glyph.texture_x,
                    glyph.bitmap_height / atlas.texture_dimensions.1 as f32,
                ],
            });

            buffer.push(Vertex {
                position: [x2 + w, -y2],
                color,
                tex_coords: [
                    glyph.texture_x + glyph.bitmap_width / atlas.texture_dimensions.0 as f32,
                    0.0,
                ],
            });
            buffer.push(Vertex {
                position: [x2, -y2 + h],
                color,
                tex_coords: [
                    glyph.texture_x,
                    glyph.bitmap_height / atlas.texture_dimensions.1 as f32,
                ],
            });
            buffer.push(Vertex {
                position: [x2 + w, -y2 + h],
                color,
                tex_coords: [
                    glyph.texture_x + glyph.bitmap_width / atlas.texture_dimensions.0 as f32,
                    glyph.bitmap_height / atlas.texture_dimensions.1 as f32,
                ],
            });
        }

        if centered {
            let mut min_x = std::f32::MAX;
            let mut min_y = std::f32::MAX;
            let mut max_x = std::f32::MIN;
            let mut max_y = std::f32::MIN;

            for vertex in &buffer {
                min_x = min_x.min(vertex.position[0]);
                min_y = min_y.min(vertex.position[1]);
                max_x = max_x.max(vertex.position[0]);
                max_y = max_y.max(vertex.position[1]);
            }

            let width = max_x - min_x;
            let height = max_y - min_y;

            for vertex in &mut buffer {
                vertex.position[0] -= width / 2.0;
                vertex.position[1] -= height / 2.0;
            }
        }

        //println!("{:?}", buffer);

        self.text_buffer.extend(buffer);
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
