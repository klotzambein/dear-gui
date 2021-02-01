use glium::{Display, glutin::{dpi::LogicalSize, event_loop::EventLoop}};


pub mod event_handling;
pub mod geometry;
pub mod graphics;
pub mod ui;
pub mod canvas;


pub fn init_window() -> (Display, EventLoop<()>) {
    let event_loop = EventLoop::new();

    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(756.0, 756.0))
        .with_title("ArchIntelligence");

    let cb = glium::glutin::ContextBuilder::new().with_vsync(true);

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    (display, event_loop)
}
