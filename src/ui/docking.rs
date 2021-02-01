use std::any::Any;

use imgui::{im_str, Condition, StyleVar, Ui, Window};

use super::{UIComponent, UIComponentAny};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockingDirection {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct DockingLayout<T, TCb> {
    components: Vec<DockedUIComponent<T, TCb>>,
    main_menu_bar: bool,
}

struct DockedUIComponent<T, TCb> {
    component: Box<dyn UIComponentAny<TCb>>,
    model_fn: Box<dyn Fn(&T) -> &dyn Any>,
    direction: DockingDirection,
    size: f32,
}

impl<T, TCb> DockingLayout<T, TCb> {
    pub fn new(main_menu_bar: bool) -> DockingLayout<T, TCb> {
        DockingLayout {
            main_menu_bar,
            components: Vec::new(),
        }
    }

    pub fn add<C, F>(&mut self, component: C, model_fn: F, direction: DockingDirection, size: f32)
    where
        C: UIComponent<TCb> + 'static,
        C::Model: 'static,
        F: (Fn(&T) -> &C::Model) + 'static,
    {
        self.components.push(DockedUIComponent {
            component: Box::new(component),
            model_fn: Box::new(move |state| (model_fn)(state)),
            direction,
            size,
        })
    }
}

impl<T, TCb> UIComponent<TCb> for DockingLayout<T, TCb> {
    type Model = T;
    fn draw(&mut self, ui: &Ui, model: &T, cmd: &mut TCb) {
        let sv = ui.push_style_vars(&[
            StyleVar::WindowRounding(0.0),
            StyleVar::WindowMinSize([0., 0.]),
        ]);

        let (mut x, mut y) = (0., 0.);
        let [mut width, mut height] = ui.io().display_size;

        let style = ui.clone_style();

        if self.main_menu_bar {
            let offset = (style.display_safe_area_padding[1] - style.frame_padding[1]).max(0.)
                + ui.current_font().font_size
                + style.frame_padding[1]
                + 2.0;
            y += offset;
            height -= offset;
        }

        for (i, comp) in self.components.iter_mut().enumerate() {
            let (x, y, width, height) = match comp.direction {
                DockingDirection::Top => {
                    let y2 = y;
                    y += comp.size;
                    height -= comp.size;
                    (x, y2, width, comp.size)
                }
                DockingDirection::Bottom => {
                    height -= comp.size;
                    (x, height + y, width, comp.size)
                }
                DockingDirection::Left => {
                    let x2 = x;
                    x += comp.size;
                    width -= comp.size;
                    (x2, y, comp.size, height)
                }
                DockingDirection::Right => {
                    width -= comp.size;
                    (width + x, y, comp.size, height)
                }
            };

            Window::new(&im_str!("##DockedWindow_{}", i))
                .position([x, y], Condition::Always)
                .size([width, height], Condition::Always)
                .title_bar(false)
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .build(ui, || {
                    comp.component.draw_any(ui, (comp.model_fn)(model), cmd)
                });
        }
        sv.pop(ui);
    }
}
