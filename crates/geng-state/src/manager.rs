use super::*;

pub struct Manager {
    stack: Vec<Box<dyn State>>,
}

impl Manager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn switch(&mut self, state: Box<dyn State>) {
        *self.stack.last_mut().unwrap() = state;
    }
    pub fn push(&mut self, state: Box<dyn State>) {
        self.stack.push(state);
    }
    pub fn pop(&mut self) {
        self.stack.pop();
    }
    pub fn current_state(&mut self) -> Option<&mut dyn State> {
        self.stack.last_mut().map(|state| state.deref_mut())
    }
}

impl State for Manager {
    fn update(&mut self, delta_time: f64) {
        if let Some(state) = self.current_state() {
            state.update(delta_time);
            if let Some(transition) = state.transition() {
                match transition {
                    Transition::Pop => self.pop(),
                    Transition::Push(state) => self.push(state),
                    Transition::Switch(state) => self.switch(state),
                }
            }
        }
    }
    fn fixed_update(&mut self, delta_time: f64) {
        if let Some(state) = self.current_state() {
            state.fixed_update(delta_time);
        }
    }
    fn handle_event(&mut self, event: Event) {
        if let Some(state) = self.current_state() {
            state.handle_event(event);
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        if let Some(state) = self.current_state() {
            state.draw(framebuffer);
        }
    }
    fn transition(&mut self) -> Option<Transition> {
        if self.stack.is_empty() {
            Some(Transition::Pop)
        } else {
            None
        }
    }
    fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> Box<dyn ui::Widget + 'a> {
        match self.current_state() {
            Some(state) => state.ui(cx),
            None => Box::new(ui::Void),
        }
    }
}
