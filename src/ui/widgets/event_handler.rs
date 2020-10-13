use super::*;

pub struct EventHandler<T, F> {
    child: T,
    handler: F,
}

impl<T: Container, F> Container for EventHandler<T, F> {
    type Leaf = T::Leaf;
    fn leaf(&self) -> &Self::Leaf {
        self.child.leaf()
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt: Widget + Sized {
        fn handle_events<F: FnMut(&Event)>(self, handler: F) -> EventHandler<Self, F> {
            EventHandler {
                child: self,
                handler,
            }
        }
        fn on_click<'a, F: FnMut() + 'a>(
            self,
            mut handler: F,
        ) -> EventHandler<Self, Box<dyn FnMut(&Event) + 'a>> {
            self.handle_events(Box::new(move |event| {
                if let Event::Click { .. } = event {
                    handler();
                }
            }))
        }
    }

    impl<T: Widget> WidgetExt for T {}
}

pub use ext::WidgetExt as _;

impl<T: Widget, F: FnMut(&Event)> Widget for EventHandler<T, F> {
    fn core(&self) -> &WidgetCore {
        self.child.core()
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        self.child.core_mut()
    }
    fn calc_constraints(&mut self) {
        self.child.calc_constraints();
    }
    fn layout_children(&mut self) {
        self.child.layout_children();
    }
    fn update(&mut self, delta_time: f64) {
        self.child.update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.child.draw(framebuffer);
    }
    fn handle_event(&mut self, event: &Event) {
        (self.handler)(event);
        self.child.handle_event(event);
    }
    fn walk_children_mut<'a>(&mut self, f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        self.child.walk_children_mut(f);
    }
}
