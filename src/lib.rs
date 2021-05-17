use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use canvas::Canvas;
use euclid::Point2D;
use event_handling::{EventHandler, FnEventHandler, Imgui};
use geometry::CanvasSpace;
use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{Event, WindowEvent, MouseButton},
        event_loop::{ControlFlow, EventLoop},
    },
    Display, Frame,
};

use log::info;

pub mod canvas;
pub mod event_handling;
pub mod geometry;
pub mod graphics;
pub mod texture;
pub mod ui;

pub struct AppInit {
    pub display: Display,
    pub event_loop: Option<EventLoop<()>>,
    pub canvas: Canvas,
    pub imgui: Rc<RefCell<Imgui>>,
}

impl AppInit {
    pub fn new(title: impl Into<String>) -> AppInit {
        let event_loop = EventLoop::new();

        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(LogicalSize::new(756.0, 756.0))
            .with_title(title);

        let cb = glium::glutin::ContextBuilder::new().with_vsync(true);

        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        let canvas = Canvas::new(&display);
        let imgui = Imgui::new(display.clone());

        AppInit {
            display,
            event_loop: Some(event_loop),
            canvas,
            imgui,
        }
    }

    pub fn set_canvas_click_handler(&mut self, handler: Box<dyn FnMut(Point2D<f32, CanvasSpace>, MouseButton)>) {
        self.canvas.set_click_handler(handler);
    }

    pub fn run(mut self, mut draw: impl FnMut(&AppInit, &mut Frame, Instant) + 'static) {
        let event_loop = self.event_loop.take().expect("No event loop");

        let frame_duration = Duration::from_secs_f64(1. / 60.);
        let mut last_frame = Instant::now();
        event_loop.run(move |event, _target, control| {
            let canvas_input = self.canvas.input();

            // Handle close,
            FnEventHandler(|e| {
                use Event::WindowEvent as WE;
                use WindowEvent::CloseRequested as CR;
                if let WE { event: CR, .. } = e {
                    *control = ControlFlow::Exit;
                    info!("Closing");
                    true
                } else {
                    false
                }
            })
            // Handle captured mouse/keyboard (moving points, etc.)
            .chain(canvas_input.capture())
            // Handle GUI
            .chain(&mut *self.imgui.borrow_mut())
            // TODO: Handle shortcuts
            // Handle interactions with canvas
            .chain(canvas_input.usual())
            .handle_event(&event);

            let now = Instant::now();
            if let Event::MainEventsCleared = event {
                if now - last_frame < frame_duration {
                    *control = ControlFlow::WaitUntil(last_frame + frame_duration);
                } else {
                    self.display.gl_window().window().request_redraw();
                }
            } else if let Event::RedrawRequested(_) = event {
                let mut target = self.display.draw();

                draw(&mut self, &mut target, last_frame);

                target.finish().expect("Failed to swap buffers");

                last_frame = now;
            }
        });
    }
}
