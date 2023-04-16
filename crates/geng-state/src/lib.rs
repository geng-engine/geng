use geng_ui as ui;
use geng_window::Event;
use std::ops::DerefMut;

mod combined;
mod manager;

pub use combined::*;
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
    fn update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
    }

    /// Called periodically every `fixed_delta_time` defined in [ContextOptions].
    /// To start the application with different `fixed_delta_time`,
    /// initialize geng with [`Geng::new_with()`].
    fn fixed_update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);

    /// Called whenever an event is registered. See [Event] for a full list of possible events.
    fn handle_event(&mut self, event: Event) {
        #![allow(unused_variables)]
    }

    /// Called every frame. If returns `Some`, then a transition occurs.
    fn transition(&mut self) -> Option<Transition> {
        None
    }

    fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> Box<dyn ui::Widget + 'a> {
        #![allow(unused_variables)]
        Box::new(ui::Void)
    }
}

pub struct Empty;

impl State for Empty {
    fn draw(&mut self, _: &mut ugli::Framebuffer) {}
}

impl<T: State + ?Sized> State for Box<T> {
    fn update(&mut self, delta_time: f64) {
        <T as State>::update(self, delta_time);
    }
    fn fixed_update(&mut self, delta_time: f64) {
        <T as State>::fixed_update(self, delta_time);
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
    fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> Box<dyn ui::Widget + 'a> {
        <T as State>::ui(self, cx)
    }
}
