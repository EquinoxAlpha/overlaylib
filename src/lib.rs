#![feature(anonymous_lifetime_in_impl_trait)]

use std::{collections::HashMap, fmt::Formatter};

use font::Font;
use glium::{
    backend::Facade, implement_vertex, program, uniform, DrawError, DrawParameters, Surface,
};
use texture::Texture2D;

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
    pub textures: Vec<Texture2D>,
    font_stack: Vec<usize>,
}

#[derive(Debug)]
pub enum OverlayError {
    BufferCreationError,
    TextureCreationError,
    ShaderCompilationError,
    FileNotFound,
    GliumError(DrawError),
}

impl Overlay {
    pub fn initialize<F>(facade: &F) -> Result<Self, OverlayError>
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
                uniform sampler2D tex;

                void main() {
                    color = texture(tex, v_tex_coords).rgba * v_color;
                }
                "
            },
        )
        .map_err(|_| OverlayError::ShaderCompilationError)?;

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
        .map_err(|_| OverlayError::ShaderCompilationError)?;

        let font = Font::new(
            facade,
            include_bytes!("../assets/fonts/NotoSansMono-Regular.ttf"),
            24.0,
        );

        let mut fonts = HashMap::new();
        fonts.insert(0, font);

        Ok(Self {
            texture_program,
            shape_program,
            fonts,
            font_stack: vec![0],
            textures: vec![],
        })
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
    pub fn add_font_from_file<F>(
        &mut self,
        facade: &F,
        path: &str,
        size: f32,
        id: usize,
    ) -> Result<(), OverlayError>
    where
        F: ?Sized + Facade,
    {
        let font_data = std::fs::read(path).map_err(|_| OverlayError::FileNotFound)?;
        let font = Font::new(facade, &font_data, size);
        self.fonts.insert(id, font);
        Ok(())
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
    /// The top font is the one that will be used. It is assumed that the font ID is valid and its corresponding font was loaded in.
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
    /// 
    /// Returns None if no font is present.
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
    ) -> Result<(), OverlayError> {
        let (width, height) = target.get_dimensions();
        let projection =
            math::Matrix4x4::orthographic(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        for buffer in &mut draw_data.buffers {
            match buffer.texture {
                Some(texture) => {
                    let vertex_buffer = glium::VertexBuffer::new(facade, &buffer.vertices)
                        .map_err(|_| OverlayError::BufferCreationError)?;

                    let tex = texture
                        .get_gl_texture()
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
                            tex: tex,
                        },
                        &DrawParameters {
                            blend: glium::Blend::alpha_blending(),
                            multisampling: true,
                            ..Default::default()
                        },
                    ).map_err(|e| OverlayError::GliumError(e))?;
                }
                None => {
                    let vertex_buffer = glium::VertexBuffer::new(facade, &buffer.vertices)
                        .map_err(|_| OverlayError::BufferCreationError)?;
                    let indices =
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
                    target.draw(
                        &vertex_buffer,
                        &indices,
                        &self.shape_program,
                        &uniform! { projection: projection.data },
                        &DrawParameters {
                            blend: glium::Blend::alpha_blending(),
                            multisampling: true,
                            ..Default::default()
                        },
                    ).map_err(|e| OverlayError::GliumError(e))?;
                }
            }
        }

        draw_data.clear();

        Ok(())
    }
}
