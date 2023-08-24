use futures::prelude::*;
use futures::{future::LocalBoxFuture, stream::LocalBoxStream, FutureExt};
use geng_window::Event;

pub trait ActiveState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);
}

impl<'a, T: ActiveState> ActiveState for &'a mut T {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        (**self).draw(framebuffer)
    }
}

pub type EventStream<'a> = LocalBoxStream<'a, Event>;

pub trait State<'a>: ActiveState {
    type Output;
    fn run(self) -> LocalBoxFuture<'a, StateResult<'a, Self::Output>>
    where
        Self: Sized;
}

pub trait Transition {
    fn finished(&self) -> bool;
    fn draw(
        &mut self,
        from: &mut dyn FnMut(&mut ugli::Framebuffer),
        to: &mut dyn FnMut(&mut ugli::Framebuffer),
        framebuffer: &mut ugli::Framebuffer,
    );
}

pub struct StateResult<'a, T> {
    pub value: T,
    pub active_state: Box<dyn ActiveState + 'a>,
}

pub struct Transitions<'a, To: State<'a> + 'a> {
    pub window: &'a geng_window::Window,
    pub outer: Box<dyn ActiveState + 'a>,
    pub inner: To,
    pub transition_enter: Box<dyn Transition + 'a>,
    pub transition_exit: Box<dyn FnOnce() -> Box<dyn Transition + 'a> + 'a>,
}

impl<'a, To: State<'a>> ActiveState for Transitions<'a, To> {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.outer.draw(framebuffer)
    }
}

impl<'a, To: State<'a>> State<'a> for Transitions<'a, To> {
    type Output = To::Output;

    fn run(mut self) -> LocalBoxFuture<'a, StateResult<'a, Self::Output>>
    where
        Self: Sized,
    {
        async move {
            let mut events = self.window.events();

            // enter transition
            while let Some(event) = events.next().await {
                match event {
                    Event::Draw => self.window.with_framebuffer(|framebuffer| {
                        self.transition_enter.draw(
                            &mut |f| self.outer.draw(f),
                            &mut |f| self.inner.draw(f),
                            framebuffer,
                        );
                    }),
                    _ => {}
                }
                if self.transition_enter.finished() {
                    break;
                }
            }

            let mut result = self.inner.run().await;

            // exit transition
            let mut transition_exit = (self.transition_exit)();
            while let Some(event) = events.next().await {
                match event {
                    Event::Draw => self.window.with_framebuffer(|framebuffer| {
                        transition_exit.draw(
                            &mut |f| result.active_state.draw(f),
                            &mut |f| self.outer.draw(f),
                            framebuffer,
                        );
                    }),
                    _ => {}
                }
                if transition_exit.finished() {
                    break;
                }
            }

            StateResult {
                value: result.value,
                active_state: self.outer,
            }
        }
        .boxed_local()
    }
}
