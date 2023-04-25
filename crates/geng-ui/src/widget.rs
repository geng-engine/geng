use super::*;

#[derive(Default)]
pub struct Sense {
    pub clicked: bool,
    pub click_time: Option<f64>,
    pub hovered_time: Option<f64>,
    pub captured_time: Option<f64>,
}

impl Sense {
    pub fn is_hovered(&self) -> bool {
        self.hovered_time.is_some()
    }
    pub fn is_captured(&self) -> bool {
        self.captured_time.is_some()
    }
    pub fn set_hovered(&mut self, hovered: bool) {
        if hovered {
            if self.hovered_time.is_none() {
                self.hovered_time = Some(0.0);
            }
        } else {
            self.hovered_time = None;
        }
    }
    pub fn set_captured(&mut self, captured: bool) {
        if captured {
            if self.captured_time.is_none() {
                self.captured_time = Some(0.0);
            }
        } else {
            self.captured_time = None;
        }
    }
    pub fn click(&mut self) {
        self.click_time = Some(0.0);
        self.clicked = true;
    }
    pub fn take_clicked(&mut self) -> bool {
        std::mem::replace(&mut self.clicked, false)
    }
    pub fn update(&mut self, delta_time: f64) {
        if let Some(time) = &mut self.click_time {
            *time += delta_time;
        }
        if let Some(time) = &mut self.hovered_time {
            *time += delta_time;
        }
        if let Some(time) = &mut self.captured_time {
            *time += delta_time;
        }
    }
    pub fn handle_event(&mut self, _event: &Event) {
        // if let Event::Click { .. } = event {
        //     self.clicked = true;
        //     self.click_time = Some(0.0);
        // }
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Constraints {
    pub min_size: vec2<f64>,
    pub flex: vec2<f64>,
}

impl Constraints {
    pub fn zero() -> Self {
        Self {
            min_size: vec2::ZERO,
            flex: vec2::ZERO,
        }
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Self {
            min_size: vec2(0.0, 0.0),
            flex: vec2(1.0, 1.0),
        }
    }
}

pub struct DrawContext<'a, 'b> {
    pub draw2d: &'a draw2d::Helper,
    pub theme: &'a Theme,
    pub position: Aabb2<f64>,
    pub framebuffer: &'a mut ugli::Framebuffer<'b>,
}

pub trait Widget {
    fn sense(&mut self) -> Option<&mut Sense> {
        None
    }
    fn update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        #![allow(unused_variables)]
    }
    fn handle_event(&mut self, event: &Event) {
        #![allow(unused_variables)]
    }
    fn walk_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        #![allow(unused_variables)]
    }
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints;
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        self.walk_children_mut(&mut |child| cx.set_position(child, cx.position));
    }
}

impl<T: Widget> Widget for Box<T> {
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints {
        children.get_constraints(&**self)
    }
    fn walk_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut **self);
    }
}

impl Widget for Box<dyn Widget + '_> {
    fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
        cx.get_constraints(&**self)
    }
    fn walk_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut **self);
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt<'a>: Widget + Sized + 'a {
        fn boxed(self) -> Box<dyn Widget + 'a> {
            Box::new(self)
        }
    }

    impl<'a, T: Widget + 'a> WidgetExt<'a> for T {}
}

pub use ext::WidgetExt as _;
