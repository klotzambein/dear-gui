use std::{cell::RefCell, rc::Rc, time::Instant};

use glium::Surface;
use imgui::{im_str, MenuItem, Ui, Window};

// use crate::command::{executor::T, value::Value, Command};
use crate::event_handling::Imgui;
// use crate::state::{CommandRequest, State, StateInner};

pub mod docking;

pub trait UIComponent<TCb> {
    type Model;
    fn draw(&mut self, ui: &Ui, model: &Self::Model, cmd: &mut TCb);
}

use std::any::Any;

pub trait UIComponentAny<TCb> {
    fn draw_any(&mut self, ui: &Ui, model: &dyn Any, cmd: &mut TCb);
}

impl<T, TCb> UIComponentAny<TCb> for T
where
    T: UIComponent<TCb>,
    T::Model: 'static,
{
    fn draw_any(&mut self, ui: &Ui, model: &dyn Any, cmd: &mut TCb) {
        let model = model.downcast_ref::<T::Model>();
        self.draw(ui, model.unwrap(), cmd);
    }
}