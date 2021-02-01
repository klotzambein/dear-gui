use std::time::{Duration, Instant};

use glium::Display;

use crate::canvas::{CanvasError, CanvasObject, DrawingContext};
use crate::geometry::{CanvasSpace, Line};
use crate::graphics::dyn_vertex_buffer::DynVertexBuffer;
use crate::graphics::primitives::{Color, Line as GLLine};

pub struct DebugGeometry {
    line_buffer: DynVertexBuffer<GLLine>,
    lines: Vec<(Option<Instant>, Vec<Line<CanvasSpace>>)>,
}

impl DebugGeometry {
    pub fn new(display: &Display) -> DebugGeometry {
        DebugGeometry {
            line_buffer: DynVertexBuffer::new(display).unwrap(),
            lines: Vec::new(),
        }
    }

    pub fn add_lines(&mut self, line: Vec<Line<CanvasSpace>>, duration: Option<f32>) {
        let deadline = duration.map(|dur| Instant::now() + Duration::from_secs_f32(dur));
        self.lines.push((deadline, line));
    }

    pub fn update(&mut self, display: &Display) {
        let len = self.lines.iter().map(|(_, ls)| ls.len()).sum();
        self.line_buffer.clear();
        self.line_buffer
            .extend_n(
                display,
                len,
                self.lines
                    .iter()
                    .map(|(_, ls)| ls)
                    .flatten()
                    .cloned()
                    .map(GLLine::from_line),
            )
            .unwrap();

        let now = Instant::now();
        self.lines
            .retain(|(d, _)| d.map(|d| d > now).unwrap_or(false));
    }
}

impl CanvasObject for DebugGeometry {
    fn draw<'a>(&self, ctx: &mut DrawingContext<'a>) -> Result<(), CanvasError> {
        ctx.programs.draw_lines(
            ctx.target,
            self.line_buffer.get(),
            Color::BLUE,
            1.,
            ctx.model_transform,
            ctx.view_transform,
        )?;

        Ok(())
    }
}
