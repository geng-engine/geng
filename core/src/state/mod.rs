use crate::*;

mod manager;

pub use manager::*;

pub enum Transition {
    Pop,
    Switch(Box<dyn State>),
    Push(Box<dyn State>),
}

pub trait State: 'static {
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);

    #[allow(unused_variables)]
    fn handle_event(&mut self, event: Event) {}

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
