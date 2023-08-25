use futures::prelude::*;
use futures::stream::LocalBoxStream;
use geng_window::Event;

pub trait Context {
    fn events(&self) -> LocalBoxStream<geng_window::Event>;
    fn with_framebuffer(&self, f: &mut dyn FnMut(&mut ugli::Framebuffer));
}

impl Context for geng_window::Window {
    fn events(&self) -> LocalBoxStream<geng_window::Event> {
        self.events().boxed_local()
    }

    fn with_framebuffer(&self, f: &mut dyn FnMut(&mut ugli::Framebuffer)) {
        self.with_framebuffer(f)
    }
}

pub trait ActiveState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);
}

impl<'a, T: ActiveState + ?Sized> ActiveState for &'a mut T {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        (**self).draw(framebuffer)
    }
}

impl<T: ActiveState + ?Sized> ActiveState for Box<T> {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        (**self).draw(framebuffer)
    }
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

pub async fn transition(
    ctx: &dyn Context,
    from: &mut dyn ActiveState,
    transition: &mut dyn Transition,
    into: &mut dyn ActiveState,
) {
    let mut events = ctx.events();
    while let Some(event) = events.next().await {
        if let Event::Draw = event {
            ctx.with_framebuffer(&mut |framebuffer| {
                transition.draw(&mut |f| from.draw(f), &mut |f| into.draw(f), framebuffer);
            });
        }
        if transition.finished() {
            break;
        }
    }
}
