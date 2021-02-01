use std::{cell::RefCell, rc::Rc, time::Instant};

use glium::Surface;
use imgui::{im_str, MenuItem, Ui, Window};

// use crate::command::{executor::CommandExecutor, value::Value, Command};
use crate::event_handling::Imgui;
// use crate::state::{CommandRequest, State, StateInner};

pub mod docking;

pub struct CommandExecutor;

pub trait UIComponent {
    type Model;
    fn draw(&mut self, ui: &Ui, model: &Self::Model, cmd: &mut CommandExecutor);
}

use std::any::Any;

pub trait UIComponentAny {
    fn draw_any(&mut self, ui: &Ui, model: &dyn Any, cmd: &mut CommandExecutor);
}

impl<T> UIComponentAny for T
where
    T: UIComponent,
    T::Model: 'static,
{
    fn draw_any(&mut self, ui: &Ui, model: &dyn Any, cmd: &mut CommandExecutor) {
        let model = model.downcast_ref::<T::Model>();
        self.draw(ui, model.unwrap(), cmd);
    }
}