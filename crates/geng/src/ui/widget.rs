use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Constraints {
    pub min_size: Vec2<f64>,
    pub flex: Vec2<f64>,
}

pub struct WidgetCore {
    #[allow(dead_code)]
    pub(crate) id: ID,
    pub(crate) hovered: bool,
    pub(crate) captured: bool,
    pub(crate) position: AABB<f64>,
    pub(crate) constraints: Constraints,
}

impl WidgetCore {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: ID::new(),
            hovered: false,
            captured: false,
            position: AABB::from_corners(vec2(0.0, 0.0), vec2(1.0, 1.0)),
            constraints: Constraints {
                min_size: vec2(0.0, 0.0),
                flex: vec2(0.0, 0.0),
            },
        }
    }
    pub(crate) fn void() -> Self {
        Self {
            id: ID::void(),
            hovered: false,
            captured: false,
            position: AABB::from_corners(vec2(0.0, 0.0), vec2(1.0, 1.0)),
            constraints: Constraints {
                min_size: vec2(0.0, 0.0),
                flex: vec2(0.0, 0.0),
            },
        }
    }
    pub fn hovered(&self) -> bool {
        self.hovered
    }
    pub fn captured(&self) -> bool {
        self.captured
    }
    pub fn position(&self) -> AABB<f64> {
        self.position
    }
}

pub trait Widget {
    fn core(&self) -> &WidgetCore;
    fn core_mut(&mut self) -> &mut WidgetCore;
    fn calc_constraints(&mut self) {}
    fn layout_children(&mut self) {}
    fn update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        #![allow(unused_variables)]
    }
    fn handle_event(&mut self, event: &Event) {
        #![allow(unused_variables)]
    }
    fn walk_children_mut<'a>(&mut self, f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        #![allow(unused_variables)]
    }
}

impl Widget for WidgetCore {
    fn core(&self) -> &WidgetCore {
        self
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        self
    }
}

impl<T: Widget + ?Sized> Widget for &'_ mut T {
    fn core(&self) -> &WidgetCore {
        (**self).core()
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        (**self).core_mut()
    }
    fn calc_constraints(&mut self) {
        (**self).calc_constraints();
    }
    fn layout_children(&mut self) {
        (**self).layout_children();
    }
    fn update(&mut self, delta_time: f64) {
        (**self).update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        (**self).draw(framebuffer);
    }
    fn handle_event(&mut self, event: &Event) {
        (**self).handle_event(event);
    }
    fn walk_children_mut<'a>(&mut self, f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        (**self).walk_children_mut(f);
    }
}

impl<T: Widget + ?Sized> Widget for Box<T> {
    fn core(&self) -> &WidgetCore {
        (**self).core()
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        (**self).core_mut()
    }
    fn calc_constraints(&mut self) {
        (**self).calc_constraints();
    }
    fn layout_children(&mut self) {
        (**self).layout_children();
    }
    fn update(&mut self, delta_time: f64) {
        (**self).update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        (**self).draw(framebuffer);
    }
    fn handle_event(&mut self, event: &Event) {
        (**self).handle_event(event);
    }
    fn walk_children_mut<'a>(&mut self, f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        (**self).walk_children_mut(f);
    }
}
