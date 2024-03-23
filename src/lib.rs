use std::{cell::RefCell, collections::HashMap, fmt::Formatter};

use font::Font;
use glium::{
    backend::Facade,
    framebuffer::{DepthRenderBuffer, SimpleFrameBuffer},
    implement_vertex, program, uniform, DrawError, DrawParameters, Surface, Texture2d,
};

pub mod font;
pub mod frame;
pub mod math;
pub mod primitives;
pub mod texture;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl std::fmt::Debug for Vertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vertex")
            .field("position", &self.position)
            .field("color", &self.color)
            .finish()
    }
}

pub type Point = [f32; 2];
pub type Color = [f32; 4];
pub type Rect = [Point; 2];

implement_vertex!(Vertex, position, tex_coords, color);

pub struct Overlay {
    texture_program: glium::Program,
    shape_program: glium::Program,
    pub fonts: HashMap<usize, Font>,
    font_stack: Vec<usize>,

    fxaa_vertex_buffer: glium::VertexBuffer<Vertex>,
    fxaa_index_buffer: glium::IndexBuffer<u16>,
    fxaa_program: glium::Program,

    target_color: RefCell<Option<Texture2d>>,
    target_depth: RefCell<Option<DepthRenderBuffer>>,
}

impl Overlay {
    pub fn initialize<F>(facade: &F) -> Self
    where
        F: ?Sized + Facade,
    {
        let texture_program = program!(facade,
            140 => {
                vertex: "
                #version 140

                in vec2 position;
                in vec2 tex_coords;
                in vec4 color;

                out vec4 v_color;
                out vec2 v_tex_coords;

                uniform mat4 projection;

                void main() {
                    gl_Position = projection * vec4(position, 0.0, 1.0);
                    v_color = color;
                    v_tex_coords = tex_coords;
                }
                ",
                fragment: "
                #version 140

                in vec4 v_color;
                in vec2 v_tex_coords;

                out vec4 color;
                uniform sampler2D font_texture;

                void main() {
                    color = texture(font_texture, v_tex_coords).aaaa * v_color;
                }
                "
            },
        )
        .unwrap();

        let shape_program = program!(facade,
            140 => {
                vertex: "
                #version 140

                in vec2 position;
                in vec2 tex_coords;
                in vec4 color;

                out vec4 v_color;
                out vec2 v_tex_coords;

                uniform mat4 projection;

                void main() {
                    gl_Position = projection * vec4(position, 0.0, 1.0);
                    v_color = color;
                    v_tex_coords = tex_coords;
                }
                ",
                fragment: "
                #version 140

                in vec4 v_color;
                in vec2 v_tex_coords;

                out vec4 color;

                void main() {
                    color = v_color;
                }
                "
            },
        )
        .unwrap();

        let fxaa_vertex_buffer = glium::VertexBuffer::new(
            facade,
            &[
                Vertex {
                    position: [-1.0, -1.0],
                    tex_coords: [0.0, 0.0],
                    color: [1.0, 1.0, 1.0, 1.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    tex_coords: [0.0, 1.0],
                    color: [1.0, 1.0, 1.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    tex_coords: [1.0, 1.0],
                    color: [1.0, 1.0, 1.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                    tex_coords: [1.0, 0.0],
                    color: [1.0, 1.0, 1.0, 1.0],
                },
            ],
        )
        .unwrap();

        let fxaa_index_buffer = glium::IndexBuffer::new(
            facade,
            glium::index::PrimitiveType::TrianglesList,
            &[0, 1, 2, 0, 2, 3],
        )
        .unwrap();

        let fxaa_program = program!(facade,
            140 => {
                vertex: "
                #version 140

                in vec2 position;
                in vec2 tex_coords;

                out vec2 v_tex_coords;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
                ",
                fragment: "
                #version 140

                precision mediump float;

                in vec2 v_tex_coords;

                out vec4 color;

                uniform vec2 resolution;
                uniform sampler2D tex;
                
                #define FXAA_REDUCE_MIN   (1.0/128.0)
                #define FXAA_REDUCE_MUL   (1.0/8.0)
                #define FXAA_SPAN_MAX     8.0

                vec4 fxaa(sampler2D tex, vec2 fragCoord, vec2 resolution) {
                    vec4 color;
                    vec3 rgbNW = texture2D(tex, fragCoord + (vec2(-1.0, -1.0))).xyz;
                    vec3 rgbNE = texture2D(tex, fragCoord + (vec2(1.0, -1.0))).xyz;
                    vec3 rgbSW = texture2D(tex, fragCoord + (vec2(-1.0, 1.0))).xyz;
                    vec3 rgbSE = texture2D(tex, fragCoord + (vec2(1.0, 1.0))).xyz;
                    vec4 texColor = texture2D(tex, fragCoord);
                    vec3 rgbM  = texColor.xyz;
                    vec3 luma = vec3(0.299, 0.587, 0.114);
                    float lumaNW = dot(rgbNW, luma);
                    float lumaNE = dot(rgbNE, luma);
                    float lumaSW = dot(rgbSW, luma);
                    float lumaSE = dot(rgbSE, luma);
                    float lumaM  = dot(rgbM,  luma);
                    float lumaMin = min(lumaM, min(min(lumaNW, lumaNE), min(lumaSW, lumaSE)));
                    float lumaMax = max(lumaM, max(max(lumaNW, lumaNE), max(lumaSW, lumaSE)));

                    mediump vec2 dir;
                    dir.x = -((lumaNW + lumaNE) - (lumaSW + lumaSE));
                    dir.y =  ((lumaNW + lumaSW) - (lumaNE + lumaSE));

                    float dirReduce = max((lumaNW + lumaNE + lumaSW + lumaSE) *
                                                    (0.25 * FXAA_REDUCE_MUL), FXAA_REDUCE_MIN);

                    float rcpDirMin = 1.0 / (min(abs(dir.x), abs(dir.y)) + dirReduce);
                    dir = min(vec2(FXAA_SPAN_MAX, FXAA_SPAN_MAX),
                                        max(vec2(-FXAA_SPAN_MAX, -FXAA_SPAN_MAX),
                                        dir * rcpDirMin));

                    vec3 rgbA = 0.5 * (
                                texture2D(tex, fragCoord + dir * (1.0 / 3.0 - 0.5)).xyz +
                                texture2D(tex, fragCoord + dir * (2.0 / 3.0 - 0.5)).xyz);
                    vec3 rgbB = rgbA * 0.5 + 0.25 * (
                                texture2D(tex, fragCoord + dir * -0.5).xyz +
                                texture2D(tex, fragCoord + dir * 0.5).xyz);

                    float lumaB = dot(rgbB, luma);
                    if ((lumaB < lumaMin) || (lumaB > lumaMax))
                        color = vec4(rgbA, texColor.a);
                    else
                        color = vec4(rgbB, texColor.a);
                    return color;
                }

                void main() {
                    color = fxaa(tex, v_tex_coords, resolution);
                }
                "
            },
        )
        .unwrap();

        let font = Font::new(
            facade,
            include_bytes!("../assets/fonts/NotoSansMono-Regular.ttf"),
            24.0,
        );

        let mut fonts = HashMap::new();
        fonts.insert(0, font);

        Self {
            texture_program,
            shape_program,
            fonts,
            font_stack: vec![0],
            fxaa_vertex_buffer,
            fxaa_index_buffer,
            fxaa_program,
            target_color: RefCell::new(None),
            target_depth: RefCell::new(None),
        }
    }

    /// Adds a font from a file to the overlay.
    ///
    /// The font will be added to the end of the font list.
    ///
    /// # Arguments
    ///
    /// * `facade` - The glium facade.
    /// * `path` - The path to the font file.
    /// * `size` - The font size.
    /// * `id` - The font ID. Used to reference the font later.
    pub fn add_font_from_file<F>(&mut self, facade: &F, path: &str, size: f32, id: usize)
    where
        F: ?Sized + Facade,
    {
        let font_data = std::fs::read(path).unwrap();
        let font = Font::new(facade, &font_data, size);
        self.fonts.insert(id, font);
    }

    /// Adds a font from memory to the overlay.
    ///
    /// The font will be added to the end of the font list.
    ///
    /// # Arguments
    ///
    /// * `facade` - The glium facade.
    /// * `data` - The font data.
    /// * `size` - The font size.
    /// * `id` - The font ID. Used to reference the font later.
    pub fn add_font_from_memory<F>(&mut self, facade: &F, data: &[u8], size: f32, id: usize)
    where
        F: ?Sized + Facade,
    {
        let font = Font::new(facade, data, size);
        self.fonts.insert(id, font);
    }

    /// Creates a new frame.
    ///
    /// The frame is used to draw shapes and text to the overlay.
    pub fn new_frame(&self) -> frame::Frame {
        frame::Frame::new(self)
    }

    /// Pushes a font onto the font stack.
    ///
    /// The font stack is used to determine which font to use when drawing text.
    /// The top font is the one that will be used.
    pub fn push_font(&mut self, font: usize) {
        self.font_stack.push(font);
    }

    /// Pops a font from the font stack.
    ///
    /// Returns the font ID that was popped, or None if the stack was empty.
    pub fn pop_font(&mut self) -> Option<usize> {
        self.font_stack.pop()
    }

    /// Returns the current font.
    pub fn current_font(&self) -> Option<&Font> {
        self.fonts.get(self.font_stack.last()?)
    }

    /// Draws the overlay.
    ///
    /// # Arguments
    ///
    /// * `facade` - The glium facade.
    /// * `target` - The glium frame.
    /// * `draw_data` - The draw data.
    ///
    /// # Returns
    ///
    /// * `Result<(), DrawError>` - The result of the draw operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use overlaylib::Overlay;
    /// # use glium::Display;
    /// # let display = Display::new();
    /// # let overlay = Overlay::initialize(&display);
    /// # let mut target = display.draw();
    /// # let draw_data = vec![];
    /// overlay.draw(&display, &mut target, draw_data);
    /// ```
    pub fn draw<F: Facade, T: Surface>(
        &self,
        facade: &F,
        target: &mut T,
        draw_data: &mut crate::frame::Frame<'_>,
    ) -> Result<(), DrawError> {
        let (width, height) = target.get_dimensions();
        let projection =
            math::Matrix4x4::orthographic(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        for buffer in draw_data.buffers.values_mut() {
            match buffer.texture {
                Some(texture) => {
                    let vertex_buffer = glium::VertexBuffer::new(facade, &buffer.vertices).unwrap();
                    let indices =
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    let tex = texture
                        .sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
                        .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat);

                    target.draw(
                        &vertex_buffer,
                        &indices,
                        &self.texture_program,
                        &uniform! {
                        projection: projection.data,
                        font_texture: tex
                        },
                        &DrawParameters {
                            blend: glium::Blend::alpha_blending(),
                            multisampling: true,
                            ..Default::default()
                        },
                    )?;
                }
                None => {
                    let vertex_buffer = glium::VertexBuffer::new(facade, &buffer.vertices).unwrap();
                    let indices =
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
                    target.draw(
                        &vertex_buffer,
                        &indices,
                        &self.shape_program,
                        &uniform! { projection: projection.data },
                        &DrawParameters {
                            blend: glium::Blend::alpha_blending(),
                            ..Default::default()
                        },
                    )?;
                }
            }
        }

        draw_data.clear();

        Ok(())
    }

    pub fn draw_fxaa<F: Facade, T: Surface>(
        &self,
        facade: &F,
        target: &mut T,
        draw_data: &mut crate::frame::Frame<'_>,
    ) -> Result<(), DrawError> {
        let target_dimensions = target.get_dimensions();
        let color_dimensions = {
            self.target_color
                .borrow()
                .as_ref()
                .map_or((0, 0), |tex| (tex.get_width(), tex.get_height().unwrap()))
        };
        let projection = math::Matrix4x4::orthographic(
            0.0,
            target_dimensions.0 as f32,
            target_dimensions.1 as f32,
            0.0,
            -1.0,
            1.0,
        );

        let depth_dimensions = {
            self.target_depth
                .borrow()
                .as_ref()
                .map_or((0, 0), |tex| tex.get_dimensions())
        };

        let mut target_color = self.target_color.borrow_mut();
        let mut target_depth = self.target_depth.borrow_mut();

        if target_color.is_none() || color_dimensions != target_dimensions {
            *target_color =
                Some(Texture2d::empty(facade, target_dimensions.0, target_dimensions.1).unwrap());
        }

        if target_depth.is_none() || depth_dimensions != target_dimensions {
            *target_depth = Some(
                DepthRenderBuffer::new(
                    facade,
                    glium::texture::DepthFormat::I24,
                    target_dimensions.0,
                    target_dimensions.1,
                )
                .unwrap(),
            );
        }

        let target_depth = target_depth.as_ref().unwrap();
        let target_color = target_color.as_ref().unwrap();

        let mut framebuffer =
            SimpleFrameBuffer::with_depth_buffer(facade, target_color, target_depth).unwrap();
        self.draw(facade, &mut framebuffer, draw_data);

        target.draw(
            &self.fxaa_vertex_buffer,
            &self.fxaa_index_buffer,
            &self.fxaa_program,
            &uniform! {
                tex: &*target_color,
                resolution: [target_dimensions.0 as f32, target_dimensions.1 as f32],
                projection: projection.data
            },
            &DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        )?;

        framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        Ok(())
    }
}
