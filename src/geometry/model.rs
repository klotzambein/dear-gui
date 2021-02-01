use euclid::Point2D;
use glium::Display;
use itertools::Itertools;

use crate::canvas::CanvasObject;
use crate::geometry::ModelSpace;
use crate::graphics::dyn_vertex_buffer::DynVertexBuffer;
use crate::graphics::primitives::{Color, ColoredLine};

pub struct ModelGeometry {
    line_buffer: DynVertexBuffer<ColoredLine>,
    regions: Vec<Vec<Point2D<f32, ModelSpace>>>,
}

impl ModelGeometry {
    pub fn new(facade: &Display) -> ModelGeometry {
        ModelGeometry {
            line_buffer: DynVertexBuffer::new(facade).unwrap(),
            regions: Vec::new(),
        }
    }

    pub fn add_region(
        &mut self,
        color: Color,
        facade: &Display,
        mut region: Vec<Point2D<f32, ModelSpace>>,
    ) {
        if let Some(fst) = region.first().cloned() {
            region.push(fst);
        }
        self.line_buffer
            .extend_n(
                facade,
                region.len() - 1,
                region.iter().tuple_windows().map(|(a, b)| ColoredLine {
                    start: (*a).into(),
                    end: (*b).into(),
                    color,
                }),
            )
            .unwrap();
        self.regions.push(region);
    }
}

impl CanvasObject for ModelGeometry {
    fn draw<'a>(
        &self,
        ctx: &mut crate::canvas::DrawingContext<'a>,
    ) -> Result<(), crate::canvas::CanvasError> {
        ctx.programs.draw_colored_lines(
            ctx.target,
            self.line_buffer.get(),
            1.,
            ctx.model_transform,
            ctx.view_transform,
        )?;

        Ok(())
    }
}
