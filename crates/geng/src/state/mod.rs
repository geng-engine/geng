use super::*;

mod manager;

pub use manager::*;

/// Represents a transition between states.
pub enum Transition {
    /// Pops (removes) the current state from the state stack.
    Pop,
    /// Replaces the current state with another state.
    Switch(Box<dyn State>),
    /// Pushes a new state on the state stack.
    Push(Box<dyn State>),
}

/// Represents a state in the game.
pub trait State: 'static {
    /// Called every frame.
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}

    /// Called every frame.
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);

    /// Called whenever an event is registered. See [Event] for a full list of possible events.
    #[allow(unused_variables)]
    fn handle_event(&mut self, event: Event) {}

    /// Called every frame. If returns `Some`, then a transition occurs.
    fn transition(&mut self) -> Option<Transition> {
        None
    }
}

pub struct EmptyState;

impl State for EmptyState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        #![allow(unused_variables)]
    }
}

impl<T: State + ?Sized> State for Box<T> {
    fn update(&mut self, delta_time: f64) {
        <T as State>::update(self, delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        <T as State>::draw(self, framebuffer);
    }
    fn handle_event(&mut self, event: Event) {
        <T as State>::handle_event(self, event);
    }
    fn transition(&mut self) -> Option<Transition> {
        <T as State>::transition(self)
    }
}
