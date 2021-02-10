use std::any::Any;
use std::cell::RefCell;

use euclid::{Point2D, Size2D, Transform2D, Vector2D};
use glium::glutin::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use glium::{Display, DrawError, Frame};
use log::debug;

use crate::event_handling::{EventHandler, FnEventHandler};
use crate::geometry::{CanvasSpace, ModelSpace, PixelSpace, ScreenSpace};

use crate::graphics::programs::Programs;
// use crate::state::{CommandRequest, State};

#[derive(Debug, Clone)]
pub enum CanvasError {
    DrawError(DrawError),
    InvalidGenericType,
}

impl From<DrawError> for CanvasError {
    fn from(err: DrawError) -> Self {
        CanvasError::DrawError(err)
    }
}

pub trait CanvasObject: Any + CanvasObjectExt {
    fn draw<'a>(&self, ctx: &mut DrawingContext<'a>) -> Result<(), CanvasError>;
}

pub trait CanvasObjectExt: Any {
    // requires same traits as Foo
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> CanvasObjectExt for T
where
    T: CanvasObject + Sized,
{
    // but it's only implemented if also Sized
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct InputState {
    mouse_pos: Point2D<f32, PixelSpace>,
    left_pressed_start: Option<Point2D<f32, PixelSpace>>,
}

pub struct Canvas {
    input: InputState,
    zoom: f32,
    translation: Vector2D<f32, CanvasSpace>,
    preview_translation: Option<Vector2D<f32, CanvasSpace>>,
    dimensions: Size2D<u32, PixelSpace>,
    programs: Programs,
    click_handler: Box<dyn FnMut(Point2D<f32, CanvasSpace>)>,
}

impl Canvas {
    pub fn new(display: &Display) -> Canvas {
        let programs = Programs::new(display).unwrap();
        Canvas {
            input: InputState {
                mouse_pos: Point2D::new(0., 0.),
                left_pressed_start: None,
            },
            zoom: 1.0,
            translation: Vector2D::new(0., 0.),
            preview_translation: None,
            dimensions: display.get_framebuffer_dimensions().into(),
            programs,
            click_handler: Box::new(|_| ()),
        }
    }

    pub fn pixel_transform(&self) -> Transform2D<f32, PixelSpace, ScreenSpace> {
        let w = self.dimensions.width as f32 / 2.;
        let h = self.dimensions.height as f32 / 2.;
        Transform2D::translation(-w, -h).then_scale(1. / w, -1. / h)
    }

    pub fn view_transform(&self) -> Transform2D<f32, CanvasSpace, ScreenSpace> {
        let tx = self.preview_translation.unwrap_or(self.translation);
        let zm = self.zoom;

        Transform2D::translation(tx.x, tx.y).then_scale(
            zm / self.dimensions.width as f32,
            zm / self.dimensions.height as f32,
        )
    }

    pub fn draw<T: Any + 'static>(
        &self,
        target: &mut Frame,
        obj: &dyn CanvasObject,
        generic: &T,
    ) -> Result<(), CanvasError> {
        let view_transform = self.view_transform();

        let mut ctx = DrawingContext {
            generic,
            programs: &self.programs,
            target,
            view_transform,
            model_transform: Transform2D::identity(),
            dimensions: self.dimensions.into(),
        };

        obj.draw(&mut ctx)
    }

    pub fn input(&mut self) -> CanvasInput<'_> {
        CanvasInput(RefCell::new(self))
    }

    pub fn set_click_handler(&mut self, handler: Box<dyn FnMut(Point2D<f32, CanvasSpace>)>) {
        self.click_handler = handler;
    }
}

/// RefCell wrapper of canvas. This is necessary because we need to have mutable
/// access to canvas while analyzing the input from two functions. Once for
/// captured input and once for usual input.
pub struct CanvasInput<'a>(RefCell<&'a mut Canvas>);
impl<'a> CanvasInput<'a> {
    pub fn capture(&'a self) -> impl EventHandler + 'a {
        FnEventHandler(move |event| {
            let event = if let Event::WindowEvent { event, .. } = event {
                event
            } else {
                return false;
            };

            let mut canvas = self.0.borrow_mut();

            if canvas.input.left_pressed_start.is_none() {
                return false;
            }

            match event {
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                    ..
                } => {
                    if let Some(start) = canvas.input.left_pressed_start {
                        let delta = canvas.input.mouse_pos - start;
                        if delta.length() < 3. {
                            let screen = canvas.pixel_transform().transform_point(start);
                            let click_pos = canvas
                                .view_transform()
                                .inverse()
                                .unwrap()
                                .transform_point(screen);
                            debug!("Clicked at: {:?}", click_pos);
                            (canvas.click_handler)(click_pos);
                        } else {
                            let screen = canvas.pixel_transform().transform_vector(delta);
                            let model = canvas
                                .view_transform()
                                .inverse()
                                .unwrap()
                                .transform_vector(screen);
                            canvas.translation += model;
                        }
                    }
                    canvas.input.left_pressed_start = None;
                    canvas.preview_translation = None;
                    // Don't capture this event so Imgui knows the LMB is released.
                    false
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let position = Point2D::new(position.x as f32, position.y as f32);
                    canvas.input.mouse_pos = position;
                    if let Some(start) = canvas.input.left_pressed_start {
                        let delta = position - start;
                        canvas.preview_translation = if delta.length() < 3. {
                            None
                        } else {
                            let screen = canvas.pixel_transform().transform_vector(delta);
                            let model = canvas
                                .view_transform()
                                .inverse()
                                .unwrap()
                                .transform_vector(screen);
                            Some(canvas.translation + model)
                        };
                    }

                    true
                }
                _ => false,
            }
        })
    }
    pub fn usual(&'a self) -> impl EventHandler + 'a {
        FnEventHandler(move |event| {
            let event = if let Event::WindowEvent { event, .. } = event {
                event
            } else {
                return false;
            };

            let mut canvas = self.0.borrow_mut();

            match event {
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_x, y),
                    ..
                } => {
                    canvas.zoom *= 1. + (y / 10.);
                    true
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                } => {
                    canvas.input.left_pressed_start = Some(canvas.input.mouse_pos);
                    true
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let position = Point2D::new(position.x as f32, position.y as f32);
                    canvas.input.mouse_pos = position;
                    true
                }
                WindowEvent::Resized(size) => {
                    canvas.dimensions = Size2D::new(size.width, size.height);
                    false
                }
                _ => false,
            }
        })
    }
}

pub struct DrawingContext<'a> {
    pub programs: &'a Programs,
    pub target: &'a mut Frame,
    pub generic: &'a (dyn Any + 'static),
    pub model_transform: Transform2D<f32, ModelSpace, CanvasSpace>,
    pub view_transform: Transform2D<f32, CanvasSpace, ScreenSpace>,
    pub dimensions: (u32, u32),
}

impl<'a> DrawingContext<'a> {
    pub fn with_model_transform<T>(
        &mut self,
        transform: Transform2D<f32, ModelSpace, CanvasSpace>,
        f: impl FnOnce(DrawingContext) -> T,
    ) -> T {
        let new = DrawingContext {
            generic: self.generic,
            model_transform: transform,
            view_transform: self.view_transform,
            programs: self.programs,
            target: self.target,
            dimensions: self.dimensions,
        };

        f(new)
    }

    pub fn with_generic<T>(&mut self, generic: &dyn Any, f: impl FnOnce(DrawingContext) -> T) -> T {
        let new = DrawingContext {
            generic,
            model_transform: self.model_transform,
            view_transform: self.view_transform,
            programs: self.programs,
            target: self.target,
            dimensions: self.dimensions,
        };

        f(new)
    }

    pub fn get_generic<T: 'static>(&self) -> Result<&'a T, CanvasError> {
        self.generic
            .downcast_ref()
            .ok_or(CanvasError::InvalidGenericType)
    }
}
