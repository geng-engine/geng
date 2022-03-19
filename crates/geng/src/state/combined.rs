use super::*;

pub struct CombinedState<A, B>(pub A, pub B);

impl<A: State, B: State> State for CombinedState<A, B> {
    fn update(&mut self, delta_time: f64) {
        self.0.update(delta_time);
        self.1.update(delta_time);
    }

    fn fixed_update(&mut self, delta_time: f64) {
        self.0.fixed_update(delta_time);
        self.1.fixed_update(delta_time);
    }

    fn handle_event(&mut self, event: Event) {
        self.0.handle_event(event.clone());
        self.1.handle_event(event);
    }

    fn transition(&mut self) -> Option<Transition> {
        if let Some(transition) = self.0.transition() {
            return Some(transition);
        }
        if let Some(transition) = self.1.transition() {
            return Some(transition);
        }
        None
    }

    fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> Box<dyn ui::Widget + 'a> {
        Box::new(ui::stack![self.0.ui(cx), self.1.ui(cx)])
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.0.draw(framebuffer);
        self.1.draw(framebuffer);
    }
}
