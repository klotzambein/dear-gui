use glium::program::ProgramChooserCreationError;
use glium::texture::Texture2d;
use glium::vertex::VertexBufferSlice;
use glium::{
    backend::Facade,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
};
use glium::{program, uniform};
use glium::{DrawError, DrawParameters, Program, Surface};

use euclid::Transform2D;

use crate::geometry::{CanvasSpace, ModelSpace, ScreenSpace};
use crate::graphics::primitives::{Color, ColoredLine, Line, LinePoint};

use super::primitives::{ColoredPoint, Sprite};

pub struct Programs {
    pub parameters: DrawParameters<'static>,
    pub line_strip: Program,
    pub line: Program,
    pub colored_point: Program,
    pub colored_line: Program,
    pub sprites: Program,
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
            colored_point: include_shaders!(display, "colored_point", "vgf")?,
            sprites: include_shaders!(display, "sprites", "vgf")?,
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

    pub fn draw_colored_points(
        &self,
        frame: &mut impl Surface,
        vertex_buffer: VertexBufferSlice<ColoredPoint>,
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
            &self.colored_point,
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

    pub fn draw_sprites(
        &self,
        frame: &mut impl Surface,
        vertex_buffer: VertexBufferSlice<Sprite>,
        texture: &Texture2d,
        model_transform: Transform2D<f32, ModelSpace, CanvasSpace>,
        view_transform: Transform2D<f32, CanvasSpace, ScreenSpace>,
    ) -> Result<(), DrawError> {
        let mt: [[f32; 2]; 3] = model_transform.to_arrays();
        let vt: [[f32; 2]; 3] = view_transform.to_arrays();
        frame.draw(
            vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::Points),
            &self.sprites,
            &uniform! {
                sprite_texture: texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::NearestMipmapNearest),
                model_transform: [
                    [mt[0][0], mt[0][1], 0.],
                    [mt[1][0], mt[1][1], 0.],
                    [mt[2][0], mt[2][1], 1.]],
                view_transform: [
                    [vt[0][0], vt[0][1], 0.],
                    [vt[1][0], vt[1][1], 0.],
                    [vt[2][0], vt[2][1], 1.]],
            },
            &self.parameters,
        )
    }
}
