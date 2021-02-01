use glium::backend::Facade;
use glium::program::ProgramChooserCreationError;
use glium::vertex::VertexBufferSlice;
use glium::{program, uniform};
use glium::{DrawError, DrawParameters, Program, Surface};

use euclid::Transform2D;

use crate::geometry::{CanvasSpace, ModelSpace, ScreenSpace};
use crate::graphics::primitives::{Color, ColoredLine, Line, LinePoint};

pub struct Programs {
    pub parameters: DrawParameters<'static>,
    pub line_strip: Program,
    pub line: Program,
    pub colored_line: Program,
}

macro_rules! include_shaders {
    ($display:expr, $name:expr, "vf") => {
        program!($display,
            330 => {
                vertex: concat!("#version 330\n",
                    include_str!("shaders/prelude.glsl"), "\n",
                    include_str!(concat!("shaders/", $name, "/vert.glsl"))),
                fragment: concat!("#version 330\n",
                    include_str!("shaders/prelude.glsl"), "\n",
                    include_str!(concat!("shaders/", $name, "/frag.glsl"))),
            }
        )
    };
    ($display:expr, $name:expr, "vgf") => {
        program!($display,
            330 => {
                vertex: concat!("#version 330\n",
                    include_str!("shaders/prelude.glsl"), "\n",
                    include_str!(concat!("shaders/", $name, "/vert.glsl"))),
                geometry: concat!("#version 330\n",
                    include_str!("shaders/prelude.glsl"), "\n",
                    include_str!(concat!("shaders/", $name, "/geo.glsl"))),
                fragment: concat!("#version 330\n",
                    include_str!("shaders/prelude.glsl"), "\n",
                    include_str!(concat!("shaders/", $name, "/frag.glsl"))),
            }
        )
    }
}

impl Programs {
    pub fn new(display: &impl Facade) -> Result<Programs, ProgramChooserCreationError> {
        Ok(Programs {
            line_strip: include_shaders!(display, "line_strip", "vgf")?,
            line: include_shaders!(display, "line", "vgf")?,
            colored_line: include_shaders!(display, "colored_line", "vgf")?,
            parameters: DrawParameters {
                blend: glium::Blend {
                    color: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::SourceAlpha,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    alpha: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::SourceAlpha,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
                backface_culling: glium::BackfaceCullingMode::CullingDisabled,
                ..Default::default()
            },
        })
    }

    pub fn draw_line_strips(
        &self,
        frame: &mut impl Surface,
        vertex_buffer: VertexBufferSlice<LinePoint>,
        color: Color,
        width: f32,
        model_transform: Transform2D<f32, ModelSpace, CanvasSpace>,
        view_transform: Transform2D<f32, CanvasSpace, ScreenSpace>,
    ) -> Result<(), DrawError> {
        let (w, h) = frame.get_dimensions();
        let aspect_ratio = w as f32 / h as f32;
        let mt: [[f32; 2]; 3] = model_transform.to_arrays();
        let vt: [[f32; 2]; 3] = view_transform.to_arrays();
        frame.draw(
            vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::LineStrip),
            &self.line_strip,
            &uniform! {
                width: width,
                pixel_width: [2. / w as f32, 2. / h as f32],
                color: color.to_rgb_array(),
                model_transform: [
                    [mt[0][0], mt[0][1], 0.],
                    [mt[1][0], mt[1][1], 0.],
                    [mt[2][0], mt[2][1], 1.]],
                view_transform: [
                    [vt[0][0], vt[0][1], 0.],
                    [vt[1][0], vt[1][1], 0.],
                    [vt[2][0], vt[2][1], 1.]],
                aspect_ratio: aspect_ratio,
            },
            &self.parameters,
        )
    }

    pub fn draw_lines(
        &self,
        frame: &mut impl Surface,
        vertex_buffer: VertexBufferSlice<Line>,
        color: Color,
        width: f32,
        model_transform: Transform2D<f32, ModelSpace, CanvasSpace>,
        view_transform: Transform2D<f32, CanvasSpace, ScreenSpace>,
    ) -> Result<(), DrawError> {
        let (w, h) = frame.get_dimensions();
        let aspect_ratio = w as f32 / h as f32;
        let mt: [[f32; 2]; 3] = model_transform.to_arrays();
        let vt: [[f32; 2]; 3] = view_transform.to_arrays();
        frame.draw(
            vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::Points),
            &self.line,
            &uniform! {
                width: width,
                pixel_width: [2. / w as f32, 2. / h as f32],
                color: color.to_rgb_array(),
                model_transform: [
                    [mt[0][0], mt[0][1], 0.],
                    [mt[1][0], mt[1][1], 0.],
                    [mt[2][0], mt[2][1], 1.]],
                view_transform: [
                    [vt[0][0], vt[0][1], 0.],
                    [vt[1][0], vt[1][1], 0.],
                    [vt[2][0], vt[2][1], 1.]],
                aspect_ratio: aspect_ratio,
            },
            &self.parameters,
        )
    }

    pub fn draw_colored_lines(
        &self,
        frame: &mut impl Surface,
        vertex_buffer: VertexBufferSlice<ColoredLine>,
        width: f32,
        model_transform: Transform2D<f32, ModelSpace, CanvasSpace>,
        view_transform: Transform2D<f32, CanvasSpace, ScreenSpace>,
    ) -> Result<(), DrawError> {
        let (w, h) = frame.get_dimensions();
        let aspect_ratio = w as f32 / h as f32;
        let mt: [[f32; 2]; 3] = model_transform.to_arrays();
        let vt: [[f32; 2]; 3] = view_transform.to_arrays();
        frame.draw(
            vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::Points),
            &self.colored_line,
            &uniform! {
                width: width,
                pixel_width: [2. / w as f32, 2. / h as f32],
                model_transform: [
                    [mt[0][0], mt[0][1], 0.],
                    [mt[1][0], mt[1][1], 0.],
                    [mt[2][0], mt[2][1], 1.]],
                view_transform: [
                    [vt[0][0], vt[0][1], 0.],
                    [vt[1][0], vt[1][1], 0.],
                    [vt[2][0], vt[2][1], 1.]],
                aspect_ratio: aspect_ratio,
            },
            &self.parameters,
        )
    }
}