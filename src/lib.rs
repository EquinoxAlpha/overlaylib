use std::fmt::Formatter;

use font::Font;
use glium::{
    backend::Facade,
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::unix::WindowBuilderExtUnix,
        ContextBuilder,
    },
    implement_vertex, program, uniform, Display, DrawError, DrawParameters, Surface,
};
use primitives::{line::Line, text::Text};

pub mod font;
pub mod frame;
pub mod math;
pub mod primitives;
pub mod texture;

#[derive(Copy, Clone)]
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

    pub fonts: Vec<Font>,
    pub current_font: usize,
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

        let font = Font::new(
            facade,
            include_bytes!("../assets/fonts/NotoSansMono-Regular.ttf"),
            128,
        );

        Self {
            texture_program,
            shape_program,
            fonts: vec![font],
            current_font: 0,
        }
    }

    pub fn add_font_from_file<F>(&mut self, facade: &F, path: &str, size: u32)
    where
        F: ?Sized + Facade,
    {
        let font_data = std::fs::read(path).unwrap();
        let font = Font::new(facade, &font_data, size);
        self.fonts.push(font);
    }

    pub fn add_font_from_memory<F>(&mut self, facade: &F, data: &[u8], size: u32)
    where
        F: ?Sized + Facade,
    {
        let font = Font::new(facade, data, size);
        self.fonts.push(font);
    }

    pub fn new_frame(&self) -> frame::Frame {
        frame::Frame::new(self)
    }

    pub fn draw<F>(
        &self,
        facade: &F,
        target: &mut glium::Frame,
        draw_data: Vec<&Vec<Vertex>>,
    ) -> Result<(), DrawError>
    where
        F: ?Sized + Facade,
    {
        let vertex_buffer = glium::VertexBuffer::new(facade, &draw_data[1]).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let (width, height) = target.get_dimensions();

        let projection =
            math::Matrix4x4::orthographic(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        let tex = self.fonts[0].get_texture(); // todo: multiple fonts support

        let tex = tex
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
                multisampling: false,
                ..Default::default()
            },
        )?;
        
        vertex_buffer.invalidate();

        let vertex_buffer = glium::VertexBuffer::new(facade, &draw_data[0]).unwrap();

        target.draw(
            &vertex_buffer,
            &indices,
            &self.shape_program,
            &uniform! { projection: projection.data },
            &DrawParameters {
                multisampling: false,
                ..Default::default()
            },
        )?;

        vertex_buffer.invalidate();

        Ok(())
    }
}

fn _main() {
    let event_loop = EventLoop::new();

    let window = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::PhysicalSize::new(800, 600))
        .with_transparent(true)
        .with_x11_window_type(vec![glium::glutin::platform::unix::XWindowType::Dock])
        .with_decorations(false)
        .with_always_on_top(true);

    let context = ContextBuilder::new()
        .with_multisampling(4);

    let display = Display::new(window, context, &event_loop).unwrap();

    let overlay = Overlay::initialize(&display);

    let time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let mut target = display.draw();
                let mut frame = overlay.new_frame();

                frame.add(
                    Text::new("Hello world")
                        .centered(true)
                        .position([400.0, 400.0])
                        .size(32.0),
                );

                frame.add(Line::new().start([1.0, 1.0]).end([100.0, 100.0]));

                target.clear_color(0.2, 0.2, 0.2, 1.0);

                overlay
                    .draw(&display, &mut target, frame.get_draw_data())
                    .unwrap();
                target.finish().unwrap();
            }
            _ => (),
        }
    });
}
