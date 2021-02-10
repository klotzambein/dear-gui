use std::{cell::RefCell, rc::Rc};

use glium::glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glium::Display;

use imgui::{Context, FontConfig, FontSource};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub trait EventHandler: Sized {
    fn handle_event(&mut self, event: &Event<()>) -> bool;
    fn chain<T>(self, other: T) -> EventHandlerChain<Self, T>
    where
        T: EventHandler,
    {
        EventHandlerChain(self, other)
    }
}

impl<T> EventHandler for &mut T
where
    T: EventHandler,
{
    fn handle_event(&mut self, event: &Event<()>) -> bool {
        (*self).handle_event(event)
    }
}

pub struct FnEventHandler<T: FnMut(&Event<()>) -> bool>(pub T);
impl<T> EventHandler for FnEventHandler<T>
where
    T: FnMut(&Event<()>) -> bool,
{
    fn handle_event(&mut self, event: &Event<()>) -> bool {
        self.0(event)
    }
}

pub struct EventHandlerChain<T1, T2>(T1, T2);

impl<T1: EventHandler, T2: EventHandler> EventHandler for EventHandlerChain<T1, T2> {
    fn handle_event(&mut self, event: &Event<()>) -> bool {
        self.0.handle_event(event) || self.1.handle_event(event)
    }
}

pub struct Imgui {
    pub context: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub display: Display,
}

impl Imgui {
    pub fn new(display: Display) -> Rc<RefCell<Imgui>> {
        let mut context = Context::create();
        context.set_ini_filename(None);

        let mut platform = WinitPlatform::init(&mut context);
        platform.attach_window(
            context.io_mut(),
            display.gl_window().window(),
            HiDpiMode::Rounded,
        );

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        context.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        }]);

        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let renderer =
            Renderer::init(&mut context, &display).expect("Failed to initialize renderer");

        let imgui = Imgui {
            context,
            platform,
            renderer,
            display,
        };
        Rc::new(RefCell::new(imgui))
    }
}

impl EventHandler for Imgui {
    fn handle_event(&mut self, event: &Event<()>) -> bool {
        let io = self.context.io_mut();
        let gl_win = self.display.gl_window();
        let window = gl_win.window();
        self.platform.handle_event(io, window, event);
        match event {
            Event::WindowEvent { window_id, event } if *window_id == window.id() => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => match (io.key_shift, io.key_ctrl, io.key_alt, key) {
                    (_, false, false, _)
                    | (_, _, _, VirtualKeyCode::LShift)
                    | (_, _, _, VirtualKeyCode::RShift)
                    | (_, _, _, VirtualKeyCode::LControl)
                    | (_, _, _, VirtualKeyCode::RControl)
                    | (_, _, _, VirtualKeyCode::LAlt)
                    | (_, _, _, VirtualKeyCode::RAlt)
                    | (false, true, false, VirtualKeyCode::Y)
                    | (false, true, false, VirtualKeyCode::Z)
                    | (false, true, false, VirtualKeyCode::C)
                    | (false, true, false, VirtualKeyCode::V)
                    | (false, true, false, VirtualKeyCode::X)
                    | (false, true, false, VirtualKeyCode::A)
                    | (_, true, false, VirtualKeyCode::Left)
                    | (_, true, false, VirtualKeyCode::Right) => io.want_capture_keyboard,
                    _ => false,
                },
                WindowEvent::ReceivedCharacter(_) => io.want_capture_keyboard,
                WindowEvent::CursorMoved { .. } => io.want_capture_mouse,
                WindowEvent::MouseWheel { .. } => io.want_capture_mouse,
                WindowEvent::MouseInput { .. } => io.want_capture_mouse,
                _ => false,
            },
            _ => false,
        }
    }
}
